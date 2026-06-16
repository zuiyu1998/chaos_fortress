//! Battle module.
//!
//! Defines the [`BattleState`] component, which stores combat attributes
//! such as hit points and armor, and the [`DeathInBattle`] message which
//! is emitted when a battle entity dies.

use bevy::prelude::*;
use crate::attribute::{Attribute, AttributeSet};


/// Plugin that registers battle-related components, messages, and systems.
///
/// Registers [`BattleState`] and [`DeathInBattle`] with Bevy's type
/// registry and message system, and adds the [`despawn_on_death`]
/// system.
pub(super) struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BattleState>();
        app.register_type::<BattleAttributeSet>();
        app.add_message::<DeathInBattle>();

        app.add_systems(Update, despawn_on_death);
    }
}

/// Message emitted when a battle entity dies (hp reaches 0).
///
/// Carries the entity that has died. Systems can read this message
/// to trigger death-related logic (e.g. play death animation, remove
/// entity, drop loot).
#[derive(Message, Clone, TypePath)]
pub struct DeathInBattle {
    /// The entity that died.
    pub entity: Entity,
}

/// System that despawns the dead entity whenever a [`DeathInBattle`]
/// message is received.
pub fn despawn_on_death(
    mut events: MessageReader<DeathInBattle>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands.entity(event.entity).despawn();
    }
}

/// Combat attributes for a battle entity.
///
/// Stores hit points, armor, and other combat-related data used by
/// the battle system for damage calculation and target evaluation.
#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct BattleState {
    /// Current hit points.
    pub hp: f32,
    /// Maximum hit points.
    pub max_hp: f32,
    /// Armor value that reduces incoming damage.
    pub armor: f32,
}

impl BattleState {
    /// Create a new [`BattleState`] with full HP.
    pub fn new(max_hp: f32, armor: f32) -> Self {
        Self {
            hp: max_hp,
            max_hp,
            armor,
        }
    }

    /// Create a [`BattleState`] from a [`BattleAttributeSet`], extracting the
    /// current `hp`, `max_hp`, and `armor` values.
    pub fn from_attribute_set(set: &BattleAttributeSet) -> Self {
        Self {
            hp: set.hp(),
            max_hp: set.max_hp(),
            armor: set.armor(),
        }
    }

    /// Returns `true` if the entity is dead (hp ≤ 0).
    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }

    /// Apply raw damage after armor reduction.
    ///
    /// Damage is reduced by `armor` (minimum 0) and subtracted from `hp`.
    pub fn take_damage(&mut self, raw_damage: f32) {
        let effective = (raw_damage - self.armor).max(0.0);
        self.hp = (self.hp - effective).max(0.0);
    }
}

/// Create a battle entity bundle from an [`AttributeSet`].
///
/// Returns a bundle containing a [`BattleState`] (derived from the attribute values)
/// and a [`BattleAttributeSet`] wrapping the given attribute set for modifier support.
pub fn battle(attributes: AttributeSet) -> impl Bundle {
    let battle_set = BattleAttributeSet { attributes };
    let state = BattleState::from_attribute_set(&battle_set);
    (state, battle_set)
}

// ---------------------------------------------------------------------------
// BattleAttributeSet
// ---------------------------------------------------------------------------

/// Wrapper around [`AttributeSet`] that provides a battle-oriented interface
/// for hit points, max hit points, and armor.
///
/// All combat attributes are stored as named [`Attribute`]s ("hp", "max_hp",
/// "armor") inside an [`AttributeSet`] and fully support [`AttributeModifier`]
/// effects (Flat, Percent, Override).
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct BattleAttributeSet {
    /// The underlying attribute set.
    pub attributes: AttributeSet,
}

impl BattleAttributeSet {
    /// Creates a new [`BattleAttributeSet`] with the given max HP and armor.
    /// HP starts at max HP.
    pub fn new(max_hp: f32, armor: f32) -> Self {
        let mut set = AttributeSet::new();
        set.insert("hp", Attribute::new(max_hp));
        set.insert("max_hp", Attribute::new(max_hp));
        set.insert("armor", Attribute::new(armor));
        Self { attributes: set }
    }

    /// Current hit points.
    pub fn hp(&self) -> f32 {
        self.attributes
            .get("hp")
            .map(|a| a.value)
            .unwrap_or(0.0)
    }

    /// Maximum hit points.
    pub fn max_hp(&self) -> f32 {
        self.attributes
            .get("max_hp")
            .map(|a| a.value)
            .unwrap_or(0.0)
    }

    /// Armor value.
    pub fn armor(&self) -> f32 {
        self.attributes
            .get("armor")
            .map(|a| a.value)
            .unwrap_or(0.0)
    }

    /// Returns `true` if the entity is dead (hp ≤ 0).
    pub fn is_dead(&self) -> bool {
        self.hp() <= 0.0
    }

    /// Applies raw damage after armor reduction.
    ///
    /// `effective_damage = max(raw_damage - armor, 0)`.
    pub fn take_damage(&mut self, raw_damage: f32) {
        let effective = (raw_damage - self.armor()).max(0.0);
        if let Some(hp) = self.attributes.get_mut("hp") {
            hp.set_value(hp.value - effective);
        }
    }

    /// Returns a mutable reference to the underlying [`AttributeSet`] for
    /// direct attribute or modifier manipulation.
    pub fn attributes_mut(&mut self) -> &mut AttributeSet {
        &mut self.attributes
    }
}

impl Default for BattleAttributeSet {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
