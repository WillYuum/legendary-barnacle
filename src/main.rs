use bevy::{app::AppExit, prelude::*};

pub mod player;

fn main() {
    println!("Game Starting Up!");
    // actions::player

    // create new App with default plugins plus throw in the player sprite from player_displayer.rs
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, main_setup)
        .add_systems(Startup, player::player_renderer::setup_sprite)
        .add_systems(Update, player::player_movement::sprite_movement)
        // .add_systems(Update, player_displayer::sprite_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, update)
        .run();
}

// Define your startup system
fn main_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Define your update system
fn update() {
    // Add your game update logic here
}
