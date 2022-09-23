use bevy::prelude::*;
use std::{collections::HashMap, fs};

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("collection/player_01.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Direction::Stop);
}

fn read_text(path: &str) -> String {
  let data = fs::read_to_string(path).expect("Unable to read file");
  return data;
}

fn spawn_trees(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut xx = 0.0;
    let mut yy = 0.0;
    let txt_map = read_text("assets/maps/examples/001.txt");
    for c in txt_map.chars() {
        xx = xx + 1.0;
        if c == '\n' {
            yy = yy - 1.0;
            xx = 0.0
        } else {
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load(map_char_to_path(&c.to_string())),
                transform: Transform {
                    translation: Vec3 {
                        x: xx * 25.0,
                        y: yy * 25.0,
                        z: 0.,
                    },
                    scale: Vec3 {
                        x: 2.,
                        y: 2.,
                        z: 1.,
                    },
                    ..Default::default()
                },
                ..default()
            });
        }
    }
}

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

fn map_char_to_path(s: &str) -> &'static str {
    let map_char_texture: HashMap<&str, &str> = HashMap::from([
        ("^", "collection/tree_01.png"),
        ("#", "collection/wall_01.png"),
        (" ", "collection/floor_01.png"),
        ("\n", ""),
    ]);

    return map_char_texture[s];
}

// pub const MAP_000: &str = "
// ^ #
// ";

// pub const MAP_001: &str = "
// ^ ^ ^ ^ ^ ^  ^
// ^ ^ ^ ^ ^ ^  ^
// ^            ^
// ^ ########   ^
// ^    #       ^
// ^    #       ^
// ";

pub struct PluginSpawn;

impl Plugin for PluginSpawn {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_trees)
            .add_system(sprite_movement);
    }
}
