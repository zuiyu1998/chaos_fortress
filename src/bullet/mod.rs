//! Bullet module.
//!
//! Defines the [`Bullet`] component, which marks an entity as a projectile
//! (bullet) that can collide with enemies and deal damage.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Bullet>();
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
