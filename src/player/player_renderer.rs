use bevy::prelude::{
    AssetServer, Commands, Component, Handle, Image, Res, SpriteBundle, Transform,
};

use super::player_movement;

pub fn setup_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture: Handle<Image> = asset_server.load("textures/tile_0331.png");

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        player_movement::Direction::Up,
    ));
}
