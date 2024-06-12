#![allow(clippy::type_complexity)]
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;
use std::time::Duration;

use crate::animation::{AnimationIndices, AnimationTimer};
use crate::movement::Velocity;
use crate::player::Player;
use crate::prelude::*;
use crate::state::GameState::InGame;
use crate::utils::get_random_position_around;

const Z_INDEX: f32 = 9.;
const SPAWN_PERIOD: f32 = 1.0;
const MAX_COUNT: usize = 100_000;
const MAX_PER_FRAME_SPAWN_COUNT: usize = 3000;
const SPEED: f32 = 100.;
const INIT_HEALTH: f32 = 100.;
const DISTANCE_FROM_PLAYER: f32 = 500.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemy.run_if(on_timer(Duration::from_secs_f32(SPAWN_PERIOD))),
                follow_player,
                flip_enemy,
                despawn_dead_enemy,
            )
                .run_if(in_state(InGame)),
        );
    }
}

#[derive(Component)]
pub struct Enemy {
    pub(crate) health: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: INIT_HEALTH,
        }
    }
}

fn despawn_dead_enemy(mut commands: Commands, q_enemy: Query<(Entity, &Enemy), With<Enemy>>) {
    if q_enemy.is_empty() {
        return;
    }

    for (entity, enemy) in q_enemy.iter() {
        if enemy.health <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

fn follow_player(
    q_player: Query<&Transform, With<Player>>,
    mut q_enemy: Query<(&Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
) {
    if q_player.is_empty() || q_enemy.is_empty() {
        return;
    }

    let player_pos = q_player.single().translation.xy();
    for (transform, mut velocity) in &mut q_enemy {
        *velocity = Velocity((player_pos - transform.translation.xy()).normalize_or_zero() * SPEED);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    q_enemy: Query<(), With<Enemy>>,
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let enemy_count = q_enemy.iter().len();
    if enemy_count >= MAX_COUNT {
        return;
    }

    if q_player.is_empty() {
        return;
    }

    let spawn_count = (MAX_COUNT - enemy_count).min(MAX_PER_FRAME_SPAWN_COUNT);
    let player_pos = q_player.single().translation.xy();

    for _ in 0..spawn_count {
        let mut rng = rand::thread_rng();
        let position = get_random_position_around(
            player_pos,
            rng.gen_range(DISTANCE_FROM_PLAYER..WORLD_WIDTH.max(WORLD_HEIGHT) / 2.0),
        );
        commands.spawn((
            SpriteSheetBundle {
                texture: sprite_sheet.texture.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: sprite_sheet.layout.clone().unwrap(),
                    index: 4,
                },
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
                    .with_translation(position.extend(Z_INDEX)),
                ..default()
            },
            Velocity::default(),
            AnimationIndices::new(4, 7),
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Enemy::default(),
        ));
    }
}

fn flip_enemy(
    mut q_enemy: Query<(&mut Sprite, &Transform), With<Enemy>>,
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if q_enemy.is_empty() || q_player.is_empty() {
        return;
    }
    let player_pos = q_player.single().translation.xy();
    for (mut sprite, enemy_transform) in &mut q_enemy {
        sprite.flip_x = player_pos.x < enemy_transform.translation.x;
    }
}
