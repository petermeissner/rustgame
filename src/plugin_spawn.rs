use bevy::prelude::*;
use std::{collections::HashMap, fs};

use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

pub const MAX_X: f32 = 244.0;
pub const MAX_Y: f32 = 142.0;
pub const MIN_Y: f32 = -122.0;
pub const MIN_X: f32 = -242.0;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct MySpriteWalkable;
#[derive(Component)]
struct MySpriteCollide;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("collection/player_01.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Direction::Stop)
        .insert(Player);
}

/**
## Simple read text from file into string
*/
fn read_text(path: &str) -> String {
    let data = fs::read_to_string(path).expect("Unable to read file");
    return data;
}

/**
## Put scenery from map file onto screen
*/
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
            let spbdl = SpriteBundle {
                texture: asset_server.load(map_char_to_path(&c.to_string())),
                transform: Transform {
                    translation: Vec3 {
                        x: xx * 25.0 - WINDOW_WIDTH / 2.0 - 12.5,
                        y: yy * 25.0 + WINDOW_HEIGHT / 2.0 - 25.0,
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
            };
            commands.spawn_bundle(spbdl);
        }
    }
}

fn sprite_movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut sprite_query: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut dir, mut transform) in &mut sprite_query {
        if input.pressed(KeyCode::Up) {
          if transform.translation.y > MAX_Y {
            *dir = Direction::Stop;
          } else {  
            *dir = Direction::Down;
          }
        } else if input.pressed(KeyCode::Down) {
          if transform.translation.y < MIN_Y {
            *dir = Direction::Stop
          } else {
            *dir = Direction::Up;
          }
        } else if input.pressed(KeyCode::Left) {
          if transform.translation.x < MIN_X {
            *dir = Direction::Stop;
          } else {
            *dir = Direction::Left;
          }
        } else if input.pressed(KeyCode::Right) {
          if transform.translation.x > MAX_X {
            *dir = Direction::Stop; 
          } else {
            *dir = Direction::Right;
          }
        } else {
            *dir = Direction::Stop;
        }


        if input.pressed(KeyCode::Space) {
            println!("{},{}", transform.translation.x, transform.translation.y);
        }

        match *dir {
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
    }
}

fn map_char_to_path(s: &str) -> &'static str {
    let map_char_texture: HashMap<&str, &str> = HashMap::from([
        ("v", "collection/wall_01.png"),
        ("V", "collection/wall_04.png"),
        ("w", "collection/wall_02.png"),
        ("W", "collection/wall_03.png"),
        ("^", "collection/tree_01.png"),
        (" ", "collection/floor_01.png"),
        ("\n", ""),
    ]);

    return map_char_texture[s];
}

pub struct PluginSpawn;

impl Plugin for PluginSpawn {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_trees)
            .add_system(sprite_movement);
    }
}
