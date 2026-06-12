//! Enemy module.
//!
//! Defines the [`Enemy`] component, which marks an entity as an enemy unit.

use avian2d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;

use crate::battle::battle;
use crate::common::{GamePhysicsLayer, VisualDisplayLayer};

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>();
    }
}

/// A component that marks an entity as an "enemy".
///
/// The entity with this component represents an enemy unit that occupies
/// one map cell (64×64 px). Enemies carry additional components for their
/// attributes, AI mode, and drop data, and are driven by the enemy system's
/// ECS systems.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

/// Spawn an enemy sprite at a given grid position with combat attributes.
///
/// Returns a bundle containing an [`Enemy`] marker, a [`Sprite`], a
/// [`Transform`] positioned at the center of the given grid cell,
/// physics components ([`RigidBody::Dynamic`], [`Collider`], [`CollisionLayers`],
/// [`LinearVelocity`]), and a [`BattleState`] via [`battle`].
/// The grid coordinate system matches [`map::map_cell`]: cell (0,0) is
/// the top-left of the grid, and the grid is centered on its parent.
pub fn enemy(
    cell_size: f32,
    column: u32,
    row: u32,
    sprite: Sprite,
    max_hp: f32,
    armor: f32,
) -> impl Bundle {
    let x = column as f32 * cell_size;
    let y = -(row as f32 * cell_size);
    (
        Name::new(format!("Enemy ({column}, {row})")),
        Enemy,
        sprite,
        Transform::from_xyz(x, y, VisualDisplayLayer::Character.z_value()),
        Visibility::default(),
        RigidBody::Dynamic,
        Collider::circle(cell_size / 2.0),
        GamePhysicsLayer::enemy_layers(),
        LinearVelocity(Vec2::new(0.0, -10.0)),
        battle(max_hp, armor),
    )
}
