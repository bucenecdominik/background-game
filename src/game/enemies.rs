//! Enemy system with the first real Drone Swarm enemy.

use bevy::prelude::*;

use super::{
    player_contact_damage_fraction, player_contact_invulnerability_seconds, ContactCooldown,
    Health, Player, Projectile, PLAYER_COLLISION_RADIUS,
};
use crate::ui::{GameMode, UiGameState};

const ENEMY_Z: f32 = 2.0;
const ENEMY_HEALTH_BAR_Z: f32 = 0.1;
const ENEMY_HEALTH_BAR_WIDTH: f32 = 34.0;
const ENEMY_HEALTH_BAR_HEIGHT: f32 = 5.0;
const ENEMY_HEALTH_BAR_Y_OFFSET: f32 = -30.0;
const DRONE_SWARM_MIN_COUNT: usize = 3;
const DRONE_SWARM_MAX_COUNT: usize = 6;
const DRONE_SWARM_BASE_SPEED: f32 = 175.0;
const DRONE_SWARM_MAX_SPEED: f32 = 255.0;
const DRONE_SWARM_ACCELERATION: f32 = 380.0;
const DRONE_SWARM_SEPARATION_RADIUS: f32 = 104.0;
const DRONE_SWARM_NEIGHBOR_RADIUS: f32 = 240.0;
const DRONE_SWARM_COHESION_WEIGHT: f32 = 0.22;
const DRONE_SWARM_ALIGNMENT_WEIGHT: f32 = 0.32;
const DRONE_SWARM_SEPARATION_WEIGHT: f32 = 2.8;
const DRONE_SWARM_SEEK_WEIGHT: f32 = 0.72;
const DRONE_SWARM_SPAWN_DISTANCE_MIN: f32 = 820.0;
const DRONE_SWARM_SPAWN_DISTANCE_MAX: f32 = 1220.0;
const DRONE_SWARM_MEMBER_SPACING: f32 = 92.0;
const DRONE_SWARM_ROTATING_SPIN_SPEED: f32 = -3.6;
const DRONE_SWARM_HARD_SEPARATION_DISTANCE: f32 = 48.0;
const DRONE_SWARM_LEFT_SPRITE: &str = "sprites/drone-swarm-left.png";
const DRONE_SWARM_CENTER_SPRITE: &str = "sprites/drone-swarm-center.png";
const DRONE_SWARM_RIGHT_SPRITE: &str = "sprites/drone-swarm-right.png";

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyWaveState>()
            .init_resource::<EnemyRandom>()
            .init_resource::<DroneSwarmAssets>()
            .add_systems(
                Update,
                (
                    reset_enemy_state,
                    spawn_drone_swarm_wave,
                    refresh_enemy_visibility,
                    move_drone_swarm,
                    sync_drone_swarm_visuals.after(move_drone_swarm),
                    handle_projectile_enemy_collisions,
                    handle_enemy_player_collisions,
                    despawn_destroyed_enemies,
                    update_enemy_health_bars,
                ),
            );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Enemy {
    pub kind: EnemyKind,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct EnemyStats {
    pub speed: f32,
    pub contact_damage: f32,
    pub collision_radius: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct DroneSwarmAgent {
    pub variant: DroneSwarmVariant,
    pub wave_id: u32,
    pub spin_angle: f32,
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SwarmVelocity(pub Vec2);

#[derive(Component, Debug, Clone, Copy)]
struct DroneSwarmVisual {
    base_rotation: f32,
    spin_speed: f32,
}

#[derive(Component)]
struct DroneSwarmSprite;

#[derive(Component)]
struct EnemyHealthBarRoot;

#[derive(Component)]
struct EnemyHealthBarFill;

#[derive(Resource, Default)]
struct EnemyWaveState {
    spawned: bool,
    next_wave_id: u32,
}

#[derive(Resource)]
struct EnemyRandom(u32);

impl Default for EnemyRandom {
    fn default() -> Self {
        Self(0xC0DE_DA7A)
    }
}

impl EnemyRandom {
    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.0
    }

    fn range_usize(&mut self, min: usize, max_inclusive: usize) -> usize {
        min + (self.next_u32() as usize % (max_inclusive - min + 1))
    }

    fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * (self.next_u32() as f32 / u32::MAX as f32)
    }
}

#[derive(Resource)]
struct DroneSwarmAssets {
    left: Handle<Image>,
    center: Handle<Image>,
    right: Handle<Image>,
}

impl FromWorld for DroneSwarmAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            left: asset_server.load(DRONE_SWARM_LEFT_SPRITE),
            center: asset_server.load(DRONE_SWARM_CENTER_SPRITE),
            right: asset_server.load(DRONE_SWARM_RIGHT_SPRITE),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyKind {
    DroneSwarm,
    KamikazeDrone,
    ShieldCarrier,
    TurretDrone,
    PhaseJumper,
    GravityDrone,
    Splitter,
    FlameChaser,
    SniperEye,
    OverlordCore,
}

impl EnemyKind {
    pub fn spec(self) -> EnemySpec {
        match self {
            Self::DroneSwarm => EnemySpec {
                display_name: "Drone Swarm",
                role: "Roj malych dronu leticich na hrace v koordinovane formaci.",
                health: 28.0,
                speed: DRONE_SWARM_BASE_SPEED,
                contact_damage: 10.0,
                size: Vec2::new(30.0, 30.0),
                color: Color::WHITE,
            },
            Self::KamikazeDrone => EnemySpec {
                display_name: "Kamikaze Drone",
                role: "Rychly utocnik navrzeny pro primy naraz.",
                health: 24.0,
                speed: 360.0,
                contact_damage: 24.0,
                size: Vec2::new(28.0, 34.0),
                color: Color::srgb(1.0, 0.28, 0.18),
            },
            Self::ShieldCarrier => EnemySpec {
                display_name: "Shield Carrier",
                role: "Podpurny nepritel, ktery bude pozdeji chranit okolni jednotky.",
                health: 90.0,
                speed: 130.0,
                contact_damage: 8.0,
                size: Vec2::new(48.0, 48.0),
                color: Color::srgb(0.2, 0.55, 1.0),
            },
            Self::TurretDrone => EnemySpec {
                display_name: "Turret Drone",
                role: "Stacionarni nebo pomaly strelec pro kontrolu zony.",
                health: 58.0,
                speed: 65.0,
                contact_damage: 10.0,
                size: Vec2::new(42.0, 42.0),
                color: Color::srgb(0.86, 0.68, 0.28),
            },
            Self::PhaseJumper => EnemySpec {
                display_name: "Phase Jumper",
                role: "Nepravidelny nepritel pro budouci teleportacni pohyb.",
                health: 38.0,
                speed: 210.0,
                contact_damage: 14.0,
                size: Vec2::new(34.0, 34.0),
                color: Color::srgb(0.72, 0.42, 1.0),
            },
            Self::GravityDrone => EnemySpec {
                display_name: "Gravity Drone",
                role: "Kontrolni jednotka pro budouci tahani nebo zpomalovani hrace.",
                health: 64.0,
                speed: 120.0,
                contact_damage: 9.0,
                size: Vec2::new(46.0, 38.0),
                color: Color::srgb(0.22, 1.0, 0.58),
            },
            Self::Splitter => EnemySpec {
                display_name: "Splitter",
                role: "Nepritel, ktery se pozdeji rozdeli na mensi casti.",
                health: 46.0,
                speed: 180.0,
                contact_damage: 12.0,
                size: Vec2::new(40.0, 30.0),
                color: Color::srgb(1.0, 0.65, 0.22),
            },
            Self::FlameChaser => EnemySpec {
                display_name: "Flame Chaser",
                role: "Agresivni pronasledovatel pro tlak z blizkosti.",
                health: 52.0,
                speed: 280.0,
                contact_damage: 18.0,
                size: Vec2::new(36.0, 44.0),
                color: Color::srgb(1.0, 0.38, 0.05),
            },
            Self::SniperEye => EnemySpec {
                display_name: "Sniper Eye",
                role: "Presny strelec pro budouci dalekonosne utoky.",
                health: 32.0,
                speed: 95.0,
                contact_damage: 7.0,
                size: Vec2::new(38.0, 38.0),
                color: Color::srgb(1.0, 0.15, 0.55),
            },
            Self::OverlordCore => EnemySpec {
                display_name: "Overlord Core",
                role: "Elitni jadro/boss pro budouci vlny a prikazy ostatnim.",
                health: 220.0,
                speed: 80.0,
                contact_damage: 30.0,
                size: Vec2::new(72.0, 72.0),
                color: Color::srgb(0.95, 0.95, 1.0),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DroneSwarmVariant {
    Left,
    Center,
    Right,
}

impl DroneSwarmVariant {
    fn random(random: &mut EnemyRandom) -> Self {
        match random.range_usize(0, 2) {
            0 => Self::Left,
            1 => Self::Center,
            _ => Self::Right,
        }
    }

    fn texture(self, assets: &DroneSwarmAssets) -> Handle<Image> {
        match self {
            Self::Left => assets.left.clone(),
            Self::Center => assets.center.clone(),
            Self::Right => assets.right.clone(),
        }
    }

    fn custom_size(self, base_size: Vec2) -> Vec2 {
        match self {
            Self::Left => base_size * 1.02,
            Self::Center => Vec2::new(base_size.x, base_size.y * 1.13),
            Self::Right => base_size * 1.18,
        }
    }

    fn spin_speed(self) -> f32 {
        match self {
            Self::Right => DRONE_SWARM_ROTATING_SPIN_SPEED,
            Self::Left | Self::Center => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnemySpec {
    pub display_name: &'static str,
    pub role: &'static str,
    pub health: f32,
    pub speed: f32,
    pub contact_damage: f32,
    pub size: Vec2,
    pub color: Color,
}

fn reset_enemy_state(
    mut commands: Commands,
    state: Res<UiGameState>,
    mut wave_state: ResMut<EnemyWaveState>,
    enemies: Query<Entity, With<Enemy>>,
) {
    let should_reset = state.is_changed()
        && (!state.is_defeated || state.selected_mode != GameMode::Arcade)
        && !(state.selected_mode == GameMode::Arcade && state.is_running);

    if !should_reset {
        return;
    }

    wave_state.spawned = false;

    for entity in &enemies {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_drone_swarm_wave(
    mut commands: Commands,
    state: Res<UiGameState>,
    player_query: Query<&Transform, With<Player>>,
    assets: Res<DroneSwarmAssets>,
    mut wave_state: ResMut<EnemyWaveState>,
    mut random: ResMut<EnemyRandom>,
) {
    if wave_state.spawned || state.selected_mode != GameMode::Arcade || !state.is_running {
        return;
    }

    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let count = random.range_usize(DRONE_SWARM_MIN_COUNT, DRONE_SWARM_MAX_COUNT);
    let wave_id = wave_state.next_wave_id;
    wave_state.next_wave_id += 1;

    let player_position = player_transform.translation.truncate();
    let spawn_center = random_spawn_center(&mut random, player_position);
    let tangent = tangent_for_spawn(spawn_center - player_position).normalize_or_zero();
    let spec = EnemyKind::DroneSwarm.spec();
    let collision_radius = spec.size.max_element() * 0.72;

    for index in 0..count {
        let slot_offset = index as f32 - (count.saturating_sub(1) as f32 / 2.0);
        let along_line = tangent * slot_offset * DRONE_SWARM_MEMBER_SPACING;
        let inward_jitter = (-spawn_center.normalize_or_zero()) * random.range_f32(0.0, 24.0);
        let lateral_jitter = tangent * random.range_f32(-18.0, 18.0);
        let position = spawn_center + along_line + inward_jitter + lateral_jitter;
        let variant = DroneSwarmVariant::random(&mut random);
        let facing = (player_position - position).normalize_or_zero();
        let base_rotation = velocity_to_angle(facing);

        commands
            .spawn((
                Enemy {
                    kind: EnemyKind::DroneSwarm,
                },
                Health::full(spec.health),
                EnemyStats {
                    speed: spec.speed,
                    contact_damage: spec.contact_damage,
                    collision_radius,
                },
                DroneSwarmAgent {
                    variant,
                    wave_id,
                    spin_angle: 0.0,
                },
                DroneSwarmVisual {
                    base_rotation,
                    spin_speed: variant.spin_speed(),
                },
                SwarmVelocity(facing * spec.speed),
                Name::new(format!(
                    "{} {} - {}",
                    spec.display_name,
                    index + 1,
                    spec.role
                )),
                SpatialBundle {
                    transform: Transform::from_xyz(position.x, position.y, ENEMY_Z),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    DroneSwarmSprite,
                    SpriteBundle {
                        texture: variant.texture(&assets),
                        sprite: Sprite {
                            color: spec.color,
                            custom_size: Some(variant.custom_size(spec.size)),
                            ..default()
                        },
                        transform: Transform::default()
                            .with_rotation(Quat::from_rotation_z(base_rotation)),
                        ..default()
                    },
                ));
                spawn_enemy_health_bar(parent);
            });
    }

    wave_state.spawned = true;
}

fn spawn_enemy_health_bar(parent: &mut ChildBuilder) {
    parent
        .spawn((
            EnemyHealthBarRoot,
            SpatialBundle {
                transform: Transform::from_xyz(0.0, ENEMY_HEALTH_BAR_Y_OFFSET, ENEMY_HEALTH_BAR_Z),
                ..default()
            },
        ))
        .with_children(|bar_root| {
            bar_root.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.05, 0.07, 0.11, 0.92),
                    custom_size: Some(Vec2::new(
                        ENEMY_HEALTH_BAR_WIDTH + 2.0,
                        ENEMY_HEALTH_BAR_HEIGHT + 2.0,
                    )),
                    ..default()
                },
                ..default()
            });

            bar_root.spawn((
                EnemyHealthBarFill,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.28, 0.96, 0.54, 0.94),
                        custom_size: Some(Vec2::new(ENEMY_HEALTH_BAR_WIDTH, ENEMY_HEALTH_BAR_HEIGHT)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.01),
                    ..default()
                },
            ));
        });
}

fn move_drone_swarm(
    time: Res<Time>,
    state: Res<UiGameState>,
    player_query: Query<&Transform, With<Player>>,
    mut swarm_queries: ParamSet<(
        Query<
            (Entity, &Transform, &SwarmVelocity, &DroneSwarmAgent),
            (With<Enemy>, Without<Player>),
        >,
        Query<
            (
                Entity,
                &mut Transform,
                &Children,
                &mut SwarmVelocity,
                &mut DroneSwarmAgent,
                &DroneSwarmVisual,
                &EnemyStats,
                &Health,
            ),
            (With<Enemy>, Without<Player>),
        >,
    )>,
) {
    if state.selected_mode != GameMode::Arcade || !state.is_running {
        return;
    }

    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let snapshots: Vec<_> = swarm_queries
        .p0()
        .iter()
        .map(|(entity, transform, velocity, agent)| {
            (
                entity,
                transform.translation.truncate(),
                velocity.0,
                agent.wave_id,
            )
        })
        .collect();

    let delta_seconds = time.delta_seconds();
    let player_position = player_transform.translation.truncate();

    for (entity, mut transform, children, mut velocity, mut agent, visual, stats, health) in
        &mut swarm_queries.p1()
    {
        let Some((_, current_position, current_velocity, wave_id)) = snapshots
            .iter()
            .copied()
            .find(|(other_entity, _, _, _)| *other_entity == entity)
        else {
            continue;
        };

        let mut center_sum = Vec2::ZERO;
        let mut heading_sum = Vec2::ZERO;
        let mut neighbor_count = 0.0;
        let mut separation = Vec2::ZERO;

        for (other_entity, other_position, other_velocity, other_wave_id) in &snapshots {
            if *other_entity == entity || *other_wave_id != wave_id {
                continue;
            }

            let personal_space = match agent.variant {
                DroneSwarmVariant::Left => 0.94,
                DroneSwarmVariant::Center => 1.0,
                DroneSwarmVariant::Right => 1.12,
            };
            let offset = current_position - *other_position;
            let distance = offset.length();

            if distance < DRONE_SWARM_NEIGHBOR_RADIUS {
                center_sum += *other_position;
                heading_sum += *other_velocity;
                neighbor_count += 1.0;
            }

            let separation_radius = DRONE_SWARM_SEPARATION_RADIUS * personal_space;
            if distance > 0.0 && distance < separation_radius {
                let closeness = (separation_radius - distance) / separation_radius;
                let hard_push = if distance < DRONE_SWARM_HARD_SEPARATION_DISTANCE {
                    1.7
                } else {
                    1.0
                };
                separation += offset.normalize() * closeness * closeness * hard_push;
            }
        }

        let seek =
            (player_position - current_position).normalize_or_zero() * DRONE_SWARM_SEEK_WEIGHT;
        let cohesion = if neighbor_count > 0.0 {
            ((center_sum / neighbor_count) - current_position).normalize_or_zero()
                * DRONE_SWARM_COHESION_WEIGHT
        } else {
            Vec2::ZERO
        };
        let alignment = if heading_sum.length_squared() > 0.0 {
            heading_sum.normalize_or_zero() * DRONE_SWARM_ALIGNMENT_WEIGHT
        } else {
            Vec2::ZERO
        };
        let separation = clamp_vec2_length(separation, 1.8) * DRONE_SWARM_SEPARATION_WEIGHT;

        let steering = seek + cohesion + alignment + separation;
        let aggression_boost = stats.contact_damage * 0.65;
        let health_scale = health.ratio();
        let desired_velocity = steering.normalize_or_zero()
            * (stats.speed + aggression_boost + randomish_speed_offset(entity.index()));
        let velocity_delta = desired_velocity - current_velocity;
        let max_delta = DRONE_SWARM_ACCELERATION * delta_seconds;
        let clamped_delta = clamp_vec2_length(velocity_delta, max_delta);
        let next_velocity = clamp_vec2_length(
            current_velocity + clamped_delta,
            DRONE_SWARM_MAX_SPEED + aggression_boost,
        );

        velocity.0 = if next_velocity.length_squared() > 0.0 {
            next_velocity
        } else {
            current_velocity
        };

        transform.translation.x += velocity.0.x * delta_seconds;
        transform.translation.y += velocity.0.y * delta_seconds;

        let movement_rotation = if velocity.0.length_squared() > 0.0 {
            velocity_to_angle(velocity.0)
        } else {
            visual.base_rotation
        };
        agent.spin_angle += visual.spin_speed * delta_seconds;
        let _ = (children, health_scale, movement_rotation);
    }
}

fn sync_drone_swarm_visuals(
    state: Res<UiGameState>,
    enemy_visuals: Query<
        (&SwarmVelocity, &DroneSwarmAgent, &DroneSwarmVisual, &Health),
        (With<Enemy>, Without<DroneSwarmSprite>),
    >,
    mut sprite_transforms: Query<(&Parent, &mut Transform), With<DroneSwarmSprite>>,
) {
    if state.selected_mode != GameMode::Arcade || !state.is_running {
        return;
    }

    for (parent, mut transform) in &mut sprite_transforms {
        let Ok((velocity, agent, visual, health)) = enemy_visuals.get(parent.get()) else {
            continue;
        };

        let movement_rotation = if velocity.0.length_squared() > 0.0 {
            velocity_to_angle(velocity.0)
        } else {
            visual.base_rotation
        };

        transform.scale = Vec3::splat(0.94 + health.ratio() * 0.12);
        transform.rotation = Quat::from_rotation_z(movement_rotation + agent.spin_angle);
    }
}

fn handle_projectile_enemy_collisions(
    mut commands: Commands,
    state: Res<UiGameState>,
    mut enemies: Query<(Entity, &Transform, &EnemyStats, &mut Health), With<Enemy>>,
    projectiles: Query<(Entity, &Transform, &Projectile)>,
) {
    if state.selected_mode != GameMode::Arcade || !state.is_running {
        return;
    }

    for (projectile_entity, projectile_transform, projectile) in &projectiles {
        let projectile_position = projectile_transform.translation.truncate();
        let mut hit_enemy = None;

        for (enemy_entity, enemy_transform, stats, health) in &mut enemies {
            let enemy_position = enemy_transform.translation.truncate();
            let collision_distance = projectile.radius + stats.collision_radius;

            if projectile_position.distance(enemy_position) <= collision_distance {
                hit_enemy = Some((enemy_entity, health));
                break;
            }
        }

        if let Some((_enemy_entity, mut health)) = hit_enemy {
            health.damage_fraction(projectile.damage_fraction);
            commands.entity(projectile_entity).despawn_recursive();
        }
    }
}

fn handle_enemy_player_collisions(
    state: Res<UiGameState>,
    enemy_query: Query<(&Transform, &EnemyStats), With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Health, &mut ContactCooldown), With<Player>>,
) {
    if state.selected_mode != GameMode::Arcade || !state.is_running {
        return;
    }

    let Ok((player_transform, mut player_health, mut contact_cooldown)) =
        player_query.get_single_mut()
    else {
        return;
    };

    if contact_cooldown.0 > 0.0 {
        return;
    }

    let player_position = player_transform.translation.truncate();

    for (enemy_transform, enemy_stats) in &enemy_query {
        let enemy_position = enemy_transform.translation.truncate();
        let collision_distance = PLAYER_COLLISION_RADIUS + enemy_stats.collision_radius;

        if player_position.distance(enemy_position) <= collision_distance {
            player_health.damage_fraction(player_contact_damage_fraction());
            contact_cooldown.0 = player_contact_invulnerability_seconds();
            break;
        }
    }
}

fn despawn_destroyed_enemies(mut commands: Commands, enemies: Query<(Entity, &Health), With<Enemy>>) {
    for (entity, health) in &enemies {
        if health.current <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_enemy_health_bars(
    state: Res<UiGameState>,
    enemy_health: Query<&Health, With<Enemy>>,
    mut transforms: ParamSet<(
        Query<(&Parent, &mut Sprite, &mut Transform), With<EnemyHealthBarFill>>,
    )>,
    root_parents: Query<&Parent, With<EnemyHealthBarRoot>>,
) {
    if state.selected_mode != GameMode::Arcade {
        return;
    }

    for (parent, mut sprite, mut transform) in &mut transforms.p0() {
        let Ok(root_parent) = root_parents.get(parent.get()) else {
            continue;
        };

        let Ok(health) = enemy_health.get(root_parent.get()) else {
            continue;
        };

        let fill_ratio = health.ratio();
        let width = ENEMY_HEALTH_BAR_WIDTH * fill_ratio.max(0.0);
        sprite.custom_size = Some(Vec2::new(width, ENEMY_HEALTH_BAR_HEIGHT));
        sprite.color = if fill_ratio > 0.5 {
            Color::srgba(0.28, 0.96, 0.54, 0.94)
        } else if fill_ratio > 0.25 {
            Color::srgba(1.0, 0.82, 0.24, 0.94)
        } else {
            Color::srgba(0.98, 0.34, 0.32, 0.94)
        };
        transform.translation.x = -(ENEMY_HEALTH_BAR_WIDTH - width) * 0.5;
    }
}

fn refresh_enemy_visibility(
    state: Res<UiGameState>,
    mut enemies: Query<&mut Visibility, With<Enemy>>,
) {
    if !state.is_changed() {
        return;
    }

    let visibility = if state.selected_mode == GameMode::Arcade && state.is_running {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut enemy_visibility in &mut enemies {
        *enemy_visibility = visibility;
    }
}

fn random_spawn_center(random: &mut EnemyRandom, player_position: Vec2) -> Vec2 {
    let angle = random.range_f32(0.0, std::f32::consts::TAU);
    let direction = Vec2::new(angle.cos(), angle.sin());
    let distance = random.range_f32(
        DRONE_SWARM_SPAWN_DISTANCE_MIN,
        DRONE_SWARM_SPAWN_DISTANCE_MAX,
    );

    player_position + direction * distance
}

fn tangent_for_spawn(spawn_center: Vec2) -> Vec2 {
    Vec2::new(-spawn_center.y, spawn_center.x)
}

fn velocity_to_angle(direction: Vec2) -> f32 {
    direction.x.atan2(direction.y)
}

fn clamp_vec2_length(vector: Vec2, max_length: f32) -> Vec2 {
    let length = vector.length();
    if length <= max_length || length == 0.0 {
        return vector;
    }

    vector / length * max_length
}

fn randomish_speed_offset(entity_index: u32) -> f32 {
    let centered = (entity_index % 7) as f32 - 3.0;
    centered * 4.5
}
