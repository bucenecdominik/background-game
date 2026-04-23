//! Application plugin wiring.

use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_test_sprite));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_test_sprite(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.2, 0.8, 0.4),
            custom_size: Some(Vec2::new(120.0, 120.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
