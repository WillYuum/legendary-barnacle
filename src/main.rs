use bevy::{app::AppExit, prelude::*};

// use player_displayer::setup_sprite;

mod player_displayer;

fn main() {
    println!("Game Starting Up!");

    // create new App with default plugins plus throw in the player sprite from player_displayer.rs
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, main_setup)
        .add_systems(Startup, player_displayer::setup_sprite)
        .add_systems(Update, player_displayer::sprite_movement)
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
