//! The game's main screen states and transitions between them.

mod gameplay;
pub(super) use gameplay::in_gameplay_and_unpaused;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;
use crate::state::Screen;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}
