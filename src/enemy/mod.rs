//! Enemy module.
//!
//! Defines the [`Enemy`] component, which marks an entity as an enemy unit,
//! and the [`Base`] component, which marks an entity as a base (enemy no-go
//! zone). Provides flexible enemy entity construction through
//! [`EnemyBuilder`], [`EnemyBuilderContext`], and
//! [`EnemyBuilderContainer`], along with collision-based game-over detection
//! via [`check_enemy_enters_base`].

use std::collections::HashMap;

use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionStart, LinearVelocity, RigidBody, Sensor};
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::attribute::{Attribute, AttributeSet, AttributeTemplate};
use crate::battle::{battle, DeathInBattle};
use crate::common::{GamePhysicsLayer, VisualDisplayLayer};
use crate::dropped_items::gold_drop;
use crate::role::archer::ProjectileDamage;
use crate::state::Finish;

pub(super) mod assets;

/// Entry plugin for the enemy module.
///
/// Registers enemy-related ECS types ([`Enemy`], [`Base`]) and systems
/// ([`check_enemy_enters_base`]) with the Bevy [`App`], and initializes
/// the [`EnemyBuilderContainer`] resource and [`assets::EnemyAssets`].
///
/// # Registration
///
/// | Item | Kind | Description |
/// |---|---|---|
/// | [`Enemy`] | Component | Marks an entity as an enemy unit |
/// | [`Base`] | Component | Marks an entity as a base (enemy no-go zone) |
/// | [`EnemyBuilderContainer`] | Resource | Registry mapping enemy type names to builders |
/// | [`assets::EnemyAssets`] | Resource/Asset | Handles for enemy asset dependencies |
/// | [`check_enemy_enters_base`] | System | Monitors collisions between enemies and the base |
///
/// # Usage
///
/// ```rust
/// app.add_plugins(enemy::EnemyPlugin);
/// ```
///
/// # Interaction flow
///
/// 1. This plugin registers [`Enemy`], [`Base`] components and
///    [`check_enemy_enters_base`] into the Bevy world.
/// 2. The level system spawns the map, a base entity (via [`base`]),
///    and enemy entities (via [`enemy`]).
/// 3. Enemy AI systems drive enemy movement.
/// 4. When an enemy enters the base area, [`check_enemy_enters_base`]
///    detects the collision and logs `"游戏已结算"`.
pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>();
        app.register_type::<Base>();
        app.insert_resource(EnemyBuilderContainer::new());
        app.load_resource::<assets::EnemyAssets>();
        let mut container = app.world_mut().resource_mut::<EnemyBuilderContainer>();
        container.register("basic", BasicEnemyBuilder);

        app.add_systems(Update, check_enemy_enters_base);
        app.add_systems(Update, drop_gold_on_enemy_death);
    }
}

/// Drops gold when an enemy dies.
///
/// Reads [`DeathInBattle`] messages; when the dead entity is an enemy,
/// spawns a gold drop via [`gold_drop`].
fn drop_gold_on_enemy_death(
    mut events: MessageReader<DeathInBattle>,
    enemies: Query<&Enemy>,
    mut commands: Commands,
) {
    for event in events.read() {
        if enemies.contains(event.entity) {
            commands.spawn(gold_drop(5));
        }
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

/// A component that marks an entity as a "base" (home base).
///
/// Base entities define an area that enemies must not enter. The enemy
/// AI treats cells occupied by a base as **blocked** — no enemy unit
/// can path into or stand on them. This prevents enemies from reaching
/// the player's deployment zone.
///
/// Unlike [`World`](crate::common::GamePhysicsLayer) collision layers,
/// which handle physical barriers, `Base` is a **logical boundary** that
/// the enemy AI queries during movement planning. It may also carry a
/// [`Collider`] with [`base_layers`](crate::common::GamePhysicsLayer::base_layers)
/// so that enemies that do collide with it trigger game-over logic.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Base;

/// System that monitors collision events between enemies and the base.
///
/// When an enemy collides with the base (a [`Base`] entity), the game is
/// considered over. This system prints `"游戏已结算"` to signal the game
/// has been settled and sets [`Finish`] to `true`.
pub fn check_enemy_enters_base(
    mut started: MessageReader<CollisionStart>,
    bases: Query<&Base>,
    enemies: Query<&Enemy>,
    mut next_finish: ResMut<NextState<Finish>>,
) {
    for event in started.read() {
        let (e1, e2) = (event.collider1, event.collider2);
        if (bases.contains(e1) && enemies.contains(e2))
            || (bases.contains(e2) && enemies.contains(e1))
        {
            info!("游戏已结算");
            next_finish.set(Finish(true));
        }
    }
}

/// Spawn a base entity at the given grid position.
///
/// The base spans a rectangular area of `width` and `height` (in grid
/// cells), starting from `(column, row)` and extending rightward and
/// upward (negative y). Each cell is `cell_size` pixels square.
///
/// The base entity carries:
/// - A [`Base`] marker component.
/// - A [`Collider::rectangle`] matching the spanned area.
/// - [`CollisionEventsEnabled`] so collision events are emitted.
/// - [`Sensor`] so the collider acts as a trigger (no physical pushback).
/// - [`GamePhysicsLayer::base_layers()`] so it physically interacts
///   only with enemies.
pub fn base(
    column: u32,
    row: u32,
    cell_size: f32,
    width: u32,
    height: u32,
) -> impl Bundle {
    let center_x = (column as f32 + (width as f32 - 1.0) / 2.0) * cell_size;
    let center_y = -(row as f32 + (height as f32 - 1.0) / 2.0) * cell_size;
    let px_width = width as f32 * cell_size;
    let px_height = height as f32 * cell_size;
    (
        Name::new(format!("Base ({column},{row}) {width}x{height}")),
        Base,
        Transform::from_xyz(center_x, center_y, VisualDisplayLayer::Character.z_value()),
        Collider::rectangle(px_width, px_height),
        RigidBody::Kinematic,
        CollisionEventsEnabled,
        Sensor,
        GamePhysicsLayer::base_layers(),
    )
}

/// Spawn an enemy entity as a child via [`ChildSpawnerCommands`].
///
/// Uses the [`EnemyBuilderContainer`] to look up the `"basic"` builder to
/// generate an enemy entity. The [`AttributeSet`] is built from the basic enemy
/// attribute template loaded via [`assets::EnemyAssets`].
pub fn enemy<'w>(
    spawner: &mut ChildSpawnerCommands<'w>,
    container: &EnemyBuilderContainer,
    column: u32,
    row: u32,
    cell_size: f32,
    enemy_assets: &assets::EnemyAssets,
    template_assets: &Assets<AttributeTemplate>,
) -> Entity {
    let attrs = template_assets
        .get(&enemy_assets.basic_attributes)
        .map(|t| t.build_attribute_set(&["hp", "max_hp", "armor", "attack"]))
        .unwrap_or_else(|| {
            let mut a = AttributeSet::new();
            a.insert("hp", Attribute::new(100.0));
            a.insert("max_hp", Attribute::new(100.0));
            a.insert("armor", Attribute::new(10.0));
            a.insert("attack", Attribute::new(10.0));
            a
        });
    let ctx = EnemyBuilderContext {
        position: (column, row),
        cell_size,
        parent: Some(spawner.target_entity()),
        attributes: attrs,
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
        let projectile_damage = ctx.attributes
            .get("attack")
            .map(|a| a.value)
            .ok_or_else(|| EnemyBuildError::BuildFailed("missing attribute 'attack'".into()))?;
        let mut entity = commands.spawn((
            Name::new(format!("Enemy ({col},{row})")),
            Enemy,
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(cell_size)),
            Transform::from_xyz(x, y, VisualDisplayLayer::Character.z_value()),
            Visibility::default(),
            RigidBody::Dynamic,
            Collider::circle(cell_size / 2.0),
            GamePhysicsLayer::enemy_layers(),
            LinearVelocity(Vec2::new(-10.0, 0.0)),
            ProjectileDamage(projectile_damage),
            battle(ctx.attributes),
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
/// Encapsulates grid position, cell size, optional parent entity, and
/// [`AttributeSet`] for combat stats.
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
    /// Attribute set for combat stats. The builder should use these
    /// attributes when constructing the entity.
    pub attributes: AttributeSet,
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
