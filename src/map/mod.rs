//! Map module.
//!
//! Defines the [`Map`] component, which marks an entity as the game map.

use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use crate::common::VisualDisplayLayer;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Map>();
    app.register_type::<MapCell>();
    app.register_type::<MapCellData>();
    app.register_type::<BenchCell>();
    app.init_resource::<MapData>();
}

/// A component storing the grid coordinates of a map cell entity.
///
/// Every cell child under the [`Map`] entity carries this component.
/// `x` is the column (0~11), `y` is the row (0~4).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct MapCell {
    /// Column (0~11), increasing to the right.
    pub x: u32,
    /// Row (0~4), increasing downward.
    pub y: u32,
}

/// A component storing the optional role entity standing on a map cell.
///
/// `role` is `Some(entity)` when a role occupies this cell, `None` when free.
/// Added automatically during cell initialization with a default of `None`.
#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct MapCellData {
    /// The role entity on this cell, if any.
    pub role: Option<Entity>,
}

/// A marker component indicating that a map cell belongs to the bench zone (备战区).
///
/// Bench-zone cells are columns 0~1 (the two leftmost columns).
/// They serve as the deployment area for the player's roles.
/// Cells in columns 2~11 (the combat zone) do NOT carry this component.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct BenchCell;

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
            width: 12,
            height: 5,
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
                    let mut entity = cell_parent.spawn(map_cell(
                        map_data.cell_size,
                        col,
                        row,
                        Sprite::from_color(
                            Color::WHITE,
                            Vec2::splat(map_data.cell_size),
                        ),
                    ));
                    if col <= 1 {
                        entity.insert(BenchCell);
                    }
                }
            }
        });
}

/// Spawn a cell sprite at a given grid position.
///
/// Returns a bundle containing a [`Sprite`] and [`Transform`] positioned
/// at the cell location relative to the map's origin (cell (0,0) is top-left).
pub fn map_cell(cell_size: f32, column: u32, row: u32, sprite: Sprite) -> impl Bundle {
    let x = column as f32 * cell_size;
    let y = -(row as f32 * cell_size);
    (
        Name::new(format!("MapCell ({column}, {row})")),
        MapCell { x: column, y: row },
        MapCellData { role: None },
        sprite,
        Transform::from_xyz(x, y, VisualDisplayLayer::Terrain.z_value()),
        Visibility::default(),
    )
}
