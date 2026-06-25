//! Shop module.
//!
//! Defines [`ShopItem`], a definition for purchasable items in the in-game
//! shop, along with the [`Shop`] state (controls shop UI open/close)
//! and the [`ShopPlugin`] that wires everything together.
//!
//! # Overview
//!
//! The shop module provides the data for the in-game store.
//! Items are defined as [`ShopItem`] records stored
//! in a [`ShopItems`] resource. When the player clicks a buy button,
//! the observer reads the [`ShopItem`] component from the button entity,
//! validates the player has enough gold, deducts the cost, and prints
//! a confirmation.

use bevy::ecs::spawn::SpawnWith;
use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::common;
use crate::level::{Level, LevelState};
use crate::map::{BenchCell, MapCell, MapState};
use crate::role::assets as role_assets;
use crate::role::{RoleBuilderContainer, RoleBuilderContext};
use crate::attribute::{Attribute, AttributeSet, AttributeTemplate};
use crate::skill::{SkillDefinition, SkillEffectBuilderContainer, SkillFeatureBuilderContainer};

// ---------------------------------------------------------------------------
// Shop state
// ---------------------------------------------------------------------------

/// Whether or not the shop UI is open.
///
/// Controls whether the shop panel is visible and interactive.
/// Analogous to [`Pause`](crate::state::Pause) for shop state management.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Shop(pub bool);

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

/// Plugin that registers [`ShopItem`], [`Shop`] state,
/// and systems for the in-game shop.
///
/// # Registration
///
/// | Item | Kind | Description |
/// |---|---|---|
/// | [`Shop`] | State | Controls whether the shop UI is open (`Shop(true)`) or closed (`Shop(false)`) |
/// | [`ShopItem`] | Type | Purchasable item definition |
/// | [`ShopItems`] | Resource | Registry of available shop items |
pub(super) struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ShopItem>();
        app.register_type::<ShopItems>();
        app.register_type::<ShopPanel>();
        app.register_type::<RoleShopItem>();
        app.init_state::<Shop>();
        app.insert_resource(ShopItems::default());

        app.add_systems(OnEnter(Shop(true)), spawn_shop_ui);
        app.add_systems(Update, spawn_bench_roles);
    }
}

// ---------------------------------------------------------------------------
// ShopItem
// ---------------------------------------------------------------------------

/// A purchasable item in the in-game shop.
///
/// Describes the item's display information (name, description, price) and
/// behavior information (type, associated value). The [`item_type`] field
/// is a string that determines how the purchase is handled (e.g. `"role"`,
/// `"enhancement"`, `"consumable"`), and the [`value`] field carries the
/// type-specific payload (e.g. a builder name or effect identifier).
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct ShopItem {
    /// Unique identifier for the item (used to look up in the shop list).
    pub id: String,
    /// Display name shown in the shop UI.
    pub name: String,
    /// Description shown in the shop UI.
    pub description: String,
    /// Purchase price in gold.
    pub price: u32,
    /// Item type string that determines purchase behavior
    /// (e.g. `"role"`, `"enhancement"`, `"consumable"`).
    pub item_type: String,
    /// Associated value whose meaning depends on [`item_type`]
    /// (e.g. role builder name, effect definition ID).
    pub value: String,
}

// ---------------------------------------------------------------------------
// ShopItems
// ---------------------------------------------------------------------------

/// A Bevy [`Resource`] that holds a single purchasable role for the
/// current level.
///
/// # Usage
///
/// ```rust
/// app.insert_resource(ShopItems {
///     role: ShopItem {
///         id: "recruit_archer".into(),
///         name: "招募弓箭手".into(),
///         description: "在场上增加一名弓箭手".into(),
///         price: 50,
///         item_type: "role".into(),
///         value: "archer".into(),
///     },
/// });
/// ```
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct ShopItems {
    /// The purchasable role item offered in the shop.
    pub role: ShopItem,
}

impl ShopItems {
    /// Create a new `ShopItems` with a default role item.
    pub fn new() -> Self {
        Self {
            role: ShopItem {
                id: "recruit_archer".into(),
                name: "招募弓箭手".into(),
                description: "在场上增加一名弓箭手".into(),
                price: 50,
                item_type: "role".into(),
                value: "archer".into(),
            },
        }
    }
}

impl Default for ShopItems {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Shop UI
// ---------------------------------------------------------------------------

/// A marker component that identifies the root entity of the shop panel.
///
/// Systems can query this component to locate or manage the shop UI.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ShopPanel;

/// A component attached to a bench-zone cell entity to mark that a role
/// should be spawned on it.
///
/// Added by the purchase logic when the player buys a role from the shop.
/// The [`spawn_bench_roles`] system reads this component and calls
/// [`role::role()`] to spawn the actual role entity as a child of the cell.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct RoleShopItem {
    /// The role name registered in [`RoleBuilderContainer`]
    /// (e.g. `"archer"`).
    pub role_name: String,
    /// Purchase price in gold, deducted when the role is actually spawned.
    pub price: u32,
}

/// Spawn the shop panel UI root.
///
/// Returns a bundle suitable for spawning as a child of a bevy_lunex UI
/// root. The panel is positioned as a centered overlay with a title header.
/// Item cards are populated dynamically by a separate system reading
/// [`ShopItems`](crate::shop::ShopItems).
pub fn shop_ui(role: &ShopItem) -> impl Bundle {
    let archer_item = role.clone();
    (
        Name::new("Shop Panel"),
        common::UIRoot2d,
        ShopPanel,
        DespawnOnExit(Shop(true)),
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                Name::new("Archer Button"),
                Button,
                archer_item,
                UiLayout::window()
                    .pos((Ab(32.0), Ab(32.0)))
                    .size((Ab(200.0), Ab(60.0)))
                    .anchor(Anchor::TOP_LEFT)
                    .pack(),
                Sprite::from_color(Color::srgb(0.3, 0.5, 0.8), Vec2::new(200.0, 60.0)),
                UiTextSize::from(Ab(24.0)),
                Text2d::new("Archer"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ))
            .observe(buy_archer);

            parent.spawn((
                Name::new("Close Button"),
                Button,
                UiLayout::window()
                    .pos((Ab(0.0), Ab(70.0)))
                    .size((Ab(200.0), Ab(60.0)))
                    .anchor(Anchor::TOP_LEFT)
                    .pack(),
                Sprite::from_color(Color::srgb(0.8, 0.3, 0.3), Vec2::new(200.0, 60.0)),
                UiTextSize::from(Ab(24.0)),
                Text2d::new("Close"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ))
            .observe(close_shop);
        })),
    )
}

// ---------------------------------------------------------------------------
// Observers
// ---------------------------------------------------------------------------

/// Observer that handles a click on the archer purchase button.
///
/// Reads the [`ShopItem`] component from the clicked entity and spawns
/// a standalone [`RoleShopItem`] entity to queue role generation.
/// Gold is not deducted here — it is deducted in [`spawn_bench_roles`]
/// when the role is actually placed on a bench cell.
fn buy_archer(
    click: On<Pointer<Click>>,
    query: Query<&ShopItem>,
    mut commands: Commands,
) {
    let Ok(item) = query.get(click.event_target()) else {
        warn!("Archer button missing ShopItem component");
        return;
    };

    commands.spawn(RoleShopItem {
        role_name: item.value.clone(),
        price: item.price,
    });
    info!(
        "Purchase '{}' queued, waiting for empty bench slot",
        item.name
    );
}

/// Observer that closes the shop UI.
fn close_shop(_: On<Pointer<Click>>, mut next_shop: ResMut<NextState<Shop>>) {
    next_shop.set(Shop(false));
}

// ---------------------------------------------------------------------------
// Systems
// ---------------------------------------------------------------------------

/// System that spawns the shop UI panel when the shop opens.
///
/// Runs on [`OnEnter(Shop(true))`] and spawns the [`shop_ui`] bundle
/// as a child of the main camera entity, matching how the level UI
/// is parented.
fn spawn_shop_ui(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>,
    shop_items: Res<ShopItems>,
) {
    let Some(camera) = camera_query.iter().next() else {
        return;
    };
    commands.entity(camera).with_children(|parent| {
        parent.spawn(shop_ui(&shop_items.role));
    });
}

/// System that reads standalone [`RoleShopItem`] entities and spawns the
/// corresponding role on the first empty bench cell via [`MapState`].
///
/// Finds a free bench-zone cell from [`MapState`], locates the matching ECS
/// entity, spawns the role as a child, and updates [`MapState`] with the new
/// role entity. The `RoleShopItem` entity is despawned after spawning.
fn spawn_bench_roles(
    role_items: Query<(Entity, &RoleShopItem)>,
    mut level_state: ResMut<LevelState>,
    mut map_state: ResMut<MapState>,
    level_entity: Single<Entity, With<Level>>,
    cell_entities: Query<(Entity, &MapCell), (With<BenchCell>, Without<Children>)>,
    container: Res<RoleBuilderContainer>,
    role_assets: Res<role_assets::RoleAssets>,
    template_assets: Res<Assets<AttributeTemplate>>,
    skill_container: Res<SkillFeatureBuilderContainer>,
    skill_effect_container: Res<SkillEffectBuilderContainer>,
    skill_assets: Res<Assets<SkillDefinition>>,
    mut commands: Commands,
) {
    let level_entity = *level_entity;
    for (item_entity, item) in &role_items {
        // Find first bench-zone cell in MapState with no role
        let free_cell = map_state
            .cells
            .iter()
            .find(|(cell, data)| cell.x <= 1 && data.role.is_none())
            .map(|(cell, _)| *cell);

        let Some(cell_key) = free_cell else {
            continue;
        };

        // Find the actual entity matching this MapCell coordinate
        let Some((_slot_entity, _)) =
            cell_entities.iter().find(|(_, mc)| **mc == cell_key)
        else {
            continue;
        };

        // Check if player has enough gold before spawning
        if level_state.money < item.price {
            warn!(
                "Not enough gold to spawn role '{}' (cost: {}, gold: {})",
                item.role_name, item.price, level_state.money
            );
            commands.entity(item_entity).despawn();
            continue;
        }
        level_state.money -= item.price;

        // Despawn the RoleShopItem entity first
        commands.entity(item_entity).despawn();

        // Build attributes from template (same logic as role::role)
        let attrs = template_assets
            .get(&role_assets.archer_attributes)
            .map(|t| {
                t.build_attribute_set(&[
                    "hp", "max_hp", "armor", "attack", "defense",
                    "attack_speed", "attack_range",
                ])
            })
            .unwrap_or_else(|| {
                let mut a = AttributeSet::new();
                a.insert("hp", Attribute::new(100.0));
                a.insert("max_hp", Attribute::new(100.0));
                a.insert("armor", Attribute::new(10.0));
                a.insert("attack", Attribute::new(10.0));
                a.insert("defense", Attribute::new(5.0));
                a.insert("attack_speed", Attribute::new(1.0));
                a.insert("attack_range", Attribute::new(2.0));
                a
            });

        let skill = skill_assets
            .get(&role_assets.archer_skill)
            .expect("archer skill asset not loaded");

        let ctx = RoleBuilderContext {
            position: (cell_key.x, cell_key.y),
            parent: Some(level_entity),
            attributes: attrs,
            skill_container: &skill_container,
            skill_effect_container: &skill_effect_container,
            skill,
            skill_handle: role_assets.archer_skill.clone(),
        };

        let role_entity = container
            .build("archer", &mut commands, ctx)
            .expect("RoleBuilderContainer build failed for 'archer'");

        // Update MapState with the new role entity
        if let Some(data) = map_state.cells.get_mut(&cell_key) {
            data.role = Some(role_entity);
        }
    }
}
