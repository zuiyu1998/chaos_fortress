//! Role module.
//!
//! Defines the [`Role`] component, which marks an entity as a controllable
//! character (player side), along with [`RoleBuilder`], [`RoleBuilderContext`],
//! and [`RoleBuilderContainer`] for flexible role entity construction.

pub mod archer;

use std::collections::HashMap;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Role>();
    app.register_type::<Archer>();
    app.register_type::<archer::AttackSpeed>();
    app.register_type::<archer::ProjectileDamage>();
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

/// A component that marks an entity as an "archer" (ranged role variant).
///
/// Archer entities carry a [`Role`] marker in addition to this component,
/// and typically have extra components for attack range, speed, and
/// projectile damage.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Archer;

/// Spawn a role entity as a child via [`ChildSpawnerCommands`] (a type alias for
/// [`RelatedSpawnerCommands`]`<ChildOf>`).
///
/// Uses the [`RoleBuilderContainer`] to look up the `"archer"` builder to generate
/// an archer entity.
pub fn role<'w>(
    spawner: &mut ChildSpawnerCommands<'w>,
    container: &RoleBuilderContainer,
    column: u32,
    row: u32,
) -> Entity {
    let ctx = RoleBuilderContext {
        position: (column, row),
        parent: Some(spawner.target_entity()),
    };
    let mut cmds = spawner.commands();
    container
        .build("archer", &mut cmds, ctx)
        .expect("RoleBuilderContainer is missing the 'archer' builder")
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
    /// Create a container pre-populated with default builders.
    ///
    /// Currently includes:
    /// - `"archer"` — an [`ArcherRoleBuilder`] with default combat stats.
    pub fn new() -> Self {
        let mut container = Self {
            builders: HashMap::new(),
        };
        container.register(
            "archer",
            archer::ArcherRoleBuilder {
                name: "Archer".into(),
                attack_range: 300.0,
                attack_speed: 0.8,
                projectile_damage: 15.0,
            },
        );
        container
    }

    /// Register a named builder from a [`RoleBuilder`] implementor.
    pub fn register(&mut self, name: impl Into<String>, builder: impl RoleBuilder + 'static) {
        let name = name.into();
        self.builders.insert(
            name,
            Box::new(move |commands, ctx| builder.build(commands, ctx)),
        );
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
