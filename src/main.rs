mod game;
mod overlay;
mod plugins;
mod ui;

use bevy::prelude::*;
use plugins::CorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(overlay::window_plugin()))
        .add_plugins(overlay::OverlayPlugin)
        .add_plugins(CorePlugin)
        .run();
}
