use crate::state::GameState::{self, GameInit, Loading};
use bevy::prelude::*;

const SPRITE_SHEET_PATH: &str = "sheet.png";
const SPRITE_SHEET_W: usize = 32;
const SPRITE_SHEET_H: usize = 32;
const TILE_W: usize = 24;
const TILE_H: usize = 24;

#[derive(Resource, Default)]
pub struct SpriteSheet {
    pub texture: Option<Handle<Image>>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
}

pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSheet>()
            .add_systems(OnEnter(Loading), load_assets);
    }
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut sprite_sheet: ResMut<SpriteSheet>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    sprite_sheet.texture = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    sprite_sheet.layout = Some(texture_atlas_layout.add(layout));
    next_state.set(GameInit);
}
