pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;
pub mod ui;

use bevy::prelude::*;

use crate::{events::GameOver, AppState};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use self::ui::GameUiPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // On Enter Systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // Plugins
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_plugins(GameUiPlugin)
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // On Exit Systems
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
