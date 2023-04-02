pub mod events;
pub mod systems;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

use bevy::app::App;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_event::<GameOver>()
        .add_system(handle_game_over)
        .add_system(exit_game)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .run();
}
