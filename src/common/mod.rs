//! Common module.
//!
//! Defines shared types used across the game, such as rendering layers.

use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, PhysicsLayer, Sensor};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<VisualDisplayLayer>();
    app.register_type::<GamePhysicsLayer>();
    app.register_type::<AttackRange>();
    app.register_type::<CoolingTimer>();
    app.register_type::<EnemyTarget>();
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
    /// Bullet layer — projectiles, arrows, magic bolts (z = 2.0)
    Bullet,
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

    /// Returns [`CollisionLayers`] that only detects enemy entities.
    ///
    /// The entity belongs to the `Character` layer but only filters
    /// for `Enemy`. Useful for sensors like attack range that should
    /// only interact with enemies.
    pub fn detect_enemy_layers() -> CollisionLayers {
        CollisionLayers::new(Self::Character, [Self::Enemy])
    }
}

/// Attack range in pixels.
///
/// Stores the maximum distance (in world pixels) at which this entity
/// can attack a target.
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct AttackRange(pub f32);

/// Current enemy target, may be empty.
///
/// Stores the entity of the enemy that this entity has currently locked
/// as a target. `None` means no target is acquired.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyTarget(pub Option<Entity>);

/// Cooldown timer.
///
/// Stores a Bevy [`Timer`] that counts down cooldown duration
/// (e.g. between attacks). Use [`TimerMode::Once`] for single-shot
/// cooldowns.
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CoolingTimer(pub Timer);

/// Tick all [`CoolingTimer`] components each frame.
///
/// Advances every entity's cooldown timer by [`Time::delta`].
pub fn tick_all(time: Res<Time>, mut query: Query<&mut CoolingTimer>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}

/// Spawn an attack-range sensor.
///
/// Returns a bundle containing an [`AttackRange`] component, a
/// [`Collider::circle`] sensor, and a [`Sensor`] marker.
/// The caller is responsible for positioning this entity via a
/// separate [`Transform`] component.
///
/// This sensor can be used to detect enemy entities entering the
/// attack range via collision events.
pub fn attack_range(range: f32, layers: CollisionLayers) -> impl Bundle {
    (
        Name::new(format!("AttackRange ({range:.1})")),
        AttackRange(range),
        CollisionEventsEnabled,
        Visibility::default(),
        Collider::circle(range),
        Sensor,
        layers,
    )
}

impl VisualDisplayLayer {
    /// Returns the z-axis value for this layer.
    ///
    /// Higher values are rendered on top.
    pub fn z_value(&self) -> f32 {
        match self {
            VisualDisplayLayer::Terrain => 0.0,
            VisualDisplayLayer::Character => 1.0,
            VisualDisplayLayer::Bullet => 2.0,
        }
    }
}
