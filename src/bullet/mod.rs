//! Bullet module.
//!
//! Defines the [`Bullet`] component, which marks an entity as a projectile
//! (bullet) that can collide with enemies and deal damage.

use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, CollisionStart, LinearVelocity, RigidBody, Sensor};
use bevy::prelude::*;

use crate::common::VisualDisplayLayer;

/// Plugin that registers bullet-related components, messages, and systems.
///
/// Registers [`Bullet`], [`BulletPosition`], [`BulletPositionTarget`],
/// and [`BulletBattleEvent`] with Bevy's type registry and message system,
/// and adds the [`emit_bullet_battle_event`] and [`despawn_on_hit`]
/// systems.
pub(super) struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>();
        app.register_type::<BulletPosition>();
        app.register_type::<BulletPositionTarget>();
        app.add_message::<BulletBattleEvent>();

        app.add_systems(Update, (emit_bullet_battle_event, despawn_on_hit));
    }
}

/// A component that marks an entity as a "bullet" (projectile).
///
/// The entity with this component represents a fired projectile
/// (e.g. an archer's arrow, a magic bolt) that travels through the
/// game world. Bullets typically carry additional components for
/// velocity, damage, and lifetime, and are cleaned up upon collision
/// with an enemy or after a timeout.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Bullet;

/// Message emitted when a bullet collides with another entity.
///
/// Carries the **rigid body** entities involved in the collision (resolved
/// from [`CollisionStart`]'s `body1`/`body2` fields, not the collider
/// entities). Systems can read this message to react to bullet hits
/// (e.g. apply damage, spawn effects, despawn the bullet).
#[derive(Message, Clone, TypePath)]
pub struct BulletBattleEvent {
    /// The bullet entity involved in the collision.
    pub bullet: Entity,
    /// The other entity the bullet collided with.
    pub other: Entity,
}

/// System that emits [`BulletBattleEvent`] when a bullet collides with
/// another entity.
///
/// Reads [`CollisionStart`] events from avian2d via bevy_gearbox, uses the
/// rigid body entities (`body1`/`body2`) from the collision event, checks
/// whether either carries a [`Bullet`] component, and writes a
/// [`BulletBattleEvent`] with the bullet entity and the other entity.
/// Events where a rigid body entity is missing are skipped.
pub fn emit_bullet_battle_event(
    mut started: MessageReader<CollisionStart>,
    bullets: Query<&Bullet>,
    mut writer: MessageWriter<BulletBattleEvent>,
) {
    for event in started.read() {
        let (e1, e2) = match (event.body1, event.body2) {
            (Some(b1), Some(b2)) => (b1, b2),
            _ => continue,
        };
        if bullets.contains(e1) {
            writer.write(BulletBattleEvent {
                bullet: e1,
                other: e2,
            });
        } else if bullets.contains(e2) {
            writer.write(BulletBattleEvent {
                bullet: e2,
                other: e1,
            });
        }
    }
}

/// System that despawns the [`Bullet`] entity whenever a [`BulletBattleEvent`]
/// is received.
///
/// This provides a simple default behaviour: a bullet is immediately removed
/// upon colliding with anything. More sophisticated systems (damage, health,
/// pierce) can be added alongside or instead of this one.
pub fn despawn_on_hit(
    mut events: MessageReader<BulletBattleEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands.entity(event.bullet).despawn();
    }
}

/// A marker component that indicates the bullet's starting position
/// has been recorded.
///
/// This marker is attached to bullet entities during spawning to
/// identify projectiles whose origin position should be tracked
/// for distance checks, range limits, or origin queries on collision.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct BulletPosition;

/// A reference to a [`BulletPosition`] entity.
///
/// Stores an [`Entity`] that points to a target entity carrying the
/// [`BulletPosition`] component. Used at runtime to retrieve the
/// world position of the bullet's origin.
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct BulletPositionTarget(pub Entity);

/// Spawn a bullet at a given position with a given velocity and collision layers.
///
/// Returns a bundle containing a [`Bullet`] marker, a [`Sprite`] of
/// 2×16 pixels (width × height), a [`Transform`] positioned at `position`,
/// and physics components ([`RigidBody::Kinematic`], [`Collider::rectangle`],
/// [`CollisionLayers`], [`LinearVelocity`], [`CollisionEventsEnabled`],
/// [`Sensor`]).
pub fn bullet(position: Vec2, velocity: Vec2, layers: CollisionLayers) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet,
        Sprite::from_color(Color::srgb(1.0, 0.8, 0.0), Vec2::new(2.0, 16.0)),
        Transform::from_xyz(position.x, position.y, VisualDisplayLayer::Bullet.z_value()),
        Visibility::default(),
        RigidBody::Kinematic,
        Collider::rectangle(2.0, 16.0),
        layers,
        LinearVelocity(velocity),
        CollisionEventsEnabled,
        Sensor,
    )
}
