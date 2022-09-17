use bevy::{prelude::*, render::texture::ImageSettings};

pub mod plugin_spawn;
use plugin_spawn::PluginSpawn;
use plugin_spawn::Player;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(CLEAR));

    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(PluginSpawn)
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}



fn move_player (
  input: Res<Input<KeyCode>>,
  mut query: Query<(&mut Transform, With<Player>)>,
){
  for (mut trans, _) in &mut query {
    if        input.pressed(KeyCode::Left) {
      trans.translation.x +=  3.0;
    } else if input.pressed(KeyCode::Right) {
      trans.translation.x +=  -3.0;
    } 
    
    if input.pressed(KeyCode::Up) {
      trans.translation.y += -3.0;
    } else if input.pressed(KeyCode::Down) {
      trans.translation.y +=  3.0;
    }
  }
}
