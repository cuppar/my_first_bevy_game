use bevy::prelude::*;

use crate::state::GameState::{GameInit, InGame, Loading};
use crate::{player, resource, world};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, log_transitions)
            .add_systems(
                OnEnter(Loading),
                (resource::load_sprite_sheet, transition_to_next_state).chain(),
            )
            .add_systems(
                OnEnter(GameInit),
                (
                    player::spawn_player,
                    player::gun::spawn_gun,
                    world::spawn_decoration,
                    transition_to_next_state,
                )
                    .chain(),
            );
    }
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<GameState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.before, transition.after
        );
    }
}

fn transition_to_next_state(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    let next = match **state {
        Loading => GameInit,
        GameInit => InGame,
        InGame => InGame,
    };
    next_state.set(next);
}
