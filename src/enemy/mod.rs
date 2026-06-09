//! Enemy module.
//!
//! Defines the [`Enemy`] component, which marks an entity as an enemy unit.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Enemy>();
}

/// A component that marks an entity as an "enemy".
///
/// The entity with this component represents an enemy unit that occupies
/// one map cell (64×64 px). Enemies carry additional components for their
/// attributes, AI mode, and drop data, and are driven by the enemy system's
/// ECS systems.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;
