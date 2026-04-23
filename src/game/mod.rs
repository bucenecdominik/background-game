//! Game domain module.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_SIZE: f32 = 120.0;
const PLAYER_HALF_SIZE: f32 = PLAYER_SIZE / 2.0;
const PLAYER_MOVE_SPEED: f32 = 180.0;
const PLAYER_ROTATION_SPEED: f32 = std::f32::consts::PI;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((Player, SpatialBundle::default()))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.8, 0.4),
                    custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                    ..default()
                },
                ..default()
            });

            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.95, 0.95, 0.2),
                    custom_size: Some(Vec2::new(18.0, 36.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 42.0, 1.0),
                ..default()
            });
        });
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else {
        return;
    };

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let delta_seconds = time.delta_seconds();
    let rotation_direction = rotation_direction(&keyboard_input);
    let movement_direction = movement_direction(&keyboard_input);

    transform.rotate_z(rotation_direction * PLAYER_ROTATION_SPEED * delta_seconds);

    if movement_direction != 0.0 {
        let forward = transform.rotation * Vec3::Y;
        transform.translation += forward * movement_direction * PLAYER_MOVE_SPEED * delta_seconds;
    }

    clamp_player_to_window(&mut transform, window);
}

fn rotation_direction(keyboard_input: &ButtonInput<KeyCode>) -> f32 {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction -= 1.0;
    }

    direction
}

fn movement_direction(keyboard_input: &ButtonInput<KeyCode>) -> f32 {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= 1.0;
    }

    direction
}

fn clamp_player_to_window(transform: &mut Transform, window: &Window) {
    let (_, _, rotation_z) = transform.rotation.to_euler(EulerRot::XYZ);
    let (sin, cos) = rotation_z.sin_cos();
    let rotated_extent = PLAYER_HALF_SIZE * (sin.abs() + cos.abs());

    let horizontal_limit = (window.resolution.width() / 2.0) - rotated_extent;
    let vertical_limit = (window.resolution.height() / 2.0) - rotated_extent;

    transform.translation.x = transform
        .translation
        .x
        .clamp(-horizontal_limit, horizontal_limit);
    transform.translation.y = transform
        .translation
        .y
        .clamp(-vertical_limit, vertical_limit);
}
