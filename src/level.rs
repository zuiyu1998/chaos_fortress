//! Spawn the main level.

use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    common, enemy,
    enemy::spawner::{EnemySpawner, SpawnArea, SpawnEntry},
    map::{self, Map, MapData},
    shop::Shop,
    state::{InGame, Screen},
};
use bevy_lunex::prelude::*;
use rand::Rng;

use crate::attribute::{Attribute, AttributeSet, AttributeTemplate};
use crate::enemy::{assets, EnemyBuilderContainer, EnemyBuilderContext, EnemySystems};

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.load_resource::<LevelAssets>();
        app.init_resource::<LevelState>();
        app.register_type::<MoneyDisplay>();
        app.add_systems(Update, update_money_display);
        app.add_systems(Update, tick_enemy_spawner.in_set(EnemySystems));
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// 关卡运行时数据资源。
///
/// 存储当前关卡的状态数据，包括单位的金钱等。
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelState {
    /// 当前金钱数量。
    pub money: u32,
}

impl Default for LevelState {
    fn default() -> Self {
        Self { money: 100 }
    }
}

/// 标记金钱显示 UI 实体的组件。
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MoneyDisplay;

/// 更新金钱显示文本。
fn update_money_display(
    level_state: Res<LevelState>,
    mut money_text: Single<&mut Text2d, With<MoneyDisplay>>,
) {
    money_text.0 = format!("Gold: {}", level_state.money);
}

/// Spawn the level HUD: money display at top-left and action buttons at
/// bottom-right.
///
/// Spawns children under the given [`ChildSpawnerCommands`] builder.
pub fn level_ui(parent: &mut ChildSpawnerCommands) {
    // Money display (top-left)
    parent.spawn((
        Name::new("Money Display"),
        UiLayout::window()
            .pos((Ab(0.0), Ab(0.0)))
            .size((Ab(200.0), Ab(40.0)))
            .anchor(Anchor::TOP_LEFT)
            .pack(),
        UiTextSize::from(Ab(24.0)),
        Text2d::new("Gold: 100"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.84, 0.0)),
        MoneyDisplay,
    ));

    // Action buttons at bottom-right.
    parent
        .spawn((
            Name::new("HUD Buttons"),
            UiLayout::window()
                .pos((Rw(75.0), Rh(92.0)))
                .size((Ab(420.0), Ab(80.0)))
                .anchor(Anchor::TOP_LEFT)
                .pack(),
        ))
        .with_children(|buttons| {
            buttons.spawn((
                Name::new("Battle"),
                Sprite::from_color(Color::srgb(0.3, 0.5, 0.8), Vec2::new(200.0, 80.0)),
                UiLayout::window()
                    .pos((Ab(0.0), Ab(0.0)))
                    .size((Ab(200.0), Ab(80.0)))
                    .anchor(Anchor::TOP_LEFT)
                    .pack(),
                UiTextSize::from(Ab(24.0)),
                Text2d::new("Battle"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ))
            .observe(start_battle);

            buttons.spawn((
                Name::new("Shop"),
                Sprite::from_color(Color::srgb(0.3, 0.5, 0.8), Vec2::new(200.0, 80.0)),
                UiLayout::window()
                    .pos((Ab(210.0), Ab(0.0)))
                    .size((Ab(200.0), Ab(80.0)))
                    .anchor(Anchor::TOP_LEFT)
                    .pack(),
                UiTextSize::from(Ab(24.0)),
                Text2d::new("Shop"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ))
            .observe(open_shop);
        });
}

/// Placeholder action for the "Battle" button.
fn start_battle(
    trigger: On<Pointer<Click>>,
    mut next_ingame: ResMut<NextState<InGame>>,
    mut commands: Commands,
) {
    next_ingame.set(InGame::Battle);
    commands.entity(trigger.event_target()).insert(Visibility::Hidden);
    info!("Battle");
}

/// Action for the "Shop" button — toggles the shop UI open/closed.
fn open_shop(_: On<Pointer<Click>>, shop: Res<State<Shop>>, mut next_shop: ResMut<NextState<Shop>>) {
    let is_open = shop.get() == &Shop(true);
    next_shop.set(Shop(!is_open));
}

/// 驱动 [`EnemySpawner`] 生成敌人的 System (`tick_enemy_spawner`)。
///
/// 在 `Update` 阶段运行，每帧递减 `spawn_timer`；归零时：
///
/// 1. 在 [`SpawnArea`](SpawnArea) 内随机选取 `(col, row)`。
/// 2. 从 [`entries`](EnemySpawner::entries) 中随机选取一个
///    [`SpawnEntry`](SpawnEntry)。
/// 3. 为该类型构造 [`EnemyBuilderContext`]，通过
///    [`EnemyBuilderContainer`] 生成敌人实体。
/// 4. 将 `spawn_timer` 重置为该 entry 的 `interval`。
pub fn tick_enemy_spawner(
    time: Res<Time>,
    mut spawner: ResMut<EnemySpawner>,
    container: Res<EnemyBuilderContainer>,
    enemy_assets: Res<assets::EnemyAssets>,
    template_assets: Res<Assets<AttributeTemplate>>,
    mut commands: Commands,
    map_entity: Single<Entity, With<Map>>,
) {
    let dt = time.delta_secs();
    spawner.spawn_timer -= dt;

    if spawner.spawn_timer > 0.0 {
        return;
    }

    let area = &spawner.spawn_area;
    let mut rng = rand::rng();

    // 在固定区域内随机选取生成坐标
    let col = rng.random_range(area.col_min..=area.col_max);
    let row = rng.random_range(area.row_min..=area.row_max);

    // 随机选取一个 entry 生成敌人
    let entry = &spawner.entries[rng.random_range(0..spawner.entries.len())];

    // 构造基础属性集
    let attrs = template_assets
        .get(&enemy_assets.basic_attributes)
        .map(|t| t.build_attribute_set(&["hp", "max_hp", "armor", "attack"]))
        .unwrap_or_else(|| {
            let mut a = AttributeSet::new();
            a.insert("hp", Attribute::new(100.0));
            a.insert("max_hp", Attribute::new(100.0));
            a.insert("armor", Attribute::new(10.0));
            a.insert("attack", Attribute::new(10.0));
            a
        });

    let ctx = EnemyBuilderContext {
        position: (col, row),
        cell_size: 64.0,
        parent: Some(*map_entity),
        attributes: attrs,
    };

    let mut cmds = commands.reborrow();
    if let Err(e) = container.build(&entry.builder_name, &mut cmds, ctx) {
        error!(
            "EnemySpawner: failed to build '{}': {e}",
            entry.builder_name
        );
    }

    spawner.spawn_timer = entry.interval;
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    map_data: Res<MapData>,
    mut camera_query: Query<(Entity, &mut Transform), With<Camera2d>>,
) {
    let mut camera_entity = None;
    for (entity, mut transform) in &mut camera_query {
        transform.translation = Vec3::new(
            map_data.cell_size + (map_data.width as f32 - 1.0) * map_data.cell_size / 2.0,
            -((map_data.height as f32 - 1.0) * map_data.cell_size / 2.0),
            0.0,
        );
        camera_entity = Some(entity);
    }

    let level_entity = commands
        .spawn((
            Name::new("Level"),
            Transform::from_xyz(map_data.cell_size, 0.0, 0.0),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .id();

    commands.insert_resource(EnemySpawner {
        spawn_area: SpawnArea {
            col_min: 13,
            col_max: 15,
            row_min: 0,
            row_max: 4,
        },
        entries: vec![SpawnEntry {
            builder_name: "basic".into(),
            count: 10,
            interval: 0.6,
        }],
        spawn_timer: 1.0,
        ..default()
    });

    commands.entity(level_entity).with_children(|level| {
            map::map(level, &map_data);
            level.spawn(enemy::base(0, 0, map_data.cell_size, 2, 5));
            level.spawn((
                Name::new("Gameplay Music"),
                music(level_assets.music.clone()),
            ));
        });
    if let Some(camera) = camera_entity {
        commands.entity(camera).with_children(|ui_root| {
            ui_root
                .spawn((common::ui_root_2d(), DespawnOnExit(Screen::Gameplay)))
                .with_children(|ui| {
                    level_ui(ui);
                });
        });
    }
}
