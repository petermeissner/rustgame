// Project Imports
pub(crate) use bevy::prelude::*;
use hello_plugin::HelloPlugin;
use window_plugin::WindowPlugin;


// # Project Modules
mod items;
pub mod hello_plugin;
pub mod window_plugin;



fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .add_plugin(WindowPlugin)
    .add_startup_system(setup)
    .run();
}


fn setup(
  mut commands: Commands, 
      asset_server: Res<AssetServer>
) {
  commands.spawn_bundle(Camera2dBundle::default());
  commands.spawn_bundle(
    SpriteBundle {
      texture: asset_server.load("flor.png"),
      ..default()
    }
  );
}
