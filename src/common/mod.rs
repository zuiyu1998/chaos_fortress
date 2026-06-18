//! Common module.
//!
//! Defines shared types used across the game, such as rendering layers.

use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, PhysicsLayer, Sensor};
use bevy::prelude::*;
use bevy_lunex::{UiFetchFromCamera, UiLayoutRoot};
use bevy_tweening::{
    lens::TransformPositionLens,
    AnimCompletedEvent, Tween, TweenAnim,
};
use std::time::Duration;

pub(super) struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VisualDisplayLayer>();
        app.register_type::<GamePhysicsLayer>();
        app.register_type::<AttackRange>();
        app.register_type::<EnemyTarget>();
        app.register_type::<EnemyTargetList>();
        app.register_type::<UIRoot2d>();
        app.register_type::<DamageNumber>();
        app.register_type::<AutoDespawn>();

        app.add_systems(PostUpdate, despawn_on_tween_complete);
    }
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

/// All enemy entities currently within attack range.
///
/// Updated by collision events: enemies are added on `CollisionStarted`
/// and removed on `CollisionEnded`. Systems can iterate this list to
/// select a target or apply area-of-effect.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyTargetList(pub Vec<Entity>);


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

/// 2D UI root marker.
///
/// Marks an entity as the root of a 2D UI tree.
/// Used alongside bevy_lunex's `UiLayoutRoot` and `UiFetchFromCamera`
/// to bind the UI tree to a camera viewport.
/// Systems can query for `&UIRoot2d` to locate the UI root conveniently.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct UIRoot2d;

/// Damage number marker.
///
/// Marks an entity as a floating damage number, typically spawned as a
/// child of a bevy_lunex UI tree. Systems query for `&DamageNumber` to
/// drive float-up / fade-out animations and lifecycle management.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct DamageNumber;

/// Auto despawn marker.
///
/// Marks an entity for automatic despawning.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct AutoDespawn;

/// Despawn [`AutoDespawn`] entities when their tween animation completes.
///
/// Listens for [`AnimCompletedEvent`] fired by bevy_tweening when a
/// [`TweenAnim`] finishes on an entity marked with [`AutoDespawn`].
fn despawn_on_tween_complete(
    mut commands: Commands,
    mut events: MessageReader<AnimCompletedEvent>,
    query: Query<&AutoDespawn>,
) {
    for event in events.read() {
        if query.contains(event.anim_entity) {
            commands.entity(event.anim_entity).despawn();
        }
    }
}

/// Spawn a damage number.
///
/// Returns a bundle containing a [`DamageNumber`] marker, a [`Text2d`]
/// with the given font, and a [`Transform`] positioned at the given
/// world coordinates with a floating-up animation.
pub fn damage_number(value: i32, pos_x: f32, pos_y: f32, font: Handle<Font>) -> impl Bundle {
    let start = Vec3::new(pos_x, pos_y, 10.0);
    let end = Vec3::new(pos_x, pos_y + 80.0, 10.0);
    let duration = Duration::from_secs(1);

    (
        Name::new(format!("DamageNumber ({value})")),
        (DamageNumber, AutoDespawn),
        Text2d::new(format!("{value}")),
        TextFont {
            font,
            font_size: 64.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.2, 0.2)),
        Transform::from_translation(start),
        Visibility::default(),
        // bevy_tweening: float upward animation
        TweenAnim::new(Tween::new(
            EaseFunction::QuadraticOut,
            duration,
            TransformPositionLens { start, end },
        )),
    )
}

/// Spawn a 2D UI root node.
///
/// Returns a bundle containing [`UIRoot2d`], [`UiLayoutRoot`], and
/// [`UiFetchFromCamera`] components, linked to camera index 0.
/// This is the entry point for all 2D UI spawned via bevy_lunex.
pub fn ui_root_2d() -> impl Bundle {
    (
        Name::new("UI Root"),
        UIRoot2d,
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
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
