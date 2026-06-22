//! Archer role submodule.
//!
//! Defines the [`ArcherRoleBuilder`] which implements [`super::RoleBuilder`],
//! along with attribute components for ranged combat.

use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use bevy_gearbox::prelude::*;

use crate::battle::battle;
use crate::bullet::{BulletPosition, BulletPositionTarget};
use crate::common::{
    AttackRange, EnemyTarget, EnemyTargetList, GamePhysicsLayer, VisualDisplayLayer,
    attack_range,
};
use crate::state::{Pause, Screen};

use super::{Archer, BuildError, Role, RoleBuilder, RoleBuilderContainer, RoleBuilderContext};
use crate::skill::{skill, SkillActive, SkillTarget};

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

/// Marker component inserted on the archer entity while in Combat state.
///
/// Uses gearbox's [`StateComponent`] mechanism: when the state machine
/// enters the `Combat` substate, `ArcherCombat` is automatically inserted on
/// the state machine root (archer) entity; when leaving Combat, it is
/// automatically removed. Query `(With<Archer>, With<ArcherCombat>)` to
/// find archers currently in Combat state.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ArcherCombat;

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
        app.register_type::<ArcherIdle>();
        app.register_state_component::<ArcherIdle>();
        app.register_type::<ArcherCombat>();
        app.register_state_component::<ArcherCombat>();

        let mut container = app.world_mut().resource_mut::<RoleBuilderContainer>();
        container.register(
            "archer",
            ArcherRoleBuilder,
        );

        app.register_transition::<Idle2Combat>();

        app.add_systems(
            Update,
            (
                detect_target_when_idle,
                add_skill_active_when_combat,
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
/// [`Archer`] marker components. All combat attributes are sourced from
/// [`RoleBuilderContext::attributes`]; no hardcoded defaults remain.
pub struct ArcherRoleBuilder;

/// Attach a state machine to an archer entity with Idle and Combat states.
///
/// The state machine starts in `Idle`. Idle → Combat transitions are
/// triggered by external systems sending [`Idle2Combat`] messages.
pub fn setup_state_machine(machine: Entity, commands: &mut Commands) {
    let idle = commands
        .spawn_substate(machine, (Name::new("Idle"), StateComponent(ArcherIdle)))
        .id();
    let combat = commands
        .spawn_substate(machine, (Name::new("Combat"), StateComponent(ArcherCombat)))
        .id();

    // Message-driven transition: Idle → Combat (triggered by external systems)
    commands.spawn_transition::<Idle2Combat>(idle, combat);

    // Initialize the state machine, starting in Idle
    commands.entity(machine).init_state_machine(idle);
}

impl RoleBuilder for ArcherRoleBuilder {
    fn build<'w, 's, 'a>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: RoleBuilderContext<'a>,
    ) -> Result<Entity, BuildError> {
        let (col, row) = ctx.position;
        let cell_size = 64.0;
        let attack_range_val = ctx.attributes
            .get("attack_range")
            .map(|a| a.value)
            .ok_or_else(|| BuildError::BuildFailed("missing attribute 'attack_range'".into()))?;
        let attack_speed_val = ctx.attributes
            .get("attack_speed")
            .map(|a| a.value)
            .ok_or_else(|| BuildError::BuildFailed("missing attribute 'attack_speed'".into()))?;
        let projectile_damage_val = ctx.attributes
            .get("attack")
            .map(|a| a.value)
            .ok_or_else(|| BuildError::BuildFailed("missing attribute 'attack'".into()))?;

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
            AttackRange(attack_range_val),
            AttackSpeed(attack_speed_val),
            ProjectileDamage(projectile_damage_val),
            EnemyTarget(None),
            EnemyTargetList(Vec::new()),
        ));
        entity.insert(battle(ctx.attributes));

        let mut bullet_position_entity = Entity::PLACEHOLDER;
        let mut skill_entity = Entity::PLACEHOLDER;
        entity.with_children(|parent| {
            parent.spawn((
                attack_range(attack_range_val, GamePhysicsLayer::detect_enemy_layers()),
                Transform::default(),
            ));
            bullet_position_entity = parent
                .spawn((
                    Name::new("BulletPosition"),
                    BulletPosition,
                    Transform::default(),
                ))
                .id();
            skill_entity = skill(parent, ctx.skill_container, ctx.skill_effect_container, ctx.skill, ctx.skill_handle.clone());
        });
        entity.insert(BulletPositionTarget(bullet_position_entity));
        entity.insert(SkillTarget(skill_entity));

        if let Some(parent) = ctx.parent {
            entity.set_parent_in_place(parent);
        }

        let id = entity.id();
        setup_state_machine(id, commands);
        Ok(id)
    }
}

/// When the Combat substate becomes active (Idle→Combat transition completes),
/// add [`SkillActive`] to the archer's skill entity.
///
/// Uses `Added<Active>` so this only fires once per Combat activation, not every
/// frame. Combined with `With<StateComponent<ArcherCombat>>` to target the Combat
/// substate. Reads `Active.machine` to get the archer root, then uses its
/// [`SkillTarget`] to find the skill child.
pub fn add_skill_active_when_combat(
    combat_states: Query<&Active, (With<StateComponent<ArcherCombat>>, Added<Active>)>,
    archers: Query<&SkillTarget, With<Archer>>,
    mut commands: Commands,
) {
    for active in &combat_states {
        if let Ok(skill_target) = archers.get(active.machine) {
            commands.entity(skill_target.0).insert(SkillActive);
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
                writer.write(Idle2Combat {
                    machine: active.machine,
                });
            }
        }
    }
}
