mod game_over_menu;
mod pause_menu;

use bevy::prelude::*;

use crate::AppState;

use self::{
    game_over_menu::systems::{
        interactions::{interact_with_main_menu_button, interact_with_play_again_button},
        layout::{despawn_game_over_menu, spawn_game_over_menu},
    },
    pause_menu::{despawn_pause_menu, spawn_pause_menu},
};

use super::SimulationState;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter Systems
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            // Systems
            .add_systems(
                Update,
                (
                    interact_with_main_menu_button,
                    interact_with_play_again_button,
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            // On Exit Systems
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
