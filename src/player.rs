use bevy::math::vec2;
use bevy::prelude::*;

use crate::movement::Velocity;
use crate::prelude::*;
use crate::resource::CursorPosition;
use crate::state::GameState::InGame;

const PLAYER_MOVE_SPEED: f32 = 200.;
const PLAYER_GUN_Z_INDEX: usize = 1;
const PLAYER_GUN_DISTANCE_OFFSET: f32 = 20.;
const PLAYER_GUN_POS_OFFSET: Vec2 = vec2(0., -30.);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_player_movement_input, gun_follow_player)
                .chain()
                .run_if(in_state(InGame)),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Gun;

pub fn spawn_player(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture: sprite_sheet.texture.clone().unwrap(),
            atlas: TextureAtlas {
                layout: sprite_sheet.layout.clone().unwrap(),
                index: 16,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Velocity {
            direction: Vec2::new(0., 0.),
            speed: PLAYER_MOVE_SPEED,
        },
        Player,
    ));
}
pub fn spawn_player_gun(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture: sprite_sheet.texture.clone().unwrap(),
            atlas: TextureAtlas {
                layout: sprite_sheet.layout.clone().unwrap(),
                index: 20,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR / 1.5))
                .with_translation(Vec2::ZERO.extend(PLAYER_GUN_Z_INDEX as f32)),
            ..default()
        },
        Velocity {
            direction: Vec2::new(0., 0.),
            speed: PLAYER_MOVE_SPEED,
        },
        Gun,
    ));
}

fn handle_player_movement_input(
    mut query: Query<&mut Velocity, With<Player>>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::new(0., 0.);
    let up = button_input.pressed(KeyCode::ArrowUp) || button_input.pressed(KeyCode::KeyW);
    let down = button_input.pressed(KeyCode::ArrowDown) || button_input.pressed(KeyCode::KeyS);
    let left = button_input.pressed(KeyCode::ArrowLeft) || button_input.pressed(KeyCode::KeyA);
    let right = button_input.pressed(KeyCode::ArrowRight) || button_input.pressed(KeyCode::KeyD);

    if up {
        direction.y += 1.;
    }
    if down {
        direction.y -= 1.;
    }
    if left {
        direction.x -= 1.;
    }
    if right {
        direction.x += 1.;
    }

    let mut player_velocity = query.single_mut();

    player_velocity.direction = direction.normalize_or_zero();
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

    let new_gun_pos = player_pos
        + PLAYER_GUN_POS_OFFSET
        + gun_transform.local_y().xy().normalize() * PLAYER_GUN_DISTANCE_OFFSET;

    gun_transform.translation = new_gun_pos.extend(gun_transform.translation.z);

    // 2d absolute
    // let mut absolute_angle = Vec2::Y.angle_between(gun_to_cursor);
    // if absolute_angle.is_nan() {
    //     absolute_angle = 0.;
    // }
    // let mut rotated_gun_transform =
    //     Transform::from_translation(gun_transform.translation).with_scale(gun_transform.scale);
    // rotated_gun_transform.rotate_z(absolute_angle);
    // *gun_transform = rotated_gun_transform;

    // 2d relative
    // let mut relative_angle = gun_transform.local_y().xy().angle_between(gun_to_cursor);
    // if relative_angle.is_nan() {
    //     relative_angle = 0.;
    // }
    // gun_transform.rotate_z(relative_angle);

    // 3d relative
    // if gun_to_cursor.length() > 0. {
    //     let rotation = Quat::from_rotation_arc(
    //         gun_transform.local_y().xyz(),
    //         gun_to_cursor.extend(0.).normalize(),
    //     );
    //     gun_transform.rotate_local(rotation);
    // } else {
    //     gun_transform.rotation = Quat::default();
    // }
}
