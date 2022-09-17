

use bevy::{prelude::*, window::WindowMode, window::WindowPosition};


const RESOLUTION: f32 = 72.0;
const WINDOW_HEIGHT: f32 = 200.0;


pub struct WindowPlugin;

impl Plugin for WindowPlugin {
  
  fn build(&self, app: &mut App) {
    app
      .insert_resource(
        WindowDescriptor {
          width: WINDOW_HEIGHT * RESOLUTION,
          height: WINDOW_HEIGHT,
          title: String::from("dings"),
          position: WindowPosition::Automatic,
          resizable: false,
          resize_constraints: bevy::window::WindowResizeConstraints {
              min_width: WINDOW_HEIGHT * RESOLUTION,
              max_width: WINDOW_HEIGHT * RESOLUTION,
              min_height: WINDOW_HEIGHT,
              max_height: WINDOW_HEIGHT,
          },
          mode: WindowMode::Windowed,
          ..Default::default()
        }
      );
  
  }
  
}


