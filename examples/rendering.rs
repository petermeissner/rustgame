// Project Imports
pub(crate) use bevy::prelude::*;
use bevy::sprite;
use hello_plugin::HelloPlugin;
use window_plugin::WindowPlugin;


// # Project Modules
mod items;
pub mod hello_plugin;
pub mod window_plugin;



fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    //.add_plugin(HelloPlugin)
    .add_plugin(WindowPlugin)
    .add_startup_system(setup)
    .run();
}


fn setup(
  mut commands: Commands, 
      asset_server: Res<AssetServer>
) {
  commands.spawn_bundle(Camera2dBundle::default());
    
  for yi in 0..=2 {
    for xi in 0..=2 {
      
      // do size and location calculation
      let sprite_size = 200 as f32;
      let x_trans = sprite_size * (xi as f32);
      let y_trans = sprite_size * (yi as f32);
      let trans_vec = Vec3::new(x_trans, y_trans, 0.);
      
      // Rectangle with texture
      commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("dungeon_tiles.png"),
        transform: 
          Transform::from_translation(trans_vec),
        sprite: Sprite {
            custom_size: Some(Vec2::new(sprite_size, sprite_size)),
            ..default()
        },
        ..default()
      });
    }
  }
}
