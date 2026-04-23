//! Game domain module.

mod enemies;

use bevy::prelude::*;

use crate::ui::{GameMode, UiGameState};
use enemies::EnemiesPlugin;

pub const MAP_RADIUS: f32 = 3200.0;
pub const PLAYER_COLLISION_RADIUS: f32 = 48.0;
const BACKGROUND_PADDING: f32 = 640.0;
const BACKGROUND_Z: f32 = -20.0;
const STAR_Z: f32 = -19.0;
const STAR_COUNT: usize = 420;
const ARCADE_BACKGROUND_COLOR: Color = Color::BLACK;
const MAP_BORDER_COLOR: Color = Color::srgba(0.96, 0.36, 0.34, 0.72);
const PLAYER_SIZE: f32 = 110.0;
const PLAYER_ASPECT_RATIO: f32 = 915.0 / 1437.0;
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
        app.init_resource::<WorldMapState>()
            .add_plugins(EnemiesPlugin)
            .add_systems(Startup, (spawn_arcade_background, spawn_player))
            .add_systems(
                Update,
                (
                    move_player,
                    follow_player_camera,
                    refresh_arcade_background,
                    draw_map_border,
                ),
            );
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct WorldMapState {
    pub radius: f32,
}

impl Default for WorldMapState {
    fn default() -> Self {
        Self { radius: MAP_RADIUS }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component, Default)]
struct DashCooldown(f32);

#[derive(Component)]
struct ArcadeBackground;

fn spawn_arcade_background(mut commands: Commands) {
    let background_size = Vec2::splat((MAP_RADIUS + BACKGROUND_PADDING) * 2.0);

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
                    custom_size: Some(background_size),
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
            random.range(-MAP_RADIUS, MAP_RADIUS),
            random.range(-MAP_RADIUS, MAP_RADIUS),
            STAR_Z + random.range(0.0, 0.6),
        );

        if position.truncate().length() > MAP_RADIUS + BACKGROUND_PADDING * 0.75 {
            continue;
        }

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
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut DashCooldown), With<Player>>,
) {
    let Ok((mut transform, mut velocity, mut dash_cooldown)) = player_query.get_single_mut() else {
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
    clamp_player_to_map(&mut transform, &mut velocity);
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

fn clamp_player_to_map(transform: &mut Transform, velocity: &mut Velocity) {
    let position = transform.translation.truncate();
    let allowed_radius = MAP_RADIUS - PLAYER_COLLISION_RADIUS;
    let distance = position.length();

    if distance <= allowed_radius || distance == 0.0 {
        return;
    }

    let normal = position / distance;
    let clamped_position = normal * allowed_radius;
    transform.translation.x = clamped_position.x;
    transform.translation.y = clamped_position.y;

    let outward_speed = velocity.0.truncate().dot(normal);
    if outward_speed > 0.0 {
        velocity.0.x -= normal.x * outward_speed;
        velocity.0.y -= normal.y * outward_speed;
    }
}

fn follow_player_camera(
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for mut camera_transform in &mut cameras {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

fn draw_map_border(
    state: Res<UiGameState>,
    mut gizmos: Gizmos,
) {
    if state.selected_mode != GameMode::Arcade {
        return;
    }

    gizmos.circle_2d(Vec2::ZERO, MAP_RADIUS, MAP_BORDER_COLOR);
}
