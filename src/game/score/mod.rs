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
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_high_scores)
            .add_system(high_scores_updates)
            // Exit State systems
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
