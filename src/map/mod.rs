//! Map module.
//!
//! Defines the [`Map`] component, which marks an entity as the game map.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Map>();
}

/// A component that marks an entity as the "map".
///
/// The entity with this component represents the current battlefield grid
/// (10 rows × 8 columns, each cell is 64×64 px).
/// Only one entity in the scene should carry this component.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Map;
