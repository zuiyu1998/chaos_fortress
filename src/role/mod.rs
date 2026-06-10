//! Role module.
//!
//! Defines the [`Role`] component, which marks an entity as a controllable
//! character (player side), along with [`RoleBuilder`], [`RoleBuilderContext`],
//! and [`RoleBuilderContainer`] for flexible role entity construction.

use std::collections::HashMap;

use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::common::{GamePhysicsLayer, VisualDisplayLayer};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Role>();
    app.insert_resource(RoleBuilderContainer::new());
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

/// Spawn a role sprite at a given grid position.
///
/// Returns a bundle containing a [`Role`] marker, a [`Sprite`], and a
/// [`Transform`] positioned at the center of the given grid cell.
/// The grid coordinate system matches [`map::map_cell`]: cell (0,0) is
/// the top-left of the grid, and the grid is centered on its parent.
pub fn role(cell_size: f32, column: u32, row: u32, sprite: Sprite) -> impl Bundle {
    let x = column as f32 * cell_size;
    let y = -(row as f32 * cell_size);
    (
        Name::new(format!("Role ({column}, {row})")),
        Role,
        sprite,
        Transform::from_xyz(x, y, VisualDisplayLayer::Character.z_value()),
        Visibility::default(),
        RigidBody::Kinematic,
        Collider::circle(cell_size / 2.0),
        GamePhysicsLayer::character_layers(),
    )
}

/// Context for building a role entity.
///
/// Encapsulates grid position and optional parent entity.
/// The `Commands` reference is passed directly to
/// [`RoleBuilder::build`] instead of being stored here.
pub struct RoleBuilderContext {
    /// Grid position as `(column, row)`. Column maps to the X axis,
    /// row to the Y axis (origin at top-left).
    pub position: (u32, u32),
    /// Optional parent entity. When `Some`, the built entity should be
    /// parented to this entity.
    pub parent: Option<Entity>,
}

/// Trait for building role entities.
///
/// Implementors define how a specific type of role (player, NPC, etc.)
/// is spawned. The [`build`](RoleBuilder::build) method receives
/// a `&mut Commands` and a [`RoleBuilderContext`].
pub trait RoleBuilder: Send + Sync {
    /// Build a role entity and return its `Entity` ID.
    fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: RoleBuilderContext) -> Entity;
}

/// A Bevy [`Resource`] that maps role names to builder closures.
///
/// Use [`register`](RoleBuilderContainer::register) to register builders,
/// and [`build`](RoleBuilderContainer::build) to construct entities by name.
#[derive(Resource)]
pub struct RoleBuilderContainer {
    builders: HashMap<
        String,
        Box<dyn for<'w, 's> Fn(&'w mut Commands<'w, 's>, RoleBuilderContext) -> Entity + Send + Sync>,
    >,
}

impl RoleBuilderContainer {
    /// Create an empty container.
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    /// Register a named builder from a [`RoleBuilder`] implementor.
    pub fn register(&mut self, name: impl Into<String>, builder: impl RoleBuilder + 'static) {
        let name = name.into();
        self.builders.insert(
            name,
            Box::new(move |commands, ctx| builder.build(commands, ctx)),
        );
    }

    /// Register a named builder closure directly.
    ///
    /// This is a convenience method for inline closures without defining
    /// a named type implementing [`RoleBuilder`].
    pub fn register_fn<F>(&mut self, name: impl Into<String>, builder: F)
    where
        F: for<'w, 's> Fn(&'w mut Commands<'w, 's>, RoleBuilderContext) -> Entity + Send + Sync + 'static,
    {
        self.builders.insert(name.into(), Box::new(builder));
    }

    /// Look up a builder by name and execute it to spawn an entity.
    ///
    /// Returns `None` if no builder is registered for `name`.
    pub fn build<'w, 's>(
        &self,
        name: &str,
        commands: &'w mut Commands<'w, 's>,
        ctx: RoleBuilderContext,
    ) -> Option<Entity> {
        self.builders.get(name).map(|f| f(commands, ctx))
    }
}

impl Default for RoleBuilderContainer {
    fn default() -> Self {
        Self::new()
    }
}
