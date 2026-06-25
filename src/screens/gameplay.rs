//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy::ecs::schedule::SystemCondition;

use avian2d::prelude::LinearVelocity;

use crate::enemy::{Enemy, EnemySystems};
use crate::theme::widget;
use crate::shop::Shop;
use crate::{level::spawn_level, state::{Finish, InGame, Menu, Pause, Screen}};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), (spawn_level, set_ingame_preparation));

    app.configure_sets(
        Update,
        EnemySystems
            .run_if(in_gameplay_and_running()),
    );

    // When the game is settled, stop all enemies and show settlement UI.
    app.add_systems(OnEnter(Finish(true)), (stop_enemies, spawn_settlement_ui));

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause, set_ingame_none));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

/// Run condition that returns true when the game is in the [`Screen::Gameplay`]
/// state, not paused ([`Pause(false)`]), and not finished ([`Finish(false)`]).
pub fn in_gameplay_and_running() -> impl SystemCondition<()> {
    in_state(InGame::Battle)
        .and(in_state(Pause(false)))
        .and(in_state(Finish(false)))
}

/// Sets [`InGame`] to [`Preparation`] when entering gameplay.
fn set_ingame_preparation(mut next_ingame: ResMut<NextState<InGame>>) {
    next_ingame.set(InGame::Preparation);
}

/// Sets [`InGame`] to [`None`] when leaving gameplay.
fn set_ingame_none(mut next_ingame: ResMut<NextState<InGame>>) {
    next_ingame.set(InGame::None);
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

/// Stop all enemy movement when the game is settled.
fn stop_enemies(mut enemies: Query<&mut LinearVelocity, With<Enemy>>) {
    for mut velocity in &mut enemies {
        velocity.0 = Vec2::ZERO;
    }
}

/// Spawn the settlement UI when the game is finished.
fn spawn_settlement_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settlement UI"),
        GlobalZIndex(2),
        DespawnOnExit(Finish(true)),
        children![
            widget::header("Game Over"),
            widget::button("Back to Title", quit_to_title),
        ],
    ));
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>, mut next_shop: ResMut<NextState<Shop>>, mut next_finish: ResMut<NextState<Finish>>) {
    next_screen.set(Screen::Title);
    next_shop.set(Shop(false));
    next_finish.set(Finish(false));
}
