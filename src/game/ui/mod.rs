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
            .add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            .add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            // Systems
            .add_systems(
                (
                    interact_with_main_menu_button,
                    interact_with_play_again_button,
                )
                    .in_set(OnUpdate(AppState::GameOver)),
            )
            // On Exit Systems
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}
