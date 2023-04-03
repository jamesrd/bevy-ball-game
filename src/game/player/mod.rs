pub mod components;
mod systems;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

use bevy::app::Plugin;
use bevy::prelude::*;
use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_startup_system(spawn_player)
            .add_system(player_movement.in_set(MovementSystemSet))
            .add_system(confine_player_movement.in_set(ConfinementSystemSet))
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
