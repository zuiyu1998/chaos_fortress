//! Role module.
//!
//! Defines the [`Role`] component, which marks an entity as a controllable
//! character (player side).

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Role>();
}

/// A component that marks an entity as a "role" (character).
///
/// The entity with this component represents a controllable character unit
/// that occupies one map cell (64×64 px). Roles carry additional components
/// for their attributes (health, attack, defense, etc.) and are driven by
/// the role system's ECS systems.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Role;
