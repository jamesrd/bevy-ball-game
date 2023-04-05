pub mod components;
pub mod resources;
mod systems;

use bevy::app::Plugin;
use bevy::prelude::*;
use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<EnemySpawnTimer>()
            // Enter State systems
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
