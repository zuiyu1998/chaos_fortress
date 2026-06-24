//! Game state definitions.
//!
//! Defines the [`Screen`], [`Menu`], [`Pause`], [`InGame`], and [`Finish`]
//! states along with the [`PausableSystems`] system set that controls which
//! systems should not run while the game is paused.

use bevy::prelude::*;

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}

/// The game's menu states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);

/// The in-game phase states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum InGame {
    /// Not in gameplay.
    #[default]
    None,
    /// Preparing for battle.
    Preparation,
}

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

/// Whether or not the game has finished (settled).
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Finish(pub bool);

/// Plugin that registers [`Pause`], [`Finish`], [`InGame`], and configures
/// [`PausableSystems`] to only run when not paused.
pub(super) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Pause>();
        app.init_state::<Finish>();
        app.init_state::<InGame>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}
