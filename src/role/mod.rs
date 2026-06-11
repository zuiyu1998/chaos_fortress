//! Role module.
//!
//! Defines the [`Role`] component, which marks an entity as a controllable
//! character (player side), along with [`RoleBuilder`], [`RoleBuilderContext`],
//! and [`RoleBuilderContainer`] for flexible role entity construction.

pub mod archer;

use std::collections::HashMap;

use avian2d::prelude::{CollisionEnd, CollisionStart};
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::common::{AttackRange, EnemyTarget, EnemyTargetList};
use crate::enemy::Enemy;

pub(super) struct RolePlugin;

impl Plugin for RolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Role>();
        app.insert_resource(RoleBuilderContainer::new());
        app.add_plugins(archer::ArcherPlugin);

        app.add_systems(
            Update,
            (
                update_enemy_target_on_collision,
                cleanup_enemy_target_list,
                sync_primary_target,
            ),
        );
    }
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
    /// Create an empty container.
    ///
    /// Builders must be registered via [`register`](RoleBuilderContainer::register)
    /// by individual role plugins (e.g. `ArcherPlugin`).
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

/// System to update [`EnemyTargetList`] based on collision events.
///
/// When a `Role` (or its `AttackRange` sensor child) collides with an
/// `Enemy`, the enemy is added to the role's [`EnemyTargetList`].
/// On `CollisionEnded` it is removed.
pub fn update_enemy_target_on_collision(
    mut started: MessageReader<CollisionStart>,
    mut ended: MessageReader<CollisionEnd>,
    sensors: Query<&ChildOf, With<AttackRange>>,
    mut lists: Query<&mut EnemyTargetList>,
    enemies: Query<&Enemy>,
) {
    for event in started.read() {
        let (e1, e2) = (event.collider1, event.collider2);
        if let Some(role_entity) = find_role(e1, e2, &sensors, &enemies) {
            if let Ok(mut list) = lists.get_mut(role_entity) {
                let enemy = if enemies.contains(e1) { e1 } else { e2 };
                if !list.0.contains(&enemy) {
                    list.0.push(enemy);
                }
            }
        }
    }
    for event in ended.read() {
        let (e1, e2) = (event.collider1, event.collider2);
        if let Some(role_entity) = find_role(e1, e2, &sensors, &enemies) {
            if let Ok(mut list) = lists.get_mut(role_entity) {
                let enemy = if enemies.contains(e1) { e1 } else { e2 };
                list.0.retain(|e| e != &enemy);
            }
        }
    }
}

/// Resolve which entity in a collision pair is the role root entity.
fn find_role(
    e1: Entity,
    e2: Entity,
    sensors: &Query<&ChildOf, With<AttackRange>>,
    enemies: &Query<&Enemy>,
) -> Option<Entity> {
    if enemies.contains(e1) {
        resolve_role_root(e2, sensors)
    } else if enemies.contains(e2) {
        resolve_role_root(e1, sensors)
    } else {
        None
    }
}

/// Walk up from a possible sensor child entity to the role root entity.
fn resolve_role_root(
    entity: Entity,
    sensors: &Query<&ChildOf, With<AttackRange>>,
) -> Option<Entity> {
    if sensors.get(entity).is_ok() {
        sensors.get(entity).ok().map(|p| p.get())
    } else {
        Some(entity)
    }
}

/// Remove despawned or no-longer-valid enemies from all [`EnemyTargetList`]s.
///
/// Cleans up entities that have been despawned or lost their [`Enemy`]
/// component, preventing stale references from accumulating.
pub fn cleanup_enemy_target_list(
    mut lists: Query<&mut EnemyTargetList>,
    enemies: Query<&Enemy>,
) {
    for mut list in &mut lists {
        list.0.retain(|e| enemies.contains(*e));
    }
}

/// Sync [`EnemyTarget`] from [`EnemyTargetList`] by picking the first entry.
///
/// If the list is non-empty, sets `EnemyTarget` to the first entity;
/// if the list is empty, sets it to `None`.
pub fn sync_primary_target(
    mut query: Query<(&EnemyTargetList, &mut EnemyTarget)>,
) {
    for (list, mut target) in &mut query {
        target.0 = list.0.first().copied();
    }
}
