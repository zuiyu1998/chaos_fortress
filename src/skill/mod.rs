//! Skill module.
//!
//! Defines the [`SkillDefinition`] asset, the [`SkillInstance`] component,
//! and helper traits for converting between skill features and runtime types.
//!
//! # Overview
//!
//! - **[`SkillDefinition`]** — An [`Asset`] that defines a skill template (id, name, features).
//! - **[`SkillFeatureDefinition`]** — A plain struct holding a numeric parameter dictionary,
//!   embedded inside [`SkillDefinition`].
//! - **[`SkillInstance`]** — A [`Component`] that tracks the runtime state of a skill on an entity.
//! - **[`SkillStatus`]** — An enum representing the skill's runtime status (Ready, Cooling, etc.).
//! - **[`SkillFeatureBuilderContext`]** — Context passed to [`SkillFeatureBuilder`].
//! - **[`BuildError`]** — Error type for builder operations.
//! - **[`SkillFeatureBuilder`]** — A trait for spawning entities from skill features.
//! - **[`SkillFeatureBuilderContainer`]** — A [`Resource`] that maps feature ids to builders.
//! - **[`CooldownFeatureBuilder`]** — A [`SkillFeatureBuilder`] that attaches a [`CoolingTimer`] to the skill entity.
//! - **[`SkillFeatureResult`]** — A trait for checking feature execution status.
//! - **[`SkillTarget`]** — A [`Component`] referencing the skill entity, attached on the owner.
//! - **[`SkillPlugin`]** — Registers the asset type, component, and container with Bevy.

use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use std::collections::HashMap;

use crate::common::CoolingTimer;

pub(super) mod loader;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

/// Plugin that registers skill-related types.
///
/// Registers [`SkillDefinition`] as an [`Asset`] and [`SkillInstance`] as a
/// type-registered [`Component`].
pub(super) struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SkillDefinition>();
        app.register_type::<SkillInstance>();
        app.register_type::<CooldownFeature>();
        app.register_type::<SkillRunContext>();
        app.register_type::<SkillTarget>();
        app.init_resource::<SkillFeatureBuilderContainer>();
        app.register_asset_loader(loader::SkillDefinitionLoader);

        app.add_systems(Update, tick_cooldown_features);
    }
}

// ---------------------------------------------------------------------------
// Factory function
// ---------------------------------------------------------------------------

/// Spawn a skill entity as a child of `target` from a [`SkillDefinition`].
///
/// Returns the spawned child entity's [`Entity`] ID.
///
/// Creates a [`SkillInstance`] and a [`Name`] component on a new child entity,
/// then iterates over `definition.features` and uses the [`SkillFeatureBuilderContainer`]
/// to apply each feature's builder with `skill` set to the child and `target` set
/// to the parent.
///
/// ```ignore
/// let skill_entity = skill(&mut spawner, &container, &my_skill_def);
/// ```
pub fn skill(
    spawner: &mut ChildSpawnerCommands,
    container: &SkillFeatureBuilderContainer,
    definition: &SkillDefinition,
) -> Entity {
    let target = spawner.target_entity();
    let mut commands = spawner.commands();
    let skill_entity = commands
        .spawn((
            Name::new(format!("Skill ({})", definition.name)),
            SkillInstance::new(&definition.id),
            SkillRunContext::new(target),
        ))
        .id();

    container.build_all(definition, skill_entity, target, &mut commands);

    skill_entity
}

// ---------------------------------------------------------------------------
// SkillFeatureDefinition (plain struct — not Component, not Asset)
// ---------------------------------------------------------------------------

/// A numeric feature definition for a skill effect.
///
/// Each instance carries a unique `id` (e.g. `"damage"`, `"range"`, `"duration"`)
/// and a [`HashMap`] of named numeric parameters.
///
/// This is a plain struct — it is **not** a Bevy [`Component`] or [`Asset`].
/// It is always embedded inside [`SkillDefinition`] as part of its `features` list.
#[derive(Debug, Clone, PartialEq)]
pub struct SkillFeatureDefinition {
    /// Unique identifier for this feature category (e.g. "damage", "range", "duration").
    pub id: String,
    /// Numeric parameter dictionary; keys are parameter names, values are f32.
    pub features: HashMap<String, f32>,
}

impl SkillFeatureDefinition {
    /// Create a new [`SkillFeatureDefinition`] with the given `id` and an empty dictionary.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            features: HashMap::new(),
        }
    }

    /// Get the value for `key`, or `None` if absent.
    pub fn get(&self, key: &str) -> Option<f32> {
        self.features.get(key).copied()
    }

    /// Set the value for `key`.
    pub fn set(&mut self, key: impl Into<String>, value: f32) {
        self.features.insert(key.into(), value);
    }
}

// ---------------------------------------------------------------------------
// SkillDefinition (Asset)
// ---------------------------------------------------------------------------

/// A skill template asset.
///
/// Defines the static properties of a skill: a unique `id`, a display `name`,
/// and a list of [`SkillFeatureDefinition`] entries that carry numeric parameters.
///
/// Can be loaded from external files via Bevy's asset system, or constructed
/// programmatically.
#[derive(Asset, Debug, Clone, TypePath)]
pub struct SkillDefinition {
    /// Unique skill identifier (e.g. `"archer_shot"`, `"enemy_charge"`).
    pub id: String,
    /// Display name for UI and logging.
    pub name: String,
    /// List of numeric feature definitions.
    pub features: Vec<SkillFeatureDefinition>,
}

impl SkillDefinition {
    /// Create a new [`SkillDefinition`] with the given `id` and `name`, and an empty features list.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            features: Vec::new(),
        }
    }

    /// Find a feature by its `feature_id`.
    ///
    /// Returns `None` when no feature matches.
    pub fn get_feature(&self, feature_id: &str) -> Option<&SkillFeatureDefinition> {
        self.features.iter().find(|f| f.id == feature_id)
    }

    /// Append a [`SkillFeatureDefinition`] to the features list.
    pub fn add_feature(&mut self, feature: SkillFeatureDefinition) {
        self.features.push(feature);
    }
}

// ---------------------------------------------------------------------------
// SkillInstance (Component)
// ---------------------------------------------------------------------------

/// Runtime state of a single skill on an entity.
///
/// Each instance tracks its own charges and status.
/// It references a [`SkillDefinition`] via `skill_id`.
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct SkillInstance {
    /// The [`SkillDefinition`] ID this instance refers to.
    pub skill_id: String,
    /// Current charge count. At least 1 charge is required to use the skill.
    pub charges: u32,
    /// Maximum charge count. `0` means no charge system (cooldown only).
    pub max_charges: u32,
    /// Current runtime status.
    pub status: SkillStatus,
}

impl SkillInstance {
    /// Create a new [`SkillInstance`] with the given `skill_id`.
    ///
    /// The instance starts in [`SkillStatus::Ready`] with zero charges (no charge system).
    pub fn new(skill_id: impl Into<String>) -> Self {
        Self {
            skill_id: skill_id.into(),
            charges: 0,
            max_charges: 0,
            status: SkillStatus::Ready,
        }
    }

    /// Returns `true` when the skill can be used:
    /// - status is `Ready`
    /// - has at least one charge (when a charge system is active)
    pub fn ready(&self) -> bool {
        if self.status != SkillStatus::Ready {
            return false;
        }
        if self.max_charges > 0 && self.charges == 0 {
            return false;
        }
        true
    }

    /// Use the skill: consume one charge and set status to [`SkillStatus::Cooling`].
    ///
    /// Panics if the skill is not [`ready`](Self::ready).
    pub fn use_skill(&mut self) {
        assert!(self.ready(), "use_skill called when skill is not ready");
        if self.max_charges > 0 {
            self.charges = self.charges.saturating_sub(1);
        }
        self.status = SkillStatus::Cooling;
    }

    /// Add `count` charges (capped at `max_charges`).
    pub fn add_charge(&mut self, count: u32) {
        if self.max_charges > 0 {
            self.charges = (self.charges + count).min(self.max_charges);
        }
    }
}

/// Runtime status of a [`SkillInstance`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum SkillStatus {
    /// Ready to be used.
    Ready,
    /// Currently cooling down.
    Cooling,
    /// Channeling — cannot be interrupted.
    Channeling,
    /// Disabled (silenced, stunned, etc.).
    Disabled,
}

// ---------------------------------------------------------------------------
// CooldownFeature
// ---------------------------------------------------------------------------

/// A cooldown feature for a skill.
///
/// Carries the cooldown duration in seconds. Implements
/// [`FromSkillFeatureDefinition`] and [`IntoSkillFeatureDefinition`] for
/// bidirectional conversion with [`SkillFeatureDefinition`].
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CooldownFeature {
    /// Cooldown duration in seconds. Defaults to `1.0`.
    pub cooldown_duration: f32,
}

impl CooldownFeature {
    /// Create a new [`CooldownFeature`] with the default duration of 1.0 s.
    pub fn new() -> Self {
        Self {
            cooldown_duration: 1.0,
        }
    }
}

impl Default for CooldownFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl FromSkillFeatureDefinition for CooldownFeature {
    fn from_feature(def: &SkillFeatureDefinition) -> Option<Self> {
        Some(Self {
            cooldown_duration: def.get("cooldown_duration").unwrap_or(1.0),
        })
    }
}

impl IntoSkillFeatureDefinition for CooldownFeature {
    fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition {
        let mut def = SkillFeatureDefinition::new(id);
        def.set("cooldown_duration", self.cooldown_duration);
        def
    }
}

// ---------------------------------------------------------------------------
// CooldownFeatureBuilder
// ---------------------------------------------------------------------------

/// Builder that attaches a [`CoolingTimer`] to the skill entity and sets its
/// parent to the target entity.
///
/// Reads `cooldown_duration` from the feature's numeric dictionary (default 1.0 s)
/// and inserts `CoolingTimer(Timer::from_seconds(cooldown_duration, TimerMode::Once))`
/// on `ctx.skill`, then parents the skill entity to `ctx.target`.
pub struct CooldownFeatureBuilder;

impl SkillFeatureBuilder for CooldownFeatureBuilder {
    fn build(
        &self,
        commands: &mut Commands,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError> {
        let cooldown_duration = ctx
            .feature
            .get("cooldown_duration")
            .unwrap_or(1.0);

        commands
            .entity(ctx.skill)
            .insert((
                CooldownFeature { cooldown_duration },
                CoolingTimer(Timer::from_seconds(
                    cooldown_duration,
                    TimerMode::Once,
                )),
            ))
            .set_parent_in_place(ctx.target);

        Ok(ctx.skill)
    }
}

// ---------------------------------------------------------------------------
// SkillFeatureResult & SkillRunContext
// ---------------------------------------------------------------------------

/// Reference to a skill entity, attached on the owner entity for easy access.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct SkillTarget(pub Entity);

/// Trait for checking whether a skill feature execution succeeded.
pub trait SkillFeatureResult: std::fmt::Debug + Send + Sync {
    /// Returns `true` if the execution was successful.
    fn is_ok(&self) -> bool;
}

/// Component that records the runtime context of a skill execution,
/// including the owning entity and the outcomes of each feature.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SkillRunContext {
    /// The entity that owns this skill.
    pub owner: Entity,
    /// Record of feature execution outcomes, keyed by feature id.
    /// Each value is a boxed trait object implementing [`SkillFeatureResult`].
    #[reflect(ignore)]
    pub feature_results: HashMap<String, Box<dyn SkillFeatureResult>>,
}

impl SkillRunContext {
    /// Create a new [`SkillRunContext`].
    pub fn new(owner: Entity) -> Self {
        Self {
            owner,
            feature_results: HashMap::new(),
        }
    }

    /// Record the result of a single feature execution.
    pub fn record_feature_result(
        &mut self,
        feature_id: impl Into<String>,
        result: impl SkillFeatureResult + 'static,
    ) {
        self.feature_results.insert(feature_id.into(), Box::new(result));
    }
}

/// Marker result indicating that a cooldown feature has completed.
#[derive(Debug)]
pub struct CooldownCompleted;

impl SkillFeatureResult for CooldownCompleted {
    fn is_ok(&self) -> bool {
        true
    }
}

/// System that ticks [`CoolingTimer`] on skill entities and, when the timer
/// finishes, records a [`CooldownCompleted`] result in the parent entity's
/// [`SkillRunContext`].
pub fn tick_cooldown_features(
    time: Res<Time>,
    mut skill_query: Query<(&mut CoolingTimer, &ChildOf), With<CooldownFeature>>,
    mut parent_query: Query<&mut SkillRunContext>,
) {
    for (mut timer, child_of) in &mut skill_query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Ok(mut ctx) = parent_query.get_mut(child_of.get()) {
                ctx.record_feature_result("cooldown", CooldownCompleted);
            }
        }
    }
}

// Traits
// ---------------------------------------------------------------------------

/// Build `Self` from a [`SkillFeatureDefinition`]'s numeric dictionary.
///
/// # Example
///
/// ```ignore
/// struct DamageFeature { base_damage: f32, ratio: f32 }
///
/// impl FromSkillFeatureDefinition for DamageFeature {
///     fn from_feature(def: &SkillFeatureDefinition) -> Option<Self> {
///         Some(Self {
///             base_damage: def.get("base_damage").unwrap_or(0.0),
///             ratio: def.get("ratio").unwrap_or(1.0),
///         })
///     }
/// }
/// ```
pub trait FromSkillFeatureDefinition: Sized {
    /// Construct `Self` from a [`SkillFeatureDefinition`].
    ///
    /// Returns `None` when a required parameter is missing.
    fn from_feature(definition: &SkillFeatureDefinition) -> Option<Self>;
}

/// Convert `Self` into a [`SkillFeatureDefinition`].
///
/// # Example
///
/// ```ignore
/// struct DamageFeature { base_damage: f32, ratio: f32 }
///
/// impl IntoSkillFeatureDefinition for DamageFeature {
///     fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition {
///         let mut def = SkillFeatureDefinition::new(id);
///         def.set("base_damage", self.base_damage);
///         def.set("ratio", self.ratio);
///         def
///     }
/// }
/// ```
pub trait IntoSkillFeatureDefinition {
    /// Convert `self` into a [`SkillFeatureDefinition`] with the given feature `id`.
    fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition;
}

// ---------------------------------------------------------------------------
// SkillFeatureBuilder
// ---------------------------------------------------------------------------

/// Context passed to [`SkillFeatureBuilder::build`].
///
/// Carries the feature definition to build from, the skill entity,
/// and the target (owner/caster) entity.
pub struct SkillFeatureBuilderContext {
    /// The [`SkillFeatureDefinition`] to build from.
    pub feature: SkillFeatureDefinition,
    /// The skill entity that is being executed.
    pub skill: Entity,
    /// The entity this skill belongs to (the owner/caster).
    pub target: Entity,
}

/// Error returned when building a skill feature entity fails.
#[derive(Debug)]
pub enum BuildError {
    /// No builder is registered for the given feature id.
    MissingBuilder(String),
    /// The builder encountered an error during construction.
    BuildFailed(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingBuilder(id) => write!(f, "no builder registered for feature '{id}'"),
            Self::BuildFailed(msg) => write!(f, "build failed: {msg}"),
        }
    }
}

impl std::error::Error for BuildError {}

/// Trait for building entities from a [`SkillFeatureDefinition`].
///
/// Implementors define how a specific skill feature (projectile, AoE, summon, etc.)
/// is spawned into the ECS world. The [`build`](SkillFeatureBuilder::build) method
/// receives `&mut Commands` and a [`SkillFeatureBuilderContext`].
pub trait SkillFeatureBuilder: Send + Sync {
    /// Build an entity from the feature definition and context.
    ///
    /// Returns the spawned entity's [`Entity`] ID on success.
    fn build(
        &self,
        commands: &mut Commands,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError>;
}

// ---------------------------------------------------------------------------
// SkillFeatureBuilderContainer (Resource)
// ---------------------------------------------------------------------------

/// A Bevy [`Resource`] that maps feature ids to [`SkillFeatureBuilder`] closures.
///
/// Use [`register`](SkillFeatureBuilderContainer::register) to register builders,
/// and [`build`](SkillFeatureBuilderContainer::build) to construct entities by
/// feature id at runtime.
#[derive(Resource)]
pub struct SkillFeatureBuilderContainer {
    builders: HashMap<
        String,
        Box<dyn Fn(&mut Commands, SkillFeatureBuilderContext) -> Result<Entity, BuildError> + Send + Sync>,
    >,
}

impl SkillFeatureBuilderContainer {
    /// Create an empty container.
    ///
    /// Builders must be registered via [`register`](SkillFeatureBuilderContainer::register)
    /// by individual skill systems (e.g. projectile plugin, AoE plugin).
    pub fn new() -> Self {
        let mut builders: HashMap<
            String,
            Box<dyn Fn(&mut Commands, SkillFeatureBuilderContext) -> Result<Entity, BuildError> + Send + Sync>,
        > = HashMap::new();

        // Register built-in default builders.
        builders.insert(
            "cooldown".to_string(),
            Box::new(|commands: &mut Commands, ctx: SkillFeatureBuilderContext| {
                CooldownFeatureBuilder.build(commands, ctx)
            }),
        );

        Self { builders }
    }

    /// Register a named builder from a [`SkillFeatureBuilder`] implementor.
    pub fn register(&mut self, id: impl Into<String>, builder: impl SkillFeatureBuilder + 'static) {
        let id = id.into();
        self.builders.insert(
            id,
            Box::new(move |commands: &mut Commands, ctx: SkillFeatureBuilderContext| {
                builder.build(commands, ctx)
            }),
        );
    }

    /// Look up a builder by feature id and execute it to spawn an entity.
    ///
    /// Returns `Err(BuildError::MissingBuilder)` if no builder is registered
    /// for `id`, or forwards errors from the builder itself.
    pub fn build(
        &self,
        id: &str,
        commands: &mut Commands,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError> {
        self.builders
            .get(id)
            .ok_or_else(|| BuildError::MissingBuilder(id.to_string()))
            .and_then(|f| f(commands, ctx))
    }

    /// Build all features from a [`SkillDefinition`] on the given entities.
    ///
    /// `skill_entity` is the skill child entity; `target` is the owner/parent entity.
    /// Each feature builder receives both.
    pub fn build_all(
        &self,
        definition: &SkillDefinition,
        skill_entity: Entity,
        target: Entity,
        commands: &mut Commands,
    ) {
        for feature in &definition.features {
            let ctx = SkillFeatureBuilderContext {
                feature: feature.clone(),
                skill: skill_entity,
                target,
            };
            if let Some(builder) = self.builders.get(&feature.id) {
                if let Err(e) = builder(commands, ctx) {
                    bevy::log::warn!("skill feature '{}' failed: {e}", feature.id);
                }
            }
        }
    }
}

impl Default for SkillFeatureBuilderContainer {
    fn default() -> Self {
        Self::new()
    }
}
