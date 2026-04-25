//! Shared procedural visual effects for the arcade game.

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct TrailEmitter {
    interval: f32,
    remaining: f32,
}

impl TrailEmitter {
    pub(super) fn new(interval: f32) -> Self {
        Self {
            interval,
            remaining: 0.0,
        }
    }

    pub(super) fn tick(&mut self, delta_seconds: f32) -> bool {
        self.remaining -= delta_seconds;

        if self.remaining > 0.0 {
            return false;
        }

        self.remaining = self.interval;
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TrailSegmentSpec {
    pub position: Vec2,
    pub direction: Vec2,
    pub z: f32,
    pub length: f32,
    pub width: f32,
    pub lifetime: f32,
    pub color: Color,
    pub alpha: f32,
    pub drift_speed: f32,
}

#[derive(Component)]
pub(super) struct FadingSprite {
    timer: Timer,
    duration: f32,
    start_size: Vec2,
    end_size: Vec2,
    color: Color,
    start_alpha: f32,
    velocity: Vec3,
    spin: f32,
}

pub(super) fn spawn_trail_segment(commands: &mut Commands, spec: TrailSegmentSpec) {
    let direction = spec.direction.normalize_or_zero();
    if direction == Vec2::ZERO {
        return;
    }

    spawn_fading_sprite(
        commands,
        FadingSpriteSpec {
            position: Vec3::new(spec.position.x, spec.position.y, spec.z),
            rotation: direction_to_rotation(direction),
            start_size: Vec2::new(spec.width, spec.length),
            end_size: Vec2::new(spec.width * 0.18, spec.length * 0.42),
            lifetime: spec.lifetime,
            color: spec.color,
            alpha: spec.alpha,
            velocity: direction.extend(0.0) * spec.drift_speed,
            spin: 0.0,
        },
    );
}

pub(super) fn spawn_explosion_effect(commands: &mut Commands, position: Vec2, radius: f32) {
    let center = Vec3::new(position.x, position.y, 4.5);
    let core_color = Color::srgba(1.0, 0.92, 0.62, 1.0);
    let heat_color = Color::srgba(1.0, 0.28, 0.08, 1.0);
    let smoke_color = Color::srgba(0.45, 0.42, 0.5, 1.0);

    spawn_fading_sprite(
        commands,
        FadingSpriteSpec {
            position: center,
            rotation: 0.0,
            start_size: Vec2::splat(radius * 0.55),
            end_size: Vec2::splat(radius * 1.18),
            lifetime: 0.18,
            color: core_color,
            alpha: 0.92,
            velocity: Vec3::ZERO,
            spin: 5.2,
        },
    );

    spawn_fading_sprite(
        commands,
        FadingSpriteSpec {
            position: center + Vec3::new(0.0, 0.0, -0.02),
            rotation: std::f32::consts::FRAC_PI_4,
            start_size: Vec2::splat(radius * 0.34),
            end_size: Vec2::splat(radius * 1.52),
            lifetime: 0.32,
            color: heat_color,
            alpha: 0.58,
            velocity: Vec3::ZERO,
            spin: -2.8,
        },
    );

    for index in 0..10 {
        let angle = index as f32 * std::f32::consts::TAU / 10.0;
        let direction = Vec2::new(angle.cos(), angle.sin());
        let length = radius * if index % 2 == 0 { 0.72 } else { 0.52 };
        let speed = radius * (1.55 + index as f32 * 0.055);

        spawn_fading_sprite(
            commands,
            FadingSpriteSpec {
                position: center + direction.extend(0.0) * radius * 0.12,
                rotation: direction_to_rotation(direction),
                start_size: Vec2::new(radius * 0.055, length),
                end_size: Vec2::new(radius * 0.018, length * 0.35),
                lifetime: 0.22 + index as f32 * 0.012,
                color: if index % 3 == 0 {
                    core_color
                } else {
                    heat_color
                },
                alpha: 0.82,
                velocity: direction.extend(0.0) * speed,
                spin: if index % 2 == 0 { 1.8 } else { -1.4 },
            },
        );
    }

    for index in 0..6 {
        let angle = (index as f32 + 0.5) * std::f32::consts::TAU / 6.0;
        let direction = Vec2::new(angle.cos(), angle.sin());

        spawn_fading_sprite(
            commands,
            FadingSpriteSpec {
                position: center + Vec3::new(0.0, 0.0, -0.04),
                rotation: direction_to_rotation(direction),
                start_size: Vec2::new(radius * 0.12, radius * 0.2),
                end_size: Vec2::new(radius * 0.35, radius * 0.46),
                lifetime: 0.44,
                color: smoke_color,
                alpha: 0.24,
                velocity: direction.extend(0.0) * radius * 0.34,
                spin: 0.75,
            },
        );
    }
}

pub(super) fn animate_visual_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut FadingSprite, &mut Sprite, &mut Transform)>,
) {
    let delta_seconds = time.delta_secs();

    for (entity, mut effect, mut sprite, mut transform) in &mut effects {
        effect.timer.tick(time.delta());
        let progress = (effect.timer.elapsed_secs() / effect.duration).clamp(0.0, 1.0);
        let fade = (1.0 - progress).powf(1.45);

        sprite.custom_size = Some(effect.start_size.lerp(effect.end_size, progress));
        sprite.color = effect.color.with_alpha(effect.start_alpha * fade);
        transform.translation += effect.velocity * delta_seconds;
        transform.rotate_z(effect.spin * delta_seconds);

        if effect.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

struct FadingSpriteSpec {
    position: Vec3,
    rotation: f32,
    start_size: Vec2,
    end_size: Vec2,
    lifetime: f32,
    color: Color,
    alpha: f32,
    velocity: Vec3,
    spin: f32,
}

fn spawn_fading_sprite(commands: &mut Commands, spec: FadingSpriteSpec) {
    commands.spawn((
        FadingSprite {
            timer: Timer::from_seconds(spec.lifetime, TimerMode::Once),
            duration: spec.lifetime,
            start_size: spec.start_size,
            end_size: spec.end_size,
            color: spec.color,
            start_alpha: spec.alpha,
            velocity: spec.velocity,
            spin: spec.spin,
        },
        Sprite::from_color(spec.color.with_alpha(spec.alpha), spec.start_size),
        Transform::from_translation(spec.position)
            .with_rotation(Quat::from_rotation_z(spec.rotation)),
    ));
}

fn direction_to_rotation(direction: Vec2) -> f32 {
    (-direction.x).atan2(direction.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trail_emitter_waits_for_interval_after_spawn() {
        let mut emitter = TrailEmitter::new(0.08);

        assert!(emitter.tick(0.016));
        assert!(!emitter.tick(0.016));
        assert!(!emitter.tick(0.016));
        assert!(!emitter.tick(0.016));
        assert!(!emitter.tick(0.016));
        assert!(emitter.tick(0.016));
    }
}
