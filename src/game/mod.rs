//! Game domain module.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_SIZE: f32 = 170.0;
const PLAYER_HALF_SIZE: f32 = PLAYER_SIZE / 2.0;
const PLAYER_ACCELERATION: f32 = 360.0;
const PLAYER_DECELERATION: f32 = 260.0;
const PLAYER_MAX_SPEED: f32 = 420.0;
const PLAYER_ROTATION_SPEED: f32 = std::f32::consts::PI;
const PLAYER_SPRITE: &str = "sprites/fighter.png";

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Velocity(Vec3);

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Velocity::default(),
        SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            ..default()
        },
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let Ok((mut transform, mut velocity)) = player_query.get_single_mut() else {
        return;
    };

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let delta_seconds = time.delta_seconds();
    let rotation_direction = rotation_direction(&keyboard_input);
    let movement_direction = movement_direction(&keyboard_input);

    transform.rotate_z(rotation_direction * PLAYER_ROTATION_SPEED * delta_seconds);
    accelerate_player(
        &mut velocity,
        transform.rotation * Vec3::Y,
        movement_direction,
        delta_seconds,
    );

    transform.translation += velocity.0 * delta_seconds;
    clamp_player_to_window(&mut transform, &mut velocity, window);
}

fn accelerate_player(
    velocity: &mut Velocity,
    forward: Vec3,
    movement_direction: f32,
    delta_seconds: f32,
) {
    if movement_direction != 0.0 {
        velocity.0 += forward * movement_direction * PLAYER_ACCELERATION * delta_seconds;
        velocity.0 = velocity.0.clamp_length_max(PLAYER_MAX_SPEED);
        return;
    }

    let speed = velocity.0.length();
    if speed == 0.0 {
        return;
    }

    let deceleration = PLAYER_DECELERATION * delta_seconds;
    velocity.0 = velocity.0.normalize() * (speed - deceleration).max(0.0);
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

fn clamp_player_to_window(transform: &mut Transform, velocity: &mut Velocity, window: &Window) {
    let (_, _, rotation_z) = transform.rotation.to_euler(EulerRot::XYZ);
    let (sin, cos) = rotation_z.sin_cos();
    let rotated_extent = PLAYER_HALF_SIZE * (sin.abs() + cos.abs());

    let horizontal_limit = (window.resolution.width() / 2.0) - rotated_extent;
    let vertical_limit = (window.resolution.height() / 2.0) - rotated_extent;

    let clamped_x = transform
        .translation
        .x
        .clamp(-horizontal_limit, horizontal_limit);
    let clamped_y = transform
        .translation
        .y
        .clamp(-vertical_limit, vertical_limit);

    if clamped_x != transform.translation.x {
        velocity.0.x = 0.0;
    }

    if clamped_y != transform.translation.y {
        velocity.0.y = 0.0;
    }

    transform.translation.x = clamped_x;
    transform.translation.y = clamped_y;
}
