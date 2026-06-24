//! Dropped items module.
//!
//! Defines [`DroppedItemDefinition`], a template definition for a single
//! dropped item (name, type, amount range, weight), and [`DroppedItemTemplate`],
//! a global resource/asset that acts as a registry of named drop definitions.
//!
//! # Design
//!
//! `DroppedItemTemplate` serves two roles:
//!
//! - **`Resource`** — created programmatically in a plugin and accessed at
//!   runtime via `Res<DroppedItemTemplate>`.
//! - **`Asset`** — loaded from `.dropped_item_template.csv` files via the
//!   Bevy `AssetServer` and the [`loader::DroppedItemTemplateLoader`].
//!
//! The [`DroppedItemPlugin`] handles registration of both the type system
//! (Reflect), the asset infrastructure, and a default `FromWorld` resource.

pub(super) mod loader;

use std::collections::HashMap;

use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::level::LevelState;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

/// Plugin that registers [`DroppedItemDefinition`], [`DroppedItemTemplate`],
/// [`GoldDroppedItem`], and [`loader::DroppedItemTemplateLoader`] with Bevy's
/// type registry, initialises [`DroppedItemTemplate`] as both a [`Resource`]
/// and an [`Asset`](bevy::asset::Asset), and registers the CSV asset loader.
pub(super) struct DroppedItemPlugin;

impl Plugin for DroppedItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DroppedItemDefinition>();
        app.register_type::<DroppedItemTemplate>();
        app.register_type::<GoldDroppedItem>();
        app.init_asset::<DroppedItemTemplate>();
        app.register_asset_loader(loader::DroppedItemTemplateLoader);
        app.load_resource::<DroppedItemTemplate>();
        app.add_systems(Update, collect_gold_drops);
    }
}

/// Collects all gold drop entities each frame, adding their gold amount to
/// [`LevelState::money`] and despawning the entity.
fn collect_gold_drops(
    mut level_state: ResMut<LevelState>,
    query: Query<(Entity, &GoldDroppedItem)>,
    mut commands: Commands,
) {
    for (entity, gold) in &query {
        level_state.money += gold.amount;
        commands.entity(entity).despawn();
    }
}

// ---------------------------------------------------------------------------
// DroppedItemDefinition
// ---------------------------------------------------------------------------

/// A template definition for a single dropped item.
///
/// Describes the name, type, amount range, and drop weight of one item
/// that can be rolled when an entity is killed or a drop event occurs.
///
/// [`DroppedItemTemplate`] stores a collection of these definitions keyed
/// by [`name`](DroppedItemDefinition::name).
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct DroppedItemDefinition {
    /// Item identifier (used as the key in [`DroppedItemTemplate`]).
    pub name: String,
    /// Item type string (e.g. `"gold"`, `"exp"`, `"material"`).
    pub item_type: String,
    /// Minimum drop amount (inclusive).
    pub min_amount: u32,
    /// Maximum drop amount (inclusive).
    pub max_amount: u32,
    /// Drop weight — higher values mean higher drop probability.
    pub weight: f32,
}

// ---------------------------------------------------------------------------
// GoldDroppedItem
// ---------------------------------------------------------------------------

/// A component that marks an entity as a pickable gold drop item.
///
/// When a character collides with this entity, the gold amount is added
/// to [`LevelState`](crate::level::LevelState) and the entity is despawned.
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct GoldDroppedItem {
    /// The amount of gold this drop contains.
    pub amount: u32,
}

/// Spawn a gold drop entity.
///
/// Returns a bundle with a [`Name`] and a [`GoldDroppedItem`] component.
/// The caller is responsible for adding the necessary visual, transform,
/// and collision components before spawning.
pub fn gold_drop(amount: u32) -> impl Bundle {
    (
        Name::new(format!("Gold ({amount})")),
        GoldDroppedItem { amount },
    )
}

// ---------------------------------------------------------------------------
// DroppedItemTemplate
// ---------------------------------------------------------------------------

/// Global dropped-item template resource/asset, keyed by item name.
///
/// Provides a declarative way to centrally define dropped item templates
/// (each being a [`DroppedItemDefinition`]), avoiding repeated hardcoded
/// values when constructing drop logic.
///
/// Can be loaded from CSV files via [`loader::DroppedItemTemplateLoader`], or
/// constructed programmatically and used as both a [`Resource`] and an
/// [`Asset`](bevy::asset::Asset).
#[derive(Resource, Asset, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct DroppedItemTemplate {
    /// Map of item names to their template definitions.
    pub definitions: HashMap<String, DroppedItemDefinition>,
}

impl DroppedItemTemplate {
    /// Creates an empty template.
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    /// Inserts (or replaces) a definition, keyed by its name.
    pub fn define(&mut self, def: DroppedItemDefinition) {
        self.definitions.insert(def.name.clone(), def);
    }

    /// Returns a reference to the definition with the given name, if any.
    pub fn get(&self, name: &str) -> Option<&DroppedItemDefinition> {
        self.definitions.get(name)
    }
}

impl FromWorld for DroppedItemTemplate {
    fn from_world(_world: &mut World) -> Self {
        let mut template = Self::new();
        template.define(DroppedItemDefinition {
            name: "gold".into(),
            item_type: "gold".into(),
            min_amount: 1,
            max_amount: 10,
            weight: 100.0,
        });
        template.define(DroppedItemDefinition {
            name: "exp".into(),
            item_type: "exp".into(),
            min_amount: 5,
            max_amount: 20,
            weight: 80.0,
        });
        template.define(DroppedItemDefinition {
            name: "wood".into(),
            item_type: "material".into(),
            min_amount: 1,
            max_amount: 3,
            weight: 40.0,
        });
        template
    }
}
