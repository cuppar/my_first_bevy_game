use bevy::prelude::*;

use crate::state::GameState::InGame;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub const fn new(first: usize, last: usize) -> AnimationIndices {
        Self { first, last }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite.run_if(in_state(InGame)));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut q: Query<(&mut TextureAtlas, &AnimationIndices, &mut AnimationTimer)>,
) {
    for (mut texture_atlas, indices, mut timer) in q.iter_mut() {
        timer.tick(time.delta());
        if !timer.just_finished() {
            continue;
        }

        if texture_atlas.index < indices.first || texture_atlas.index > indices.last {
            texture_atlas.index = indices.first;
            continue;
        }

        texture_atlas.index = if texture_atlas.index == indices.last {
            indices.first
        } else {
            texture_atlas.index + 1
        }
    }
}
