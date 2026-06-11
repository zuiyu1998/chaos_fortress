//! Bullet module.
//!
//! Defines the [`Bullet`] component, which marks an entity as a projectile
//! (bullet) that can collide with enemies and deal damage.

use avian2d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;

use crate::common::{GamePhysicsLayer, VisualDisplayLayer};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Bullet>();
    app.register_type::<BulletPosition>();
    app.register_type::<BulletPositionTarget>();
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

/// Spawn a bullet at a given position with a given velocity.
///
/// Returns a bundle containing a [`Bullet`] marker, a [`Sprite`] of
/// 2×16 pixels (width × height), a [`Transform`] positioned at `position`,
/// and physics components ([`RigidBody::Dynamic`], [`Collider::rectangle`],
/// [`CollisionLayers`], [`LinearVelocity`]).
pub fn bullet(position: Vec2, velocity: Vec2) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet,
        Sprite::from_color(Color::srgb(1.0, 0.8, 0.0), Vec2::new(2.0, 16.0)),
        Transform::from_xyz(position.x, position.y, VisualDisplayLayer::Bullet.z_value()),
        Visibility::default(),
        RigidBody::Dynamic,
        Collider::rectangle(2.0, 16.0),
        GamePhysicsLayer::character_layers(),
        LinearVelocity(velocity),
    )
}
