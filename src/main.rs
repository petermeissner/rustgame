use bevy::{prelude::*, render::texture::ImageSettings};

pub mod plugin_spawn;
use plugin_spawn::{PluginSpawn};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn setup(mut commands: Commands) {
    // commands.insert_resource(ClearColor(CLEAR));

    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(PluginSpawn)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(test_spawn)
        .run();
}


use crate::plugin_spawn::Player;
use crate::plugin_spawn::Scenery;


fn move_player (
  input: Res<Input<KeyCode>>,
  mut query: Query<(&mut Transform), (Without<Player>, With<Scenery>) >,
){
  
  // let mut i: i128 =0;
   let speed: f32 = 10.0;
   
  for (mut trans) in query.iter_mut() {
    // i += 1;
    // println!("{i} : {x},{y} : ");
    
    if input.pressed(KeyCode::Left) {
      trans.translation.x +=  -speed;
    } else if input.pressed(KeyCode::Right) {
      trans.translation.x +=  speed;
    } 
    
    if input.pressed(KeyCode::Up) {
      trans.translation.y += speed;
    } else if input.pressed(KeyCode::Down) {
      trans.translation.y +=  speed;
    }
  }
}



  
  
fn test_spawn(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
  asset_server: Res<AssetServer>
) {
let texture_handle = asset_server.load("Tilemap/tilemap.png");
let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 4, 4).clone();
let render_me = texture_atlases.add(texture_atlas);


  commands.spawn_bundle(SpriteSheetBundle {
      texture_atlas: render_me.clone(),
      transform: Transform {
          translation: Vec3::new(-50., -50., 10.),
          scale: Vec3::new(3.0, 3.0, 1.),
          ..Default::default()
      },
      ..Default::default()
  });
}
