//! Enemy module.
//!
//! Defines the [`Enemy`] component, which marks an entity as an enemy unit,
//! along with [`EnemyBuilder`], [`EnemyBuilderContext`], and
//! [`EnemyBuilderContainer`] for flexible enemy entity construction.

use std::collections::HashMap;

use avian2d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::attribute::{Attribute, AttributeSet};
use crate::battle::battle;
use crate::common::{GamePhysicsLayer, VisualDisplayLayer};

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>();
        app.insert_resource(EnemyBuilderContainer::new());
        let mut container = app.world_mut().resource_mut::<EnemyBuilderContainer>();
        container.register("basic", BasicEnemyBuilder);
    }
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

/// Spawn an enemy entity as a child via [`ChildSpawnerCommands`].
///
/// Uses the [`EnemyBuilderContainer`] to look up the `"basic"` builder to
/// generate an enemy entity.
pub fn enemy<'w>(
    spawner: &mut ChildSpawnerCommands<'w>,
    container: &EnemyBuilderContainer,
    column: u32,
    row: u32,
    cell_size: f32,
) -> Entity {
    let ctx = EnemyBuilderContext {
        position: (column, row),
        cell_size,
        parent: Some(spawner.target_entity()),
    };
    let mut cmds = spawner.commands();
    container
        .build("basic", &mut cmds, ctx)
        .expect("EnemyBuilderContainer build failed for 'basic'")
}

/// A basic enemy builder that produces a simple red-square enemy.
pub struct BasicEnemyBuilder;

impl EnemyBuilder for BasicEnemyBuilder {
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: EnemyBuilderContext,
    ) -> Result<Entity, EnemyBuildError> {
        let (col, row) = ctx.position;
        let cell_size = ctx.cell_size;
        let x = col as f32 * cell_size;
        let y = -(row as f32 * cell_size);
        let mut attrs = AttributeSet::new();
        attrs.insert("hp", Attribute::new(100.0));
        attrs.insert("max_hp", Attribute::new(100.0));
        attrs.insert("armor", Attribute::new(10.0));
        let mut entity = commands.spawn((
            Name::new(format!("Enemy ({col},{row})")),
            Enemy,
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(cell_size)),
            Transform::from_xyz(x, y, VisualDisplayLayer::Character.z_value()),
            Visibility::default(),
            RigidBody::Dynamic,
            Collider::circle(cell_size / 2.0),
            GamePhysicsLayer::enemy_layers(),
            LinearVelocity(Vec2::new(0.0, -10.0)),
            battle(attrs),
        ));
        if let Some(parent) = ctx.parent {
            entity.set_parent_in_place(parent);
        }
        Ok(entity.id())
    }
}

/// Error returned when building an enemy entity fails.
#[derive(Debug)]
pub enum EnemyBuildError {
    /// No builder is registered for the given name.
    MissingBuilder(String),
    /// The builder encountered an error during construction.
    BuildFailed(String),
}

impl std::fmt::Display for EnemyBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingBuilder(name) => write!(f, "no enemy builder registered for '{name}'"),
            Self::BuildFailed(msg) => write!(f, "enemy build failed: {msg}"),
        }
    }
}

impl std::error::Error for EnemyBuildError {}

/// Context for building an enemy entity.
///
/// Encapsulates grid position, cell size, and optional parent entity.
/// The `Commands` reference is passed directly to
/// [`EnemyBuilder::build`] instead of being stored here.
pub struct EnemyBuilderContext {
    /// Grid position as `(column, row)`. Column maps to the X axis,
    /// row to the Y axis (origin at top-left).
    pub position: (u32, u32),
    /// Pixel size of each grid cell (square side length), used to convert
    /// grid coordinates to screen coordinates: `x = col * cell_size`,
    /// `y = -(row * cell_size)`.
    pub cell_size: f32,
    /// Optional parent entity. When `Some`, the built entity should be
    /// parented to this entity.
    pub parent: Option<Entity>,
}

/// Trait for building enemy entities.
///
/// Implementors define how a specific type of enemy (normal, elite, boss,
/// summon, etc.) is spawned. The [`build`](EnemyBuilder::build) method
/// receives a `&mut Commands` and an [`EnemyBuilderContext`].
pub trait EnemyBuilder: Send + Sync {
    /// Build an enemy entity and return its `Entity` ID on success.
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: EnemyBuilderContext,
    ) -> Result<Entity, EnemyBuildError>;
}

/// A Bevy [`Resource`] that maps enemy type names to builder closures.
///
/// Use [`register`](EnemyBuilderContainer::register) to register builders,
/// and [`build`](EnemyBuilderContainer::build) to construct entities by name.
#[derive(Resource)]
pub struct EnemyBuilderContainer {
    builders: HashMap<
        String,
        Box<dyn for<'w, 's> Fn(&'w mut Commands<'w, 's>, EnemyBuilderContext) -> Result<Entity, EnemyBuildError> + Send + Sync>,
    >,
}

impl EnemyBuilderContainer {
    /// Create an empty container.
    ///
    /// Builders must be registered via [`register`](EnemyBuilderContainer::register)
    /// by individual enemy plugins.
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    /// Register a named builder from an [`EnemyBuilder`] implementor.
    pub fn register(&mut self, name: impl Into<String>, builder: impl EnemyBuilder + 'static) {
        let name = name.into();
        self.builders.insert(
            name,
            Box::new(move |commands, ctx| builder.build(commands, ctx)),
        );
    }

    /// Look up a builder by name and execute it to spawn an entity.
    ///
    /// Returns `Err(EnemyBuildError::MissingBuilder)` if no builder is registered
    /// for `name`, or forwards errors from the builder itself.
    pub fn build<'w, 's>(
        &self,
        name: &str,
        commands: &'w mut Commands<'w, 's>,
        ctx: EnemyBuilderContext,
    ) -> Result<Entity, EnemyBuildError> {
        self.builders
            .get(name)
            .ok_or_else(|| EnemyBuildError::MissingBuilder(name.to_string()))
            .and_then(|f| f(commands, ctx))
    }
}

impl Default for EnemyBuilderContainer {
    fn default() -> Self {
        Self::new()
    }
}
