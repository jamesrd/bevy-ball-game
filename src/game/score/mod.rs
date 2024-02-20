pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            // Enter State Systems
            .add_systems(OnEnter(AppState::Game),insert_score)
            // Systems
            .add_systems(Update, update_score.run_if(in_state(AppState::Game)))
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updates)
            // Exit State systems
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
