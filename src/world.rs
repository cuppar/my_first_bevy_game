use bevy::math::vec2;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::prelude::SpriteSheet;
use crate::prelude::*;

const Z_INDEX: f32 = 0.;
const DECORATION_COUNT: usize = 100;

pub fn spawn_decoration(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    for _ in 0..DECORATION_COUNT {
        let x = thread_rng().gen_range((-WORLD_WIDTH / 2.)..(WORLD_WIDTH / 2.));
        let y = thread_rng().gen_range((-WORLD_HEIGHT / 2.)..(WORLD_HEIGHT / 2.));

        commands.spawn((SpriteSheetBundle {
            texture: sprite_sheet.texture.clone().unwrap(),
            atlas: TextureAtlas {
                layout: sprite_sheet.layout.clone().unwrap(),
                index: thread_rng().gen_range(4..=5),
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
                .with_translation(vec2(x, y).extend(Z_INDEX)),
            ..default()
        },));
    }
}
