use bevy::{prelude::*, render::texture::ImageSettings, ecs::system::Command};

use rand::Rng;

pub mod render_plugin;
use render_plugin::RenderPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(CLEAR));

    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .run();
}


#[derive(Component)]
struct Player {
  sprite_i: usize,
  x: f32,
  y: f32
}


fn spawn_player(mut commands: Commands) {
    commands
      .spawn()
      .insert(Player {sprite_i: 24, x: 0.0, y: 0.0})
    ;
    
    
}


fn move_player (
  input: Res<Input<KeyCode>>,
  mut query: Query<&mut Player>,
){
  let mut rng = rand::thread_rng();
  
  for mut p in &mut query {
    p.x = rng.gen_range(0.0..1000.0)-500.0;
    p.y = rng.gen_range(0.0..1000.0)-500.0;
  }
}
