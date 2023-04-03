pub mod events;
mod systems;
mod game;
mod main_menu;

use bevy::prelude::*;
use systems::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_system(handle_game_over)
        .add_system(exit_game)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .run();
}
