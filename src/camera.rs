use bevy::prelude::*;

use crate::state::GameState::GameInit;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameInit), setup_camera);
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
