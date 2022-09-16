use bevy::{prelude::*, render::texture::ImageSettings};


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
        .run();
}
