//! Common module.
//!
//! Defines shared types used across the game, such as rendering layers.

use avian2d::prelude::{CollisionLayers, PhysicsLayer};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<VisualDisplayLayer>();
    app.register_type::<GamePhysicsLayer>();
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

/// A physics collision layer for determining which entities interact.
///
/// Each variant corresponds to a distinct bit in the collision mask.
/// The [`PhysicsLayer`] derive macro assigns the bits automatically.
#[derive(PhysicsLayer, Clone, Copy, Debug, Default, Reflect, PartialEq, Eq)]
pub enum GamePhysicsLayer {
    /// The static world layer — terrain, walls, obstacles.
    #[default]
    World,
    /// Player character layer.
    Character,
    /// Enemy layer.
    Enemy,
}

impl GamePhysicsLayer {
    /// Returns [`CollisionLayers`] for a static world entity.
    ///
    /// World entities belong to the `World` layer and interact with
    /// everything (Character and Enemy).
    pub fn world_layers() -> CollisionLayers {
        CollisionLayers::new(Self::World, [Self::Character, Self::Enemy])
    }

    /// Returns [`CollisionLayers`] for a player character entity.
    ///
    /// Character entities belong to the `Character` layer and interact
    /// with the world and enemies.
    pub fn character_layers() -> CollisionLayers {
        CollisionLayers::new(Self::Character, [Self::World, Self::Enemy])
    }

    /// Returns [`CollisionLayers`] for an enemy entity.
    ///
    /// Enemy entities belong to the `Enemy` layer and interact with
    /// the world and player characters.
    pub fn enemy_layers() -> CollisionLayers {
        CollisionLayers::new(Self::Enemy, [Self::World, Self::Character])
    }
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
