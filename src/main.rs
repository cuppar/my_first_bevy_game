use bevy::prelude::*;

use my_first_bevy_game::animation;
use my_first_bevy_game::camera::CameraPlugin;
use my_first_bevy_game::collision::CollisionPlugin;
use my_first_bevy_game::enemy::EnemyPlugin;
use my_first_bevy_game::movement::MovementPlugin;
use my_first_bevy_game::player::PlayerPlugin;
use my_first_bevy_game::resource::ResourcePlugin;
use my_first_bevy_game::setup::SetupPlugin;
use my_first_bevy_game::state::GameStatePlugin;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_plugins(ResourcePlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}
