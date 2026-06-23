//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    attribute::AttributeTemplate,
    audio::music,
    common,
    enemy,
    map::{self, MapData},
    role,
    skill::{SkillDefinition, SkillEffectBuilderContainer, SkillFeatureBuilderContainer},
    state::Screen,
};

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.load_resource::<LevelAssets>();
        app.init_resource::<LevelState>();
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

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    map_data: Res<MapData>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    role_container: Res<role::RoleBuilderContainer>,
    role_assets: Res<role::assets::RoleAssets>,
    template_assets: Res<Assets<AttributeTemplate>>,
    skill_container: Res<SkillFeatureBuilderContainer>,
    skill_effect_container: Res<SkillEffectBuilderContainer>,
    skill_assets: Res<Assets<SkillDefinition>>,
    enemy_container: Res<enemy::EnemyBuilderContainer>,
    enemy_assets: Res<enemy::assets::EnemyAssets>,
) {
    for mut transform in &mut camera_query {
        transform.translation = Vec3::new(
            map_data.cell_size + (map_data.width as f32 - 1.0) * map_data.cell_size / 2.0,
            -((map_data.height as f32 - 1.0) * map_data.cell_size / 2.0),
            0.0,
        );
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
            role::role(level, &role_container, 2, 4, &role_assets, &template_assets, &skill_container, &skill_effect_container, &skill_assets);
            level.spawn(enemy::base(0, 0, map_data.cell_size, 2, 5));
            for row in 0..map_data.height {
                enemy::enemy(level, &enemy_container, 10, row, map_data.cell_size, &enemy_assets, &template_assets);
            }
            level.spawn((
                Name::new("Gameplay Music"),
                music(level_assets.music.clone()),
            ));
            level.spawn(common::ui_root_2d());
        });
}
