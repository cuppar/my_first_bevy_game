use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState::InGame;

const SPRITE_SHEET_PATH: &str = "sheet.png";
const SPRITE_SHEET_W: usize = 4;
const SPRITE_SHEET_H: usize = 2;
const TILE_W: usize = 16;
const TILE_H: usize = 16;

#[derive(Resource, Default)]
pub struct SpriteSheet {
    pub texture: Option<Handle<Image>>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
}

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct CursorPosition(Option<Vec2>);

pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSheet>()
            .init_resource::<CursorPosition>()
            .add_systems(Update, update_cursor_position.run_if(in_state(InGame)));
    }
}

pub fn load_sprite_sheet(
    asset_server: Res<AssetServer>,
    mut sprite_sheet: ResMut<SpriteSheet>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
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
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let window = q_window.single();
    let (camera, camera_global_transform) = q_camera.single();

    **cursor_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_global_transform, cursor))
        .map(|ray| ray.origin.truncate());
}
