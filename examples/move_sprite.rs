// source: https://github.com/bevyengine/bevy/blob/main/examples/2d/move_sprite.rs

// Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(CLEAR));

    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player_01.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Direction::Stop);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("tree_01.png"),
        transform: Transform {
            translation: Vec3 {
                x: -50.0,
                y: 0.,
                z: 0.,
            },
            scale: Vec3 {
                x: 3.,
                y: 3.,
                z: 1.,
            },
            ..Default::default()
        },
        ..default()
    });
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut logo, mut transform) in &mut sprite_position {
        if input.pressed(KeyCode::Up) {
            *logo = Direction::Down;
        } else if input.pressed(KeyCode::Down) {
            *logo = Direction::Up;
        } else if input.pressed(KeyCode::Left) {
            *logo = Direction::Left;
        } else if input.pressed(KeyCode::Right) {
            *logo = Direction::Right;
        } else {
            *logo = Direction::Stop;
        }

        match *logo {
            Direction::Up => {
                transform.translation.x += 0. * time.delta_seconds();
                transform.translation.y += -150. * time.delta_seconds();
            }
            Direction::Down => {
                transform.translation.x += 0. * time.delta_seconds();
                transform.translation.y += 150. * time.delta_seconds();
            }
            Direction::Left => {
                transform.translation.x += -150. * time.delta_seconds();
                transform.translation.y += 0. * time.delta_seconds();
            }
            Direction::Right => {
                transform.translation.x += 150. * time.delta_seconds();
                transform.translation.y += 0. * time.delta_seconds();
            }
            Direction::Stop => {
                transform.translation.x += 0. * time.delta_seconds();
                transform.translation.y += 0. * time.delta_seconds();
            }
        }

        if transform.translation.y > 200. {
            *logo = Direction::Stop;
        } else if transform.translation.y < -200. {
            *logo = Direction::Stop;
        } else if transform.translation.x > 200. {
            *logo = Direction::Stop;
        } else if transform.translation.x < -200. {
            *logo = Direction::Stop;
        }
    }
}
