//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    enemy,
    map::{self, MapData},
    role,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
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

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    map_data: Res<MapData>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    for mut transform in &mut camera_query {
        transform.translation = Vec3::new(640.0, -360.0, 0.0);
    }
    commands.spawn((
        Name::new("Level"),
        Transform::from_xyz(map_data.cell_size, 0.0, 0.0),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
    ))
    .with_children(|level| {
        map::map(level, &map_data);
        level.spawn(role::role(
            map_data.cell_size,
            0,
            9,
            Sprite::from_color(Color::BLACK, Vec2::splat(map_data.cell_size)),
        ));
        level.spawn(enemy::enemy(
            map_data.cell_size,
            4,
            2,
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(map_data.cell_size)),
        ));
        level.spawn((
            Name::new("Gameplay Music"),
            music(level_assets.music.clone()),
        ));
    });
}
