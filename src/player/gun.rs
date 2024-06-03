use bevy::math::vec2;
use bevy::prelude::*;
use std::time::Duration;

use crate::player::gun::bullet::BulletPlugin;
use crate::player::Player;
use crate::prelude::*;
use crate::resource::CursorPosition;
use crate::state::GameState::InGame;

mod bullet;

const Z_INDEX: f32 = 20.;
const DISTANCE_OFFSET: f32 = 20.;
const POS_OFFSET: Vec2 = vec2(0., -20.);
const BULLET_RELOAD_TIME: f32 = 0.07;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BulletPlugin)
            .add_systems(Update, gun_follow_player.run_if(in_state(InGame)));
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
                index: 1,
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

fn gun_follow_player(
    cursor_position: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let mut gun_transform = gun_query.single_mut();
    let gun_pos = gun_transform.translation.xy();

    let cursor_pos = cursor_position.unwrap_or(gun_pos);
    let gun_to_cursor = cursor_pos - gun_pos;

    // 3d absolute
    if gun_to_cursor.length() > 0. {
        gun_transform.rotation =
            Quat::from_rotation_arc(Vec3::Y, gun_to_cursor.extend(0.).normalize());
    } else {
        gun_transform.rotation = Quat::default();
    }

    let player_pos = player_query.single().translation.xy();

    let new_gun_pos =
        player_pos + POS_OFFSET + gun_transform.local_y().xy().normalize() * DISTANCE_OFFSET;

    gun_transform.translation = new_gun_pos.extend(gun_transform.translation.z);
}
