// #![allow(dead_code)]
// #![allow(unused_variables)]

use bevy::{prelude::*, render::texture::ImageSettings, window::WindowResizeConstraints};

pub mod plugin_spawn;
pub mod plugin_window;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_WIDTH: f32 = 1300.0;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            position: WindowPosition::At(Vec2 { x: 1.0, y: 1.0 }),
            resize_constraints: WindowResizeConstraints {
                max_height: WINDOW_HEIGHT,
                max_width: WINDOW_WIDTH,
                min_height: WINDOW_HEIGHT,
                min_width: WINDOW_WIDTH,
            },
            // scale_factor_override:,
            title: "EsCaPaTist".to_string(),
            // present_mode:,
            // resizable:,
            // decorations:,
            // cursor_visible:,
            // cursor_locked:,
            // mode:,
            // transparent:,
            // canvas:,
            // fit_canvas_to_parent:,
            ..default()
        })
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        // .add_plugin(plugin_window::WindowPlugin)
        .add_plugin(plugin_spawn::PluginSpawn)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(CLEAR));
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}
