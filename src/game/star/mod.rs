pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 10;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<StarSpawnTimer>()
            // Enter State Systems
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            //Exit State Systems
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
