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
use crate::level::LevelState;

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
        app.init_state::<Shop>();
        app.insert_resource(ShopItems::default());

        app.add_systems(OnEnter(Shop(true)), spawn_shop_ui);
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
/// Reads the [`ShopItem`] component from the clicked entity, checks
/// whether the player has enough gold, deducts the cost on success,
/// and prints a confirmation message.
fn buy_archer(
    click: On<Pointer<Click>>,
    mut level_state: ResMut<LevelState>,
    query: Query<&ShopItem>,
) {
    let Ok(item) = query.get(click.event_target()) else {
        warn!("Archer button missing ShopItem component");
        return;
    };

    if level_state.money < item.price {
        warn!(
            "Not enough gold to purchase '{}' (cost: {}, gold: {})",
            item.name, item.price, level_state.money
        );
        return;
    }

    level_state.money -= item.price;
    info!("Purchased '{}' for {} gold", item.name, item.price);
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
