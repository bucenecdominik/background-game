mod game;
mod overlay;
mod plugins;
mod ui;

use bevy::prelude::*;
use plugins::CorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .run();
}
