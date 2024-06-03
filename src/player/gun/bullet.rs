use bevy::prelude::*;
use std::time::Duration;

use crate::player::gun::{BulletReloadTimer, Gun};
use crate::prelude::*;
use crate::state::GameState::InGame;

const Z_INDEX: usize = 11;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_bullet_input.run_if(in_state(InGame)));
    }
}

#[derive(Component)]
pub struct Bullet;

pub fn handle_bullet_input(
    mut commands: Commands,
    time: Res<Time>,
    sprite_sheet: Res<SpriteSheet>,
    mut q_gun: Query<(&Transform, &mut BulletReloadTimer), With<Gun>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if q_gun.is_empty() {
        return;
    }

    let (gun_transform, mut bullet_reload_timer) = q_gun.single_mut();
    bullet_reload_timer.tick(Duration::from_secs_f32(time.delta_seconds()));

    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    if !bullet_reload_timer.finished() {
        return;
    }

    bullet_reload_timer.reset();
    commands.spawn((
        SpriteSheetBundle {
            texture: sprite_sheet.texture.clone().unwrap(),
            atlas: TextureAtlas {
                layout: sprite_sheet.layout.clone().unwrap(),
                index: 2,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
                .with_translation(gun_transform.translation.xy().extend(Z_INDEX as f32))
                .with_rotation(gun_transform.rotation),
            ..default()
        },
        Bullet,
    ));
}
