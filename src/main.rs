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
        .add_state::<AppState>()
        .add_startup_system(spawn_camera)
        .add_system(handle_game_over)
        .add_system(exit_game)
        .add_system(transition_to_main_menu_state)
        .add_system(transition_to_game_state)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
