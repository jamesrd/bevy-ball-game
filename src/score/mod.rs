pub mod resources;
mod systems;

use bevy::app::Plugin;
use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_system(update_score)
            .add_system(update_high_scores)
            .add_system(high_scores_updates);
    }
}
