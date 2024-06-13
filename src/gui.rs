use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::state::GameState::InGame;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_text.run_if(in_state(InGame)));
    }
}

#[derive(Component, Debug)]
struct StatText;

pub fn spawn_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "hello\nbevy!",
            TextStyle {
                font_size: 50.0,
                color: Color::RED,
                ..default()
            },
        ),
        StatText,
    ));
}

fn update_text(
    mut q_text: Query<&mut Text, With<StatText>>,
    q_enemy: Query<(), With<Enemy>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if q_text.is_empty() {
        return;
    }

    let enemy_count = q_enemy.iter().count();
    let mut text = q_text.single_mut();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps.smoothed() {
            text.sections[0].value = format!("FPS: {:.2}\nEnemys: {}", fps, enemy_count);
        }
    }
}
