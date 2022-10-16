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

struct Position {
    x: f32,
    y: f32,
}
#[derive(Component)]
struct Player {
    position: Position,
    moving_to: Position,
}

impl Player {
    fn new() -> Player {
        return Player {
            position: Position { x: 0.0, y: 0.0 },
            moving_to: Position { x: 0.0, y: 0.0 },
        };
    }

    fn new_at_pos(x: f32, y: f32) -> Player {
        return Player {
            position: Position { x: x, y: y },
            moving_to: Position { x: x, y: y },
        };
    }
}

#[derive(Component)]
struct MySpriteWalkable;
#[derive(Component)]
struct MySpriteCollide;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pl = Player::new_at_pos(0.0, 0.0);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("collection/player_01.png"),
            transform: Transform::from_scale(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }),
            ..default()
        })
        .insert(Direction::Stop)
        .insert(Player::new_at_pos(0.0, 0.0));
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
fn draw_sprite_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut xx = 0.0;
    let mut yy = 0.0;
    let txt_map = read_text("assets/maps/examples/002.txt");

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
                        x: xx * 45.0 - WINDOW_WIDTH / 2.0 - 45.0,
                        y: yy * 45.0 + WINDOW_HEIGHT / 2.0 - 45.0,
                        z: 0.,
                    },
                    scale: Vec3 {
                        x: 1.5,
                        y: 1.5,
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

fn pixel_to_coord() {}

fn coord_to_pixel() {}

use crate::MainCamera;

fn sprite_movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut sprite_query: Query<(&mut Direction, &mut Transform), Without<MainCamera>>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera_transform = q_camera.single_mut();
    for (mut dir, mut transform) in &mut sprite_query {
        if input.pressed(KeyCode::Up) {
            *dir = Direction::Down;
        } else if input.pressed(KeyCode::Down) {
            *dir = Direction::Up;
        } else if input.pressed(KeyCode::Left) {
            *dir = Direction::Left;
        } else if input.pressed(KeyCode::Right) {
            *dir = Direction::Right;
        } else {
            *dir = Direction::Stop;
        }

        if input.pressed(KeyCode::Space) {
            println!("{},{}", transform.translation.x, transform.translation.y);
        }

        let mut x_sign = 0.0;
        let mut y_sign = 0.0;

        match *dir {
            Direction::Up => {
                y_sign = -1.0;
            }
            Direction::Right => {
                x_sign = 1.0;
            }
            Direction::Down => {
                y_sign = 1.0;
            }
            Direction::Left => {
                x_sign = -1.0;
            }
            Direction::Stop => {}
        }

        let x_trans = 150. * time.delta_seconds() * x_sign;
        let y_trans = 150. * time.delta_seconds() * y_sign;

        transform.translation.x += x_trans;
        transform.translation.y += y_trans;
        camera_transform.translation.x += x_trans;
        camera_transform.translation.y += y_trans;
    }
}

fn map_char_to_path(s: &str) -> &'static str {
    let map_char_texture: HashMap<&str, &str> =
        HashMap::from([("#", "labyrinth_tile.png"), (" ", ""), ("\n", "")]);

    return map_char_texture[s];
}

pub struct PluginSpawn;

impl Plugin for PluginSpawn {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(draw_sprite_map)
            .add_system(sprite_movement);
    }
}
