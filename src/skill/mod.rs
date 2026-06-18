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
//! - **[`SkillInstance`]** — A [`Component`] that binds a skill asset handle to an entity.
//! - **[`SkillFeatureBuilderContext`]** — Context passed to [`SkillFeatureBuilder`].
//! - **[`BuildError`]** — Error type for builder operations.
//! - **[`SkillFeatureBuilder`]** — A trait for spawning entities from skill features.
//! - **[`SkillFeatureBuilderContainer`]** — A [`Resource`] that maps feature ids to builders.
//! - **[`CooldownFeatureBuilder`]** — A [`SkillFeatureBuilder`] that attaches a [`CoolingTimer`] to the skill entity.
//! - **[`SkillFeatureResult`]** — An enum for checking feature execution status (Ready, Ok, Error).
//! - **[`SkillEvent`]** — A message broadcast when a skill completes execution.
//! - **[`SkillTarget`]** — A [`Component`] referencing the skill entity, attached on the owner.
//! - **[`SkillPlugin`]** — Registers the asset type, component, and container with Bevy.

use bevy::prelude::*;
use std::collections::HashMap;

pub(super) mod cooldown;
pub(super) mod loader;

pub use cooldown::*;

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
        app.register_type::<CoolingTimer>();
        app.register_type::<SkillRunContext>();
        app.register_type::<SkillTarget>();
        app.add_message::<SkillEvent>();
        app.init_resource::<SkillFeatureBuilderContainer>();
        app.init_resource::<SkillEffectBuilderContainer>();
        app.register_asset_loader(loader::SkillDefinitionLoader);
    }
}

// ---------------------------------------------------------------------------
// Factory function
// ---------------------------------------------------------------------------

/// Spawn a skill entity as a child of `target` from a [`SkillDefinition`].
///
/// Returns the spawned child entity's [`Entity`] ID.
///
/// Creates a [`SkillInstance`] with the given `skill_handle` and a [`Name`] component
/// on a new child entity, then iterates over `definition.features` and `definition.effects`
/// and uses the respective builder containers to apply each feature/effect builder with
/// `skill` set to the child and `target` set to the parent.
///
/// ```ignore
/// let skill_entity = skill(&mut spawner, &feature_container, &effect_container, &my_skill_def, my_handle);
/// ```
pub fn skill(
    spawner: &mut ChildSpawnerCommands,
    feature_container: &SkillFeatureBuilderContainer,
    effect_container: &SkillEffectBuilderContainer,
    definition: &SkillDefinition,
    skill_handle: Handle<SkillDefinition>,
) -> Entity {
    let target = spawner.target_entity();
    let mut commands = spawner.commands();
    let skill_entity = commands
        .spawn((
            Name::new(format!("Skill ({})", definition.name)),
            SkillInstance {
                skill: skill_handle,
            },
            SkillRunContext::new(target, definition),
        ))
        .id();

    feature_container.build_all(definition, skill_entity, target, &mut commands);
    effect_container.build_all(definition, skill_entity, target, &mut commands);

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
/// a list of [`SkillFeatureDefinition`] entries that carry numeric parameters,
/// and a list of [`SkillEffectDefinition`] entries that describe runtime effects.
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
    /// List of effect definitions describing runtime skill effects
    /// (e.g. fire bullet, heal, AoE damage).
    pub effects: Vec<SkillEffectDefinition>,
}

impl SkillDefinition {
    /// Create a new [`SkillDefinition`] with the given `id` and `name`, and empty
    /// features and effects lists.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            features: Vec::new(),
            effects: Vec::new(),
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

    /// Find an effect by its effect id.
    ///
    /// Returns `None` when no effect matches.
    pub fn get_effect(&self, effect_id: &str) -> Option<&SkillEffectDefinition> {
        self.effects.iter().find(|e| e.id == effect_id)
    }

    /// Append a [`SkillEffectDefinition`] to the effects list.
    pub fn add_effect(&mut self, effect: SkillEffectDefinition) {
        self.effects.push(effect);
    }
}

// ---------------------------------------------------------------------------
// SkillInstance (Component)
// ---------------------------------------------------------------------------

/// Component that binds a skill asset handle to an entity.
///
/// Holds only a handle referencing a [`SkillDefinition`] asset.
/// Runtime state such as cooldowns is managed by separate components
/// (e.g. [`CoolingTimer`]).
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct SkillInstance {
    /// Handle to the [`SkillDefinition`] asset this instance refers to.
    pub skill: Handle<SkillDefinition>,
}

// ---------------------------------------------------------------------------
// SkillFeatureResult & SkillRunContext
// ---------------------------------------------------------------------------

/// Reference to a skill entity, attached on the owner entity for easy access.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct SkillTarget(pub Entity);

/// Message broadcast when a skill completes execution, carrying the
/// results of all skill features.
///
/// Derived from [`SkillRunContext`] when all features have finished
/// (either succeeded or failed).
#[derive(Message, Clone, TypePath)]
pub struct SkillEvent {
    /// The skill entity that completed execution.
    pub skill: Entity,
    /// The entity that owns this skill.
    pub owner: Entity,
    /// Record of feature execution outcomes, keyed by feature id.
    pub feature_results: HashMap<String, SkillFeatureResult>,
}

impl From<(Entity, SkillRunContext)> for SkillEvent {
    fn from((skill, ctx): (Entity, SkillRunContext)) -> Self {
        Self {
            skill,
            owner: ctx.owner,
            feature_results: ctx.feature_results,
        }
    }
}

/// Emits a [`SkillEvent`] when all feature results in a [`SkillRunContext`]
/// have completed successfully (all entries are `Ok`).
pub fn emit_skill_event(
    mut query: Query<(Entity, &mut SkillRunContext)>,
    mut messages: MessageWriter<SkillEvent>,
) {
    for (entity, mut ctx) in query.iter_mut() {
        if ctx.feature_results.values().all(|r| matches!(r, SkillFeatureResult::Ok(_))) {
            messages.write(SkillEvent {
                skill: entity,
                owner: ctx.owner,
                feature_results: ctx.feature_results.clone(),
            });
            ctx.feature_results.values_mut().for_each(|r| *r = SkillFeatureResult::Ready);
        }
    }
}

/// Enum representing the execution result of a skill feature.
///
/// - [`Ready`](SkillFeatureResult::Ready) — Still in progress, not yet completed.
/// - [`Ok`](SkillFeatureResult::Ok) — Completed successfully, carries structured data.
/// - [`Error`](SkillFeatureResult::Error) — Completed with failure, carries an error message.
#[derive(Debug)]
pub enum SkillFeatureResult {
    /// Feature execution is still in progress.
    Ready,
    /// Feature executed successfully, carries structured data.
    Ok(Box<dyn SkillFeatureResultData>),
    /// Feature execution failed.
    Error(String),
}

impl Clone for SkillFeatureResult {
    fn clone(&self) -> Self {
        match self {
            Self::Ready => Self::Ready,
            Self::Ok(data) => Self::Ok(data.clone_box()),
            Self::Error(msg) => Self::Error(msg.clone()),
        }
    }
}

/// Component that records the runtime context of a skill execution,
/// including the owning entity and the outcomes of each feature.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SkillRunContext {
    /// The entity that owns this skill.
    pub owner: Entity,
    /// Record of feature execution outcomes, keyed by feature id.
    /// Each value is a [`SkillFeatureResult`] enum.
    #[reflect(ignore)]
    pub feature_results: HashMap<String, SkillFeatureResult>,
}

impl SkillRunContext {
    /// Create a new [`SkillRunContext`] with default [`SkillFeatureResult::Ready`]
    /// for each feature defined in the given skill definition.
    pub fn new(owner: Entity, definition: &SkillDefinition) -> Self {
        Self {
            owner,
            feature_results: definition
                .features
                .iter()
                .map(|f| (f.id.clone(), SkillFeatureResult::Ready))
                .collect(),
        }
    }

    /// Record the result of a single feature execution.
    pub fn record_feature_result(
        &mut self,
        feature_id: impl Into<String>,
        result: SkillFeatureResult,
    ) {
        self.feature_results.insert(feature_id.into(), result);
    }
}

// ---------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------

/// Marker trait for structured data stored in [`SkillFeatureResult::Ok`].
///
/// Each [`SkillFeature`](crate::skill::SkillFeature) can implement this trait
/// on its own result data type, allowing the success variant to carry
/// arbitrary structured information instead of a plain string.
pub trait SkillFeatureResultData: std::fmt::Debug + Send + Sync {
    /// Clone self into a boxed trait object.
    fn clone_box(&self) -> Box<dyn SkillFeatureResultData>;
}

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

// ---------------------------------------------------------------------------
// SkillEffectDefinition
// ---------------------------------------------------------------------------

/// A skill effect definition, embedded inside [`SkillDefinition`] as part of
/// its `effects` list.
///
/// Each effect carries a unique `id` and a `params` dictionary of numeric
/// values. Effects describe *what* the skill does (fire a bullet, heal, apply
/// an AoE), while [`SkillFeatureDefinition`] describes the *numeric dimensions*
/// of those actions.
///
/// This is a plain struct — it is **not** a Bevy [`Component`] or [`Asset`].
#[derive(Debug, Clone, PartialEq)]
pub struct SkillEffectDefinition {
    /// Unique identifier for this effect (e.g. "fire_bullet", "heal", "aoe_damage").
    pub id: String,
    /// Numeric parameter dictionary; keys are parameter names, values are f32.
    pub params: HashMap<String, f32>,
}

impl SkillEffectDefinition {
    /// Create a new [`SkillEffectDefinition`] with the given `id` and an empty dictionary.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            params: HashMap::new(),
        }
    }

    /// Get the value for `key`, or `None` if absent.
    pub fn get(&self, key: &str) -> Option<f32> {
        self.params.get(key).copied()
    }

    /// Set the value for `key`.
    pub fn set(&mut self, key: impl Into<String>, value: f32) {
        self.params.insert(key.into(), value);
    }
}

// ---------------------------------------------------------------------------
// SkillEffectBuilder
// ---------------------------------------------------------------------------

/// Context passed to [`SkillEffectBuilder::build`].
///
/// Carries the effect definition to build from, the skill entity,
/// and the target (owner/caster) entity.
pub struct SkillEffectBuilderContext {
    /// The [`SkillEffectDefinition`] to build from.
    pub effect: SkillEffectDefinition,
    /// The skill entity that is being executed.
    pub skill: Entity,
    /// The entity this skill belongs to (the owner/caster).
    pub target: Entity,
}

/// Trait for building entities from a [`SkillEffectDefinition`].
///
/// Implementors define how a specific skill effect (fire bullet, heal,
/// AoE damage, etc.) is spawned into the ECS world. The
/// [`build`](SkillEffectBuilder::build) method receives `&mut Commands`
/// and a [`SkillEffectBuilderContext`].
///
/// See also [`SkillFeatureBuilder`] for the counterpart that works with
/// [`SkillFeatureDefinition`].
pub trait SkillEffectBuilder: Send + Sync {
    /// Build an entity from the effect definition and context.
    ///
    /// Returns the spawned entity's [`Entity`] ID on success.
    fn build(
        &self,
        commands: &mut Commands,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError>;
}

// ---------------------------------------------------------------------------
// SkillEffectBuilderContainer (Resource)
// ---------------------------------------------------------------------------

/// A Bevy [`Resource`] that maps effect ids to [`SkillEffectBuilder`] closures.
///
/// Use [`register`](SkillEffectBuilderContainer::register) to register builders,
/// and [`build`](SkillEffectBuilderContainer::build) to construct entities by
/// effect id at runtime.
///
/// Unlike [`SkillFeatureBuilderContainer`], this container starts empty —
/// no builders are registered by default. Each effect system must register
/// its own builder during plugin setup.
#[derive(Resource)]
pub struct SkillEffectBuilderContainer {
    builders: HashMap<
        String,
        Box<dyn Fn(&mut Commands, SkillEffectBuilderContext) -> Result<Entity, BuildError> + Send + Sync>,
    >,
}

impl SkillEffectBuilderContainer {
    /// Create an empty container.
    ///
    /// Builders must be registered via [`register`](SkillEffectBuilderContainer::register)
    /// by individual effect systems (e.g. fire-bullet plugin, heal plugin).
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    /// Register a named builder from a [`SkillEffectBuilder`] implementor.
    pub fn register(&mut self, id: impl Into<String>, builder: impl SkillEffectBuilder + 'static) {
        let id = id.into();
        self.builders.insert(
            id,
            Box::new(move |commands: &mut Commands, ctx: SkillEffectBuilderContext| {
                builder.build(commands, ctx)
            }),
        );
    }

    /// Look up a builder by effect id and execute it to spawn an entity.
    ///
    /// Returns `Err(BuildError::MissingBuilder)` if no builder is registered
    /// for `id`, or forwards errors from the builder itself.
    pub fn build(
        &self,
        id: &str,
        commands: &mut Commands,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError> {
        self.builders
            .get(id)
            .ok_or_else(|| BuildError::MissingBuilder(id.to_string()))
            .and_then(|f| f(commands, ctx))
    }

    /// Build all effects from a [`SkillDefinition`] on the given entities.
    ///
    /// `skill_entity` is the skill child entity; `target` is the owner/parent entity.
    /// Each effect builder receives both.
    pub fn build_all(
        &self,
        definition: &SkillDefinition,
        skill_entity: Entity,
        target: Entity,
        commands: &mut Commands,
    ) {
        for effect in &definition.effects {
            let ctx = SkillEffectBuilderContext {
                effect: effect.clone(),
                skill: skill_entity,
                target,
            };
            if let Some(builder) = self.builders.get(&effect.id) {
                if let Err(e) = builder(commands, ctx) {
                    bevy::log::warn!("skill effect '{}' failed: {e}", effect.id);
                }
            }
        }
    }
}

impl Default for SkillEffectBuilderContainer {
    fn default() -> Self {
        Self::new()
    }
}
