//! Archer role submodule.
//!
//! Defines the [`ArcherRoleBuilder`] which implements [`super::RoleBuilder`],
//! along with attribute components for ranged combat.

use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::bullet::{bullet, BulletPosition, BulletPositionTarget};
use crate::common::{attack_range, AttackRange, CoolingTimer, GamePhysicsLayer, VisualDisplayLayer};
use crate::{Pause, screens::Screen};

use super::{Archer, Role, RoleBuilder, RoleBuilderContext, RoleBuilderContainer};

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

        app.add_systems(
            Update,
            run_skill.run_if(in_state(Screen::Gameplay).and(in_state(Pause(false)))),
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

        entity.id()
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
