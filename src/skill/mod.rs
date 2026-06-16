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
//! - **[`SkillPlugin`]** — Registers the asset type and component with Bevy.

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
    }
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
/// Each instance tracks its own cooldown timer, charges, and status.
/// It references a [`SkillDefinition`] via `skill_id`.
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct SkillInstance {
    /// The [`SkillDefinition`] ID this instance refers to.
    pub skill_id: String,
    /// Remaining cooldown in seconds; the skill is usable when this is `≤ 0.0`.
    pub cooldown_timer: f32,
    /// Total cooldown duration in seconds — the value `cooldown_timer` resets to after use.
    pub cooldown_seconds: f32,
    /// Current charge count. At least 1 charge is required to use the skill.
    pub charges: u32,
    /// Maximum charge count. `0` means no charge system (cooldown only).
    pub max_charges: u32,
    /// Current runtime status.
    pub status: SkillStatus,
}

impl SkillInstance {
    /// Create a new [`SkillInstance`] with the given `skill_id` and `cooldown_seconds`.
    ///
    /// The instance starts in [`SkillStatus::Ready`] with zero charges (no charge system).
    pub fn new(skill_id: impl Into<String>, cooldown_seconds: f32) -> Self {
        Self {
            skill_id: skill_id.into(),
            cooldown_timer: 0.0,
            cooldown_seconds,
            charges: 0,
            max_charges: 0,
            status: SkillStatus::Ready,
        }
    }

    /// Returns `true` when the skill can be used:
    /// - status is `Ready`
    /// - cooldown timer has expired
    /// - has at least one charge (when a charge system is active)
    pub fn ready(&self) -> bool {
        if self.status != SkillStatus::Ready || self.cooldown_timer > 0.0 {
            return false;
        }
        if self.max_charges > 0 && self.charges == 0 {
            return false;
        }
        true
    }

    /// Use the skill: consume one charge and reset the cooldown.
    ///
    /// Panics if the skill is not [`ready`](Self::ready).
    pub fn use_skill(&mut self) {
        assert!(self.ready(), "use_skill called when skill is not ready");
        if self.max_charges > 0 {
            self.charges = self.charges.saturating_sub(1);
        }
        self.cooldown_timer = self.cooldown_seconds;
        self.status = SkillStatus::Cooling;
    }

    /// Advance the cooldown timer by `delta` seconds.
    ///
    /// When the timer reaches zero the status becomes [`SkillStatus::Ready`] again,
    /// provided there is at least one charge (or no charge system is active).
    pub fn tick(&mut self, delta: f32) {
        if self.cooldown_timer > 0.0 {
            self.cooldown_timer = (self.cooldown_timer - delta).max(0.0);
        }
        if self.cooldown_timer <= 0.0 && self.status != SkillStatus::Disabled {
            let has_charge = self.max_charges == 0 || self.charges > 0;
            if has_charge {
                self.status = SkillStatus::Ready;
            }
        }
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
