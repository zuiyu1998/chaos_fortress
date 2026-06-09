//! Map module.
//!
//! Defines the [`Map`] component, which marks an entity as the game map.

use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;

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

/// Spawn the map entity along with grid cells.
///
/// Spawns a [`Map`] entity under the given `parent` builder, then adds
/// a grid of cell children using [`map_cell`]. The grid dimensions and
/// cell size come from [`MapData`].
pub fn map(parent: &mut ChildSpawnerCommands, map_data: &MapData) {
    parent
        .spawn((
            Name::new("Map"),
            Map,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|cell_parent| {
            for row in 0..map_data.height {
                for col in 0..map_data.width {
                    cell_parent.spawn(map_cell(
                        map_data,
                        col,
                        row,
                        Sprite::from_color(
                            Color::WHITE,
                            Vec2::splat(map_data.cell_size),
                        ),
                    ));
                }
            }
        });
}

/// Spawn a cell sprite at a given grid position.
///
/// Returns a bundle containing a [`Sprite`] and [`Transform`] positioned
/// at the cell location relative to the map's origin. The grid is centered
/// on the map entity (cell (0,0) is top-left).
pub fn map_cell(map_data: &MapData, column: u32, row: u32, sprite: Sprite) -> impl Bundle {
    let cell_size = map_data.cell_size;
    // Center the grid on the map's origin
    let x = (column as f32 - (map_data.width as f32 - 1.0) / 2.0) * cell_size;
    let y = -((row as f32 - (map_data.height as f32 - 1.0) / 2.0) * cell_size);
    (
        Name::new(format!("MapCell ({column}, {row})")),
        sprite,
        Transform::from_xyz(x, y, 0.0),
        Visibility::default(),
    )
}
