use bevy::{prelude::*, render::texture::ImageSettings};

pub mod plugin_spawn;
use plugin_spawn::{PluginSpawn};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);


fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(PluginSpawn)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_trees)
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

fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(CLEAR));
    commands.spawn_bundle(Camera2dBundle::default());
}


fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
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

fn spawn_trees(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.insert_resource(ClearColor(CLEAR));

  let mut xx = 0.0;
  let mut yy = 0.0;
  for c in map_000.chars(){
    xx = xx + 1.0;
    if c == '\n' {
      yy = yy + 1.0;
    }
    commands.spawn_bundle(SpriteBundle {
      texture: asset_server.load("tree_01.png"),
      transform: Transform {
          translation: Vec3 {
              x: xx * 25.0,
              y: yy * 25.0,
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



pub const map_000: &str = "
^ #
";

pub const map_001: &str = "
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
^                                  ^
^ ^ ^ ^ ^ ^                        ^
^ ^ ^ ^ ^ ^                        ^
^                                  ^
^ ########                         ^
^    #                             ^
^    #                             ^
^                                  ^
^                                  ^
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
";
