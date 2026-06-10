//! Role module.
//!
//! Defines the [`Role`] component, which marks an entity as a controllable
//! character (player side).

use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::common::{GamePhysicsLayer, VisualDisplayLayer};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Role>();
}

/// A component that marks an entity as a "role" (character).
///
/// The entity with this component represents a controllable character unit
/// that occupies one map cell (64×64 px). Roles carry additional components
/// for their attributes (health, attack, defense, etc.) and are driven by
/// the role system's ECS systems.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Role;

/// Spawn a role sprite at a given grid position.
///
/// Returns a bundle containing a [`Role`] marker, a [`Sprite`], and a
/// [`Transform`] positioned at the center of the given grid cell.
/// The grid coordinate system matches [`map::map_cell`]: cell (0,0) is
/// the top-left of the grid, and the grid is centered on its parent.
pub fn role(cell_size: f32, column: u32, row: u32, sprite: Sprite) -> impl Bundle {
    let x = column as f32 * cell_size;
    let y = -(row as f32 * cell_size);
    (
        Name::new(format!("Role ({column}, {row})")),
        Role,
        sprite,
        Transform::from_xyz(x, y, VisualDisplayLayer::Character.z_value()),
        Visibility::default(),
        RigidBody::Kinematic,
        Collider::circle(cell_size / 2.0),
        GamePhysicsLayer::character_layers(),
    )
}
