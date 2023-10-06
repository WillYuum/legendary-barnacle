use bevy::{
    prelude::{
        AssetServer, Camera2dBundle, Commands, Component, Handle, Image, Query, Res, SpriteBundle,
        Transform,
    },
    time::Time,
};

pub struct Player(u64);

pub fn setup_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture: Handle<Image> = asset_server.load("../sprites/tile_0331.png");

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Direction::Up,
    ));
}

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
pub fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}
