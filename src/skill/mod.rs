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
//! - **[`SkillPlugin`]** — Registers the asset type, component, and container with Bevy.

use bevy::prelude::*;
use std::collections::HashMap;

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
        app.init_resource::<SkillFeatureBuilderContainer>();
    }
}

// ---------------------------------------------------------------------------
// Factory function
// ---------------------------------------------------------------------------

/// Create a skill bundle from a [`SkillDefinition`].
///
/// Returns a bundle containing a [`SkillInstance`] and a [`Name`] component.
///
/// This is the recommended way to attach a skill to an entity at spawn time:
///
/// ```ignore
/// commands.spawn(skill(&my_skill_def));
/// ```
pub fn skill(definition: &SkillDefinition) -> impl Bundle {
    let instance = SkillInstance::new(&definition.id);
    (
        Name::new(format!("Skill ({})", definition.name)),
        instance,
    )
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
#[derive(Debug, Clone, PartialEq)]
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
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
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
        Box<dyn for<'w, 's> Fn(
            &'w mut Commands<'w, 's>,
            SkillFeatureBuilderContext,
        ) -> Result<Entity, BuildError> + Send + Sync>,
    >,
}

impl SkillFeatureBuilderContainer {
    /// Create an empty container.
    ///
    /// Builders must be registered via [`register`](SkillFeatureBuilderContainer::register)
    /// by individual skill systems (e.g. projectile plugin, AoE plugin).
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    /// Register a named builder from a [`SkillFeatureBuilder`] implementor.
    pub fn register(&mut self, id: impl Into<String>, builder: impl SkillFeatureBuilder + 'static) {
        let id = id.into();
        self.builders.insert(
            id,
            Box::new(move |commands, ctx| builder.build(commands, ctx)),
        );
    }

    /// Look up a builder by feature id and execute it to spawn an entity.
    ///
    /// Returns `Err(BuildError::MissingBuilder)` if no builder is registered
    /// for `id`, or forwards errors from the builder itself.
    pub fn build<'w, 's>(
        &self,
        id: &str,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError> {
        self.builders
            .get(id)
            .ok_or_else(|| BuildError::MissingBuilder(id.to_string()))
            .and_then(|f| f(commands, ctx))
    }
}

impl Default for SkillFeatureBuilderContainer {
    fn default() -> Self {
        Self::new()
    }
}
