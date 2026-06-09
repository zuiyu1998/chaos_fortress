//! Common module.
//!
//! Defines shared types used across the game, such as rendering layers.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<VisualDisplayLayer>();
}

/// A z-order layer for visual elements.
///
/// Controls the rendering order (z-axis) of entities by providing a
/// corresponding `f32` value for each layer. Entities with a higher value
/// are rendered on top.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum VisualDisplayLayer {
    /// Terrain layer — map cells, terrain tiles, etc. (z = 0.0)
    #[default]
    Terrain,
    /// Character layer — roles, enemies, interactive units (z = 1.0)
    Character,
}

impl VisualDisplayLayer {
    /// Returns the z-axis value for this layer.
    ///
    /// Higher values are rendered on top.
    pub fn z_value(&self) -> f32 {
        match self {
            VisualDisplayLayer::Terrain => 0.0,
            VisualDisplayLayer::Character => 1.0,
        }
    }
}
