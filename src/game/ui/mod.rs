mod pause_menu;
mod styles;

use bevy::prelude::*;

use self::pause_menu::{despawn_pause_menu, spawn_pause_menu};

use super::SimulationState;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter Systems
            .add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            // Systems
            // On Exit Systems
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}
