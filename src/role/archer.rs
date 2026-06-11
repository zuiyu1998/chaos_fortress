//! Archer role submodule.
//!
//! Defines the [`ArcherRoleBuilder`] which implements [`super::RoleBuilder`],
//! along with attribute components for ranged combat.

use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use bevy_gearbox::prelude::*;

use crate::bullet::{bullet, BulletPosition, BulletPositionTarget};
use crate::common::{attack_range, AttackRange, CoolingTimer, EnemyTarget, GamePhysicsLayer, VisualDisplayLayer};
use crate::{Pause, screens::Screen};

use super::{Archer, Role, RoleBuilder, RoleBuilderContext, RoleBuilderContainer};

/// Marker component inserted on the archer entity while in Idle state.
///
/// Uses gearbox's [`StateComponent`] mechanism: when the state machine
/// enters the `Idle` substate, `ArcherIdle` is automatically inserted on
/// the state machine root (archer) entity; when leaving Idle, it is
/// automatically removed. Query `(With<Archer>, With<ArcherIdle>)` to
/// find archers currently in Idle state.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ArcherIdle;

/// Message for transitioning an archer from Idle to Combat state.
///
/// External systems write this message to trigger the Idle → Combat
/// transition on a state machine. The `target` field identifies which
/// state machine entity the message is addressed to.
#[derive(Message, Clone, TypePath)]
pub struct Idle2Combat {
    /// The target state machine entity.
    pub machine: Entity,
}

impl GearboxMessage for Idle2Combat {
    type Validator = AcceptAll;

    fn target(&self) -> Entity {
        self.machine
    }
}

/// Plugin for registering archer-related components.
///
/// Registers [`Archer`], [`AttackSpeed`], and [`ProjectileDamage`] with
/// Bevy's reflection system.
pub struct ArcherPlugin;

impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Archer>();
        app.register_type::<AttackSpeed>();
        app.register_type::<ProjectileDamage>();
        app.register_type::<CoolingTimer>();
        app.register_type::<ArcherIdle>();
        app.register_state_component::<ArcherIdle>();

        let mut container = app.world_mut().resource_mut::<RoleBuilderContainer>();
        container.register(
            "archer",
            ArcherRoleBuilder {
                name: "Archer".into(),
                attack_range: 300.0,
                attack_speed: 0.8,
                projectile_damage: 15.0,
            },
        );

        app.register_transition::<Idle2Combat>();

        app.add_systems(
            Update,
            (
                run_skill,
                detect_target_when_idle,
            )
                .run_if(in_state(Screen::Gameplay).and(in_state(Pause(false)))),
        );
    }
}

/// Attack interval in seconds.
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct AttackSpeed(pub f32);

/// Projectile damage value.
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct ProjectileDamage(pub f32);

/// Builder for archer role entities.
///
/// Implements [`RoleBuilder`] to spawn an entity with both [`Role`] and
/// [`Archer`] marker components, alongside ranged combat attributes.
pub struct ArcherRoleBuilder {
    /// The archer's display name.
    pub name: String,
    /// Attack range in pixels.
    pub attack_range: f32,
    /// Attack interval in seconds.
    pub attack_speed: f32,
    /// Projectile damage value.
    pub projectile_damage: f32,
}

/// Attach a state machine to an archer entity with Idle and Combat states.
///
/// The state machine starts in `Idle`. A Combat → Idle automatic transition
/// is set up via `AlwaysEdge`; Idle → Combat transitions are triggered by
/// external systems sending gearbox messages.
pub fn setup_state_machine(machine: Entity, commands: &mut Commands) {
    let idle = commands
        .spawn_substate(machine, (Name::new("Idle"), StateComponent(ArcherIdle)))
        .id();
    let combat = commands
        .spawn_substate(machine, Name::new("Combat"))
        .id();

    // Message-driven transition: Idle → Combat (triggered by external systems)
    commands.spawn_transition::<Idle2Combat>(idle, combat);
    // Automatic transition: Combat → Idle (back to rest when conditions clear)
    commands.spawn_transition_always(combat, idle);

    // Initialize the state machine, starting in Idle
    commands.entity(machine).init_state_machine(idle);
}

impl RoleBuilder for ArcherRoleBuilder {
    fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: RoleBuilderContext) -> Entity {
        let (col, row) = ctx.position;
        let cell_size = 64.0;
        let mut entity = commands.spawn((
            Name::new(format!("Archer ({col},{row})")),
            Role,
            Archer,
            Sprite::from_color(Color::srgb(0.0, 0.8, 0.2), Vec2::splat(cell_size)),
            Transform::from_xyz(
                col as f32 * cell_size,
                -(row as f32 * cell_size),
                VisualDisplayLayer::Character.z_value(),
            ),
            Visibility::default(),
            RigidBody::Kinematic,
            Collider::circle(cell_size / 2.0),
            GamePhysicsLayer::character_layers(),
            AttackRange(self.attack_range),
            AttackSpeed(self.attack_speed),
            ProjectileDamage(self.projectile_damage),
            CoolingTimer(Timer::from_seconds(1.0, TimerMode::Once)),
            EnemyTarget(None),
        ));

        let mut bullet_position_entity = Entity::PLACEHOLDER;
        entity.with_children(|parent| {
            parent.spawn((
                attack_range(self.attack_range, GamePhysicsLayer::detect_enemy_layers()),
                Transform::default(),
            ));
            bullet_position_entity = parent.spawn((
                Name::new("BulletPosition"),
                BulletPosition,
                Transform::default(),
            )).id();
        });
        entity.insert(BulletPositionTarget(bullet_position_entity));

        if let Some(parent) = ctx.parent {
            entity.set_parent_in_place(parent);
        }

        let id = entity.id();
        setup_state_machine(id, commands);
        id
    }
}

/// System that drives archers to fire bullets automatically.
///
/// Queries all entities with [`Archer`] and [`CoolingTimer`]. When the timer
/// has just finished, it resets the cooldown, reads the [`BulletPosition`]
/// child entity's world position, and spawns a bullet flying upward.
pub fn run_skill(
    mut commands: Commands,
    mut query: Query<(&mut CoolingTimer, &BulletPositionTarget), With<Archer>>,
    bullet_position_query: Query<&GlobalTransform, With<BulletPosition>>,
) {
    for (mut timer, target) in &mut query {
        if timer.0.just_finished() {
            timer.0.reset();

            if let Ok(transform) = bullet_position_query.get(target.0) {
                let position = transform.translation().truncate();
                commands.spawn(bullet(position, Vec2::new(0.0, 200.0)));
            }
        }
    }
}

/// When an archer is in Idle state and has acquired an enemy target,
/// send an [`Idle2Combat`] message to transition the state machine to Combat.
///
/// Queries active Idle substates via `StateComponent<ArcherIdle>` + [`Active`],
/// reads `Active.machine` to get the archer entity, then checks its
/// [`EnemyTarget`].
pub fn detect_target_when_idle(
    idle_states: Query<&Active, (With<StateComponent<ArcherIdle>>, With<Active>)>,
    archers: Query<&EnemyTarget, With<Archer>>,
    mut writer: MessageWriter<Idle2Combat>,
) {
    for active in &idle_states {
        if let Ok(target) = archers.get(active.machine) {
            if target.0.is_some() {
                writer.write(Idle2Combat { machine: active.machine });
            }
        }
    }
}
