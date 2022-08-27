pub(crate) use bevy::prelude::*;
mod items;
pub mod hello_plugin;


fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(hello_plugin::HelloPlugin)
    .run();
}
