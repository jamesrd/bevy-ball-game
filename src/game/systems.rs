use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    mut next_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space){
        let to_state = match simulation_state.get() {
            SimulationState::Paused => SimulationState::Running,
            SimulationState::Running => SimulationState::Paused,
        };

        println!("Simulation {:?}", to_state);
        next_state.set(to_state);
    }
}
