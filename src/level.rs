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
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
    ))
    .with_children(|level| {
        map::map(level, &map_data);
        level.spawn(role::role(
            &map_data,
            3,
            5,
            Sprite::from_color(Color::BLACK, Vec2::splat(map_data.cell_size)),
        ));
        level.spawn(enemy::enemy(
            &map_data,
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
