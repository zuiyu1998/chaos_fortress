// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
pub mod attribute;
mod audio;
pub mod battle;
pub mod bullet;
pub mod common;
pub mod dropped_items;
mod enemy;
mod level;
mod map;
pub mod role;
pub mod skill;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod screens;
mod state;
mod theme;

use avian2d::PhysicsPlugins;
use avian2d::prelude::Gravity;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_gearbox::GearboxPlugin;
use bevy_lunex::{UiLunexPlugin, UiSourceCamera};
use bevy_tweening::TweeningPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Chaos Fortress".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            GearboxPlugin,
            UiLunexPlugin,
            PhysicsPlugins::default(),
            TweeningPlugin,
            asset_tracking::plugin,
            attribute::AttributePlugin,
            audio::plugin,
            bullet::BulletPlugin,
            skill::SkillPlugin,
            battle::BattlePlugin,
            common::CommonPlugin,
        ));
        app.add_plugins((
            dropped_items::DroppedItemPlugin,
            enemy::EnemyPlugin,
            level::LevelPlugin,
            map::plugin,
            role::RolePlugin,
        ));
        app.add_plugins((
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            state::StatePlugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Set gravity to zero so entities only move when given explicit velocity.
        app.insert_resource(Gravity::ZERO);

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        UiSourceCamera::<0>,
    ));
}
