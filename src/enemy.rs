use std::time::Duration;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::{thread_rng, Rng};

use crate::movement::Velocity;
use crate::player::Player;
use crate::prelude::*;
use crate::state::GameState::InGame;

const Z_INDEX: f32 = 9.;
const SPAWN_PERIOD: f32 = 1.0;
const MAX_COUNT: usize = 100_000;
const MAX_PER_FRAME_SPAWN_COUNT: usize = 100;
const SPEED: f32 = 100.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemy.run_if(on_timer(Duration::from_secs_f32(SPAWN_PERIOD))),
                follow_player,
                flip_enemy,
            )
                .run_if(in_state(InGame)),
        );
    }
}

#[derive(Component)]
pub struct Enemy;

fn follow_player(
    q_player: Query<&Transform, With<Player>>,
    mut q_enemy: Query<(&Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
) {
    if q_player.is_empty() || q_enemy.is_empty() {
        return;
    }

    let player_pos = q_player.single().translation.xy();
    for (transform, mut velocity) in &mut q_enemy {
        velocity.direction = (player_pos - transform.translation.xy()).normalize_or_zero();
    }
}

fn spawn_enemy(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    q_enemy: Query<(), With<Enemy>>,
) {
    let enemy_count = q_enemy.iter().len();
    if enemy_count >= MAX_COUNT {
        return;
    }

    let spawn_count = (MAX_COUNT - enemy_count).min(MAX_PER_FRAME_SPAWN_COUNT);

    for _ in 0..spawn_count {
        let x = thread_rng().gen_range((-WORLD_WIDTH / 2.)..(WORLD_WIDTH / 2.));
        let y = thread_rng().gen_range((-WORLD_HEIGHT / 2.)..(WORLD_HEIGHT / 2.));
        commands.spawn((
            SpriteSheetBundle {
                texture: sprite_sheet.texture.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: sprite_sheet.layout.clone().unwrap(),
                    index: 4,
                },
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
                    .with_translation(vec2(x, y).extend(Z_INDEX)),
                ..default()
            },
            Velocity {
                speed: SPEED,
                ..default()
            },
            Enemy,
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
