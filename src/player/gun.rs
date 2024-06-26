use bevy::math::vec2;
use bevy::prelude::*;
use std::time::Duration;

use crate::player::gun::bullet::BulletPlugin;
use crate::player::Player;
use crate::prelude::*;
use crate::resource::CursorPosition;
use crate::state::GameState::InGame;

pub mod bullet;

const Z_INDEX: f32 = 20.;
const ROTATION_RADIANS: f32 = 50.;
const ORIGIN_OFFSET_TO_PLAYER: Vec2 = vec2(0., -35.);
const BULLET_RELOAD_TIME: f32 = 0.1;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BulletPlugin).add_systems(
            Update,
            (gun_follow_player, flip_gun).run_if(in_state(InGame)),
        );
    }
}

#[derive(Component)]
pub struct Gun;

#[derive(Component, Deref, DerefMut)]
struct BulletReloadTimer(Timer);

pub fn spawn_gun(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture: sprite_sheet.texture.clone().unwrap(),
            atlas: TextureAtlas {
                layout: sprite_sheet.layout.clone().unwrap(),
                index: 13,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
                .with_translation(Vec2::ZERO.extend(Z_INDEX)),
            ..default()
        },
        BulletReloadTimer(Timer::new(
            Duration::from_secs_f32(BULLET_RELOAD_TIME),
            TimerMode::Once,
        )),
        Gun,
    ));
}
fn flip_gun(mut q_gun: Query<(&mut Sprite, &Transform), With<Gun>>, cursor: Res<CursorPosition>) {
    if q_gun.is_empty() {
        return;
    }
    let (mut sprite, transform) = q_gun.single_mut();
    if let Some(cursor) = cursor.0 {
        sprite.flip_x = cursor.x < transform.translation.x;
    }
}

fn gun_follow_player(
    cursor_position: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let mut gun_transform = gun_query.single_mut();
    let player_pos = player_query.single().translation.xy();

    let gun_origin = player_pos + ORIGIN_OFFSET_TO_PLAYER;
    let gun_to_cursor = cursor_position.unwrap_or(gun_origin) - gun_origin;

    // 3d absolute
    if gun_to_cursor.length() > 0. {
        gun_transform.rotation =
            Quat::from_rotation_arc(Vec3::Y, gun_to_cursor.extend(0.).normalize());
    }

    let gun_direction = gun_transform.local_y().xy().normalize();

    let new_gun_pos = gun_origin + gun_direction * ROTATION_RADIANS;

    gun_transform.translation = new_gun_pos.extend(gun_transform.translation.z);
}
