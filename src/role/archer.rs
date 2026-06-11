//! Archer role submodule.
//!
//! Defines the [`ArcherRoleBuilder`] which implements [`super::RoleBuilder`],
//! along with attribute components for ranged combat.

use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::bullet::BulletPosition;
use crate::common::{attack_range, AttackRange, GamePhysicsLayer, VisualDisplayLayer};

use super::{Archer, Role, RoleBuilder, RoleBuilderContext};

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
        ));

        entity.with_children(|parent| {
            parent.spawn((
                attack_range(self.attack_range, GamePhysicsLayer::detect_enemy_layers()),
                Transform::default(),
            ));
            parent.spawn((
                BulletPosition,
                Transform::default(),
            ));
        });

        if let Some(parent) = ctx.parent {
            entity.set_parent_in_place(parent);
        }

        entity.id()
    }
}
