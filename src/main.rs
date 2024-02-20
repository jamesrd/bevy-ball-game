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
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, exit_game)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, transition_to_game_state)
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
