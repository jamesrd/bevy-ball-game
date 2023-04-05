use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    mut next_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let to_state = match simulation_state.0 {
            SimulationState::Paused => SimulationState::Running,
            SimulationState::Running => SimulationState::Paused,
        };

        println!("Simulation {:?}", to_state);
        next_state.set(to_state);
    }
}
