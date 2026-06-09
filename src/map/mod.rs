//! Map module.
//!
//! Defines the [`Map`] component, which marks an entity as the game map.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Map>();
    app.init_resource::<MapData>();
}

/// A component that marks an entity as the "map".
///
/// The entity with this component represents the current battlefield grid
/// (10 rows × 8 columns, each cell is 64×64 px).
/// Only one entity in the scene should carry this component.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Map;

/// A resource that stores map configuration data.
///
/// Used when constructing child entities under the [`Map`] entity,
/// such as terrain tiles and grid lines.
#[derive(Resource, Debug, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct MapData {
    /// Number of horizontal grid cells (width / columns).
    pub width: u32,
    /// Number of vertical grid cells (height / rows).
    pub height: u32,
    /// Pixel size of each grid cell (square side length).
    pub cell_size: f32,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            width: 8,
            height: 10,
            cell_size: 64.0,
        }
    }
}
