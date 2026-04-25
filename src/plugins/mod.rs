//! Application plugin wiring.

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use crate::game::GamePlugin;
use crate::ui::UiPlugin;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FrameTimeDiagnosticsPlugin, UiPlugin, GamePlugin))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_camera_creates_camera_with_black_clear_color() {
        let mut app = App::new();
        app.add_systems(Startup, spawn_camera);

        app.update();

        let mut cameras = app.world_mut().query::<&Camera>();
        let camera = cameras
            .iter(app.world())
            .next()
            .expect("expected startup system to spawn a camera");

        assert!(matches!(
            camera.clear_color,
            ClearColorConfig::Custom(color) if color == Color::BLACK
        ));
    }
}
