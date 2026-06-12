//! Battle module.
//!
//! Defines the [`BattleState`] component, which stores combat attributes
//! such as hit points and armor, and the [`DeathInBattle`] message which
//! is emitted when a battle entity dies.

use bevy::prelude::*;

/// Plugin that registers battle-related components and messages.
///
/// Registers [`BattleState`] and [`DeathInBattle`] with Bevy's type
/// registry and message system.
pub(super) struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BattleState>();
        app.add_message::<DeathInBattle>();
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

/// Create a battle entity with combat attributes.
///
/// Returns a [`BattleState`] initialized with the given hit points and armor.
pub fn battle(max_hp: f32, armor: f32) -> BattleState {
    BattleState::new(max_hp, armor)
}
