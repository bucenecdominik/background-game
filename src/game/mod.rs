//! Game domain module.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::ui::{GameMode, UiGameState};

const BACKGROUND_SIZE: Vec2 = Vec2::new(4096.0, 4096.0);
const BACKGROUND_Z: f32 = -20.0;
const STAR_Z: f32 = -19.0;
const STAR_COUNT: usize = 260;
const ARCADE_BACKGROUND_COLOR: Color = Color::BLACK;
const PLAYER_SIZE: f32 = 110.0;
const PLAYER_ASPECT_RATIO: f32 = 915.0 / 1437.0;
const PLAYER_HALF_SIZE: f32 = PLAYER_SIZE / 2.0;
const PLAYER_ACCELERATION: f32 = 560.0;
const PLAYER_DECELERATION: f32 = 340.0;
const PLAYER_LATERAL_DECELERATION: f32 = 720.0;
const PLAYER_MAX_SPEED: f32 = 620.0;
const PLAYER_ROTATION_SPEED: f32 = std::f32::consts::PI;
const PLAYER_DASH_SPEED: f32 = 980.0;
const PLAYER_DASH_DECELERATION: f32 = 760.0;
const PLAYER_DASH_COOLDOWN_SECONDS: f32 = 0.55;
const PLAYER_SPRITE: &str = "sprites/ally-jet.png";

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_arcade_background, spawn_player))
            .add_systems(Update, (move_player, refresh_arcade_background));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component, Default)]
struct DashCooldown(f32);

#[derive(Component)]
struct ArcadeBackground;

fn spawn_arcade_background(mut commands: Commands) {
    commands
        .spawn((
            ArcadeBackground,
            SpatialBundle {
                visibility: Visibility::Visible,
                ..default()
            },
        ))
        .with_children(|background| {
            background.spawn(SpriteBundle {
                sprite: Sprite {
                    color: ARCADE_BACKGROUND_COLOR,
                    custom_size: Some(BACKGROUND_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
                ..default()
            });

            spawn_starfield(background);
        });
}

fn spawn_starfield(parent: &mut ChildBuilder) {
    let mut random = StarRandom::new(0x5EED_5ACE);

    for _ in 0..STAR_COUNT {
        let position = Vec3::new(
            random.range(-BACKGROUND_SIZE.x / 2.0, BACKGROUND_SIZE.x / 2.0),
            random.range(-BACKGROUND_SIZE.y / 2.0, BACKGROUND_SIZE.y / 2.0),
            STAR_Z + random.range(0.0, 0.6),
        );
        let size = random.range(1.0, 3.2);
        let brightness = random.range(0.45, 0.95);

        parent.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.82, 0.92, 1.0, brightness),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        });
    }
}

struct StarRandom(u32);

impl StarRandom {
    fn new(seed: u32) -> Self {
        Self(seed)
    }

    fn range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_unit()
    }

    fn next_unit(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.0 as f32 / u32::MAX as f32
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Velocity::default(),
        DashCooldown::default(),
        SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE * PLAYER_ASPECT_RATIO, PLAYER_SIZE)),
                ..default()
            },
            ..default()
        },
    ));
}

fn refresh_arcade_background(
    state: Res<UiGameState>,
    mut backgrounds: Query<&mut Visibility, With<ArcadeBackground>>,
) {
    if !state.is_changed() {
        return;
    }

    let visibility = if state.selected_mode == GameMode::Arcade {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut background_visibility in &mut backgrounds {
        *background_visibility = visibility;
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut DashCooldown), With<Player>>,
) {
    let Ok((mut transform, mut velocity, mut dash_cooldown)) = player_query.get_single_mut() else {
        return;
    };

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let delta_seconds = time.delta_seconds();
    let rotation_direction = rotation_direction(&keyboard_input);
    let movement_direction = movement_direction(&keyboard_input);

    transform.rotate_z(rotation_direction * PLAYER_ROTATION_SPEED * delta_seconds);
    let forward = transform.rotation * Vec3::Y;

    tick_dash_cooldown(&mut dash_cooldown, delta_seconds);
    if keyboard_input.just_pressed(KeyCode::KeyC) && dash_cooldown.0 == 0.0 {
        dash_player(&mut velocity, forward, &mut dash_cooldown);
    } else {
        accelerate_player(&mut velocity, forward, movement_direction, delta_seconds);
    }

    transform.translation += velocity.0 * delta_seconds;
    clamp_player_to_window(&mut transform, &mut velocity, window);
}

fn tick_dash_cooldown(dash_cooldown: &mut DashCooldown, delta_seconds: f32) {
    dash_cooldown.0 = (dash_cooldown.0 - delta_seconds).max(0.0);
}

fn dash_player(velocity: &mut Velocity, forward: Vec3, dash_cooldown: &mut DashCooldown) {
    velocity.0 = forward * PLAYER_DASH_SPEED;
    dash_cooldown.0 = PLAYER_DASH_COOLDOWN_SECONDS;
}

fn accelerate_player(
    velocity: &mut Velocity,
    forward: Vec3,
    movement_direction: f32,
    delta_seconds: f32,
) {
    let was_above_normal_max_speed = velocity.0.length() > PLAYER_MAX_SPEED;

    if movement_direction != 0.0 {
        velocity.0 += forward * movement_direction * PLAYER_ACCELERATION * delta_seconds;
    }

    damp_lateral_velocity(velocity, forward, delta_seconds);
    limit_player_speed(velocity, was_above_normal_max_speed);
    decelerate_dash_speed(velocity, delta_seconds);

    if movement_direction != 0.0 {
        return;
    }

    decelerate_velocity(velocity, delta_seconds);
}

fn damp_lateral_velocity(velocity: &mut Velocity, forward: Vec3, delta_seconds: f32) {
    let forward_velocity = forward * velocity.0.dot(forward);
    let lateral_velocity = velocity.0 - forward_velocity;
    let damped_lateral = move_toward_zero(
        lateral_velocity,
        PLAYER_LATERAL_DECELERATION * delta_seconds,
    );

    velocity.0 = forward_velocity + damped_lateral;
}

fn limit_player_speed(velocity: &mut Velocity, was_above_normal_max_speed: bool) {
    let speed_limit = if was_above_normal_max_speed {
        PLAYER_DASH_SPEED
    } else {
        PLAYER_MAX_SPEED
    };

    velocity.0 = velocity.0.clamp_length_max(speed_limit);
}

fn decelerate_dash_speed(velocity: &mut Velocity, delta_seconds: f32) {
    let speed = velocity.0.length();
    if speed <= PLAYER_MAX_SPEED {
        return;
    }

    let deceleration = PLAYER_DASH_DECELERATION * delta_seconds;
    let speed = (speed - deceleration).max(PLAYER_MAX_SPEED);
    velocity.0 = velocity.0.normalize() * speed;
}

fn decelerate_velocity(velocity: &mut Velocity, delta_seconds: f32) {
    let speed = velocity.0.length();
    if speed == 0.0 {
        return;
    }

    velocity.0 = move_toward_zero(velocity.0, PLAYER_DECELERATION * delta_seconds);
}

fn move_toward_zero(vector: Vec3, amount: f32) -> Vec3 {
    let length = vector.length();
    if length <= amount {
        return Vec3::ZERO;
    }

    vector.normalize() * (length - amount)
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
