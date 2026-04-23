use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_log_system);
    }
}

fn startup_log_system() {
    info!("Core plugin initialized.");
}
