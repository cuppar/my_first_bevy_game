use bevy::prelude::*;

use crate::animation::{AnimationIndices, AnimationTimer};
use crate::movement::Velocity;
use crate::player::gun::GunPlugin;
use crate::prelude::*;
use crate::resource::CursorPosition;
use crate::state::GameState::InGame;

pub mod gun;

const PLAYER_MOVE_SPEED: f32 = 200.;
const Z_INDEX: f32 = 10.;
const IDLE_ANIMATION_INDICES: AnimationIndices = AnimationIndices::new(0, 3);
const WALKING_ANIMATION_INDICES: AnimationIndices = AnimationIndices::new(8, 12);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_plugins(GunPlugin)
            .add_systems(
                Update,
                (
                    log_transitions,
                    (
                        handle_player_movement_input,
                        update_player_state,
                        update_player_animation,
                        flip_player,
                    )
                        .chain()
                        .run_if(in_state(InGame)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum PlayerState {
    #[default]
    Idle,
    Walking,
}

pub fn spawn_player(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture: sprite_sheet.texture.clone().unwrap(),
            atlas: TextureAtlas {
                layout: sprite_sheet.layout.clone().unwrap(),
                index: IDLE_ANIMATION_INDICES.first,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
                .with_translation(Vec2::ZERO.extend(Z_INDEX)),
            ..default()
        },
        Velocity::default(),
        IDLE_ANIMATION_INDICES,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
    ));
}

fn flip_player(
    mut q_player: Query<(&mut Sprite, &Transform), With<Player>>,
    cursor: Res<CursorPosition>,
) {
    if q_player.is_empty() {
        return;
    }
    let (mut sprite, transform) = q_player.single_mut();
    if let Some(cursor) = cursor.0 {
        sprite.flip_x = cursor.x < transform.translation.x;
    }
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

    *player_velocity = Velocity(direction.normalize_or_zero() * PLAYER_MOVE_SPEED);
}

fn update_player_state(
    q_player: Query<&Velocity, With<Player>>,
    player_state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    if q_player.is_empty() {
        return;
    }

    let player_velocity = q_player.single();
    match player_state.get() {
        PlayerState::Idle => {
            if player_velocity.length() > f32::EPSILON {
                next_state.set(PlayerState::Walking);
            }
        }
        PlayerState::Walking => {
            if player_velocity.length() < f32::EPSILON {
                next_state.set(PlayerState::Idle);
            }
        }
    };
}

fn update_player_animation(
    mut q_player: Query<&mut AnimationIndices, With<Player>>,
    mut transition: EventReader<StateTransitionEvent<PlayerState>>,
) {
    if q_player.is_empty() {
        return;
    }

    let mut animation_indices = q_player.single_mut();
    for transition in transition.read() {
        *animation_indices = match transition.after {
            PlayerState::Idle => IDLE_ANIMATION_INDICES,
            PlayerState::Walking => WALKING_ANIMATION_INDICES,
        };
    }
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<PlayerState>>) {
    for transition in transitions.read() {
        info!(
            "PlayerState: {:?} => {:?}",
            transition.before, transition.after
        );
    }
}
