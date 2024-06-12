use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::player::gun::bullet;
use crate::player::gun::bullet::Bullet;
use crate::state::GameState::InGame;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_bullet_enemy_collision).run_if(in_state(InGame)),
        );
    }
}

fn handle_bullet_enemy_collision(
    mut q_enemy: Query<(&mut Enemy, &Transform), With<Enemy>>,
    q_bullet: Query<&Transform, With<Bullet>>,
) {
    if q_enemy.is_empty() || q_bullet.is_empty() {
        return;
    }

    for (mut enemy, enemy_transform) in &mut q_enemy {
        for bullet_transform in q_bullet.iter() {
            if enemy_transform
                .translation
                .xy()
                .distance_squared(bullet_transform.translation.xy())
                < 1000.
            {
                enemy.health -= bullet::DAMAGE;
            }
        }
    }
}
