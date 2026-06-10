//! Physics debug rendering plugin.
//!
//! Renders collider shapes, contacts, and other physics debugging
//! information using Bevy gizmos.

use avian2d::debug_render::PhysicsDebugPlugin as AvianPhysicsDebugPlugin;
use bevy::prelude::*;

/// A thin wrapper / re-export of avian2d's `PhysicsDebugPlugin`.
///
/// This module exists so that physics debug configuration can be fine-tuned
/// without touching the upstream plugin directly.
pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AvianPhysicsDebugPlugin);
    }
}
