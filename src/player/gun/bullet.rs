use bevy::prelude::*;
use rand::Rng;
use std::time::{Duration, Instant};

use crate::movement::Velocity;
use crate::player::gun::{BulletReloadTimer, Gun};
use crate::prelude::*;
use crate::state::GameState::InGame;

const Z_INDEX: f32 = 11.;
const OFFSET: f32 = 30.;
const SPEED: f32 = 1000.;
pub(crate) const DAMAGE: f32 = 100.;
const ALIVE_TIMER: f32 = 1.2;
const COUNT_PER_SPAWN: usize = 3;
const SPAWN_ANGLE: f32 = 30.;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_bullet_input, despawn_old_bullets).run_if(in_state(InGame)),
        );
    }
}
#[derive(Component)]
pub struct Bullet;

#[derive(Component, Deref, DerefMut)]
struct SpawnInstant(Instant);

fn despawn_old_bullets(
    mut commands: Commands,
    mut query: Query<(&SpawnInstant, Entity), With<Bullet>>,
) {
    if query.is_empty() {
        return;
    }

    for (spawn_instant, entity) in &mut query {
        if spawn_instant.elapsed().as_secs_f32() > ALIVE_TIMER {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_bullet_input(
    mut commands: Commands,
    time: Res<Time>,
    sprite_sheet: Res<SpriteSheet>,
    mut q_gun: Query<(&Transform, &mut BulletReloadTimer), With<Gun>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if q_gun.is_empty() {
        return;
    }

    let (gun_transform, mut bullet_reload_timer) = q_gun.single_mut();
    bullet_reload_timer.tick(Duration::from_secs_f32(time.delta_seconds()));

    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    if !bullet_reload_timer.finished() {
        return;
    }

    bullet_reload_timer.reset();

    let mut pos = gun_transform.translation.xy();
    let mut direction = gun_transform.local_y().xy().normalize();
    pos += direction * OFFSET;

    for _ in 0..COUNT_PER_SPAWN {
        let mut rng = rand::thread_rng();
        let angle = rng
            .gen_range(-SPAWN_ANGLE / 2.0..SPAWN_ANGLE / 2.0)
            .to_radians();
        direction = Vec2::from_angle(angle).rotate(direction);
        let bullet_rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.));

        commands.spawn((
            SpriteSheetBundle {
                texture: sprite_sheet.texture.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: sprite_sheet.layout.clone().unwrap(),
                    index: 14,
                },
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
                    .with_translation(pos.extend(Z_INDEX))
                    .with_rotation(bullet_rotation),
                ..default()
            },
            Velocity(direction * SPEED),
            Bullet,
            SpawnInstant(Instant::now()),
        ));
    }
}
