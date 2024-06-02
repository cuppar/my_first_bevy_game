use bevy::prelude::*;

use crate::movement::Velocity;
use crate::prelude::*;
use crate::state::GameState::InGame;

const PLAYER_MOVE_SPEED: f32 = 200.;
const PLAYER_GUN_Z_INDEX: usize = 1;

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
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
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
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }
    let player_translation = player_query.single().translation;
    let mut gun_transform = gun_query.single_mut();

    // 只移动xy平面,保持z-index不变
    gun_transform.translation = player_translation.xy().extend(gun_transform.translation.z);
}
