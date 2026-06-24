//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    common,
    map::{self, MapData},
    state::Screen,
};
use bevy_lunex::prelude::*;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.load_resource::<LevelAssets>();
        app.init_resource::<LevelState>();
        app.register_type::<MoneyDisplay>();
        app.add_systems(Update, update_money_display);
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
    commands
        .spawn((
            Name::new("Level"),
            Transform::from_xyz(map_data.cell_size, 0.0, 0.0),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|level| {
            map::map(level, &map_data);
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
                    ui.spawn((
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
                });
        });
    }
}
