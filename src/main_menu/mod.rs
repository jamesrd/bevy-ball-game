use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(print_main_menu);
    }
}

pub fn print_main_menu() {
    println!("Main menu");
}
