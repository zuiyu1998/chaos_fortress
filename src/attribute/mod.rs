//! Attribute module.
//!
//! Defines the [`Attribute`] component, the [`AttributeModifier`] value object,
//! the [`AttributeSet`] component, and their helper methods.

use bevy::prelude::*;
use std::collections::HashMap;

/// Plugin that registers [`Attribute`] with Bevy's type registry.
pub(super) struct AttributePlugin;

impl Plugin for AttributePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attribute>();
        app.register_type::<AttributeSet>();
    }
}

/// A bounded numeric attribute with base, current, min, and max values, plus a
/// list of modifiers that can affect the current value.
///
/// Useful for representing entity stats such as health, attack power, or
/// defense. The [`value`](Attribute::value) field can be temporarily modified
/// via [`set_value`](Attribute::set_value) or automatically recalculated from
/// [`modifiers`](Attribute::modifiers) via [`recalculate`](Attribute::recalculate).
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Attribute {
    /// Base value (unaffected by temporary effects).
    pub base: f32,
    /// Current value (affected by temporary effects).
    pub value: f32,
    /// Maximum value ceiling.
    pub max: f32,
    /// Minimum value floor.
    pub min: f32,
    /// List of modifiers applied to this attribute.
    pub modifiers: Vec<AttributeModifier>,
}

impl Attribute {
    /// Creates a new attribute with `base` and `value` set to `base`,
    /// `max` set to [`f32::MAX`], `min` set to [`f32::MIN`], and an empty
    /// modifier list — meaning no bounds are enforced unless explicitly
    /// overridden.
    pub fn new(base: f32) -> Self {
        Self {
            base,
            value: base,
            max: f32::MAX,
            min: f32::MIN,
            modifiers: Vec::new(),
        }
    }

    /// Sets the current value, automatically clamping to `[min, max]`.
    pub fn set_value(&mut self, new_value: f32) {
        self.value = new_value.clamp(self.min, self.max);
    }

    /// Resets the current value to the base value.
    pub fn reset(&mut self) {
        self.value = self.base;
    }

    /// Adds a modifier to the list. If the `id` already exists the call is a
    /// no-op. After a successful addition [`recalculate`](Attribute::recalculate)
    /// is called automatically.
    pub fn add_modifier(&mut self, modifier: AttributeModifier) {
        if self.modifiers.iter().any(|m| m.id == modifier.id) {
            return;
        }
        self.modifiers.push(modifier);
        self.recalculate();
    }

    /// Removes the modifier with the given `id`. If a modifier was removed,
    /// [`recalculate`](Attribute::recalculate) is called automatically.
    pub fn remove_modifier(&mut self, id: &str) {
        let len_before = self.modifiers.len();
        self.modifiers.retain(|m| m.id != id);
        if self.modifiers.len() < len_before {
            self.recalculate();
        }
    }

    /// Recalculates `value` from `base` by applying all modifiers in order:
    ///
    /// 1. **Flat** — all `Flat` amounts are summed and added to `base`.
    /// 2. **Percent** — all `Percent` ratios are summed and applied
    ///    multiplicatively.
    /// 3. **Override** — if any `Override` modifier exists, the **last** one
    ///    wins and its value is used directly (ignoring Flat & Percent).
    pub fn recalculate(&mut self) {
        let mut value = self.base;

        // 1) Flat —— accumulate all flat modifiers
        for m in &self.modifiers {
            if let ModifierKind::Flat(amount) = &m.kind {
                value += amount;
            }
        }

        // 2) Percent —— sum ratios, apply multiplicatively
        let mut percent_sum = 0.0f32;
        for m in &self.modifiers {
            if let ModifierKind::Percent(ratio) = &m.kind {
                percent_sum += ratio;
            }
        }
        value *= 1.0 + percent_sum;

        // 3) Override —— last override wins
        for m in &self.modifiers {
            if let ModifierKind::Override(val) = &m.kind {
                value = *val;
            }
        }

        self.value = value;
    }
}

// ---------------------------------------------------------------------------
// AttributeModifier
// ---------------------------------------------------------------------------

/// A modifier value object used to apply numeric corrections to an
/// [`Attribute`] (e.g. from buffs, debuffs, equipment, or skills).
///
/// Each modifier carries a unique `id`, a `tag_id` for grouping / source
/// tracking, and a [`ModifierKind`] that determines how it affects the
/// attribute.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct AttributeModifier {
    /// Unique identifier for deduplication.
    pub id: String,
    /// Tag identifier for grouping or source marking (e.g. equipment, skill,
    /// buff).
    pub tag_id: String,
    /// The kind of modification this modifier applies.
    pub kind: ModifierKind,
}

/// The kind of modification an [`AttributeModifier`] applies.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum ModifierKind {
    /// Adds or subtracts a fixed amount directly.
    Flat(f32),
    /// Modifies the value by a percentage of the base (e.g. `0.1` = +10%).
    Percent(f32),
    /// Overrides the attribute value entirely, ignoring other modifiers.
    Override(f32),
}

// ---------------------------------------------------------------------------
// AttributeSet
// ---------------------------------------------------------------------------

/// A collection of named [`Attribute`]s, providing keyed access to manage all
/// attributes of an entity (e.g. health, attack power, defense) in one place.
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct AttributeSet {
    /// Map of attribute names (e.g. "hp", "attack", "defense") to [`Attribute`]s.
    pub attributes: HashMap<String, Attribute>,
}

impl AttributeSet {
    /// Creates an empty attribute set.
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    /// Inserts or replaces an attribute by name.
    pub fn insert(&mut self, name: &str, attribute: Attribute) {
        self.attributes.insert(name.to_string(), attribute);
    }

    /// Returns an immutable reference to the attribute with the given name.
    pub fn get(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }

    /// Returns a mutable reference to the attribute with the given name.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Attribute> {
        self.attributes.get_mut(name)
    }

    /// Removes the attribute with the given name, returning it if it existed.
    pub fn remove(&mut self, name: &str) -> Option<Attribute> {
        self.attributes.remove(name)
    }

    /// Returns an iterator over all attribute names.
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.attributes.keys()
    }

    /// Returns an iterator over all (name, attribute) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Attribute)> {
        self.attributes.iter()
    }
}

impl Default for AttributeSet {
    fn default() -> Self {
        Self::new()
    }
}
