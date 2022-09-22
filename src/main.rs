// #![allow(dead_code)]
// #![allow(unused_variables)]


use bevy::{prelude::*, render::texture::ImageSettings};

pub mod plugin_spawn;
use plugin_spawn::PluginSpawn;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(PluginSpawn)
        .add_startup_system(setup)
        .run();
}



fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(CLEAR));
    commands.spawn_bundle(Camera2dBundle::default());
}
