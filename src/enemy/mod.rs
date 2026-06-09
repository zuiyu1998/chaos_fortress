//! Enemy module.
//!
//! Defines the [`Enemy`] component, which marks an entity as an enemy unit.

use bevy::prelude::*;

use crate::common::VisualDisplayLayer;
use crate::map::MapData;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Enemy>();
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

/// Spawn an enemy sprite at a given grid position.
///
/// Returns a bundle containing an [`Enemy`] marker, a [`Sprite`], and a
/// [`Transform`] positioned at the center of the given grid cell.
/// The grid coordinate system matches [`map::map_cell`]: cell (0,0) is
/// the top-left of the grid, and the grid is centered on its parent.
pub fn enemy(map_data: &MapData, column: u32, row: u32, sprite: Sprite) -> impl Bundle {
    let cell_size = map_data.cell_size;
    let x = (column as f32 - (map_data.width as f32 - 1.0) / 2.0) * cell_size;
    let y = -((row as f32 - (map_data.height as f32 - 1.0) / 2.0) * cell_size);
    (
        Name::new(format!("Enemy ({column}, {row})")),
        Enemy,
        sprite,
        Transform::from_xyz(x, y, VisualDisplayLayer::Character.z_value()),
        Visibility::default(),
    )
}
