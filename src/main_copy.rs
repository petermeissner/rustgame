// Project Imports
pub(crate) use bevy::prelude::*;
// use bevy::{sprite, window::PresentMode};
// use hello_plugin::HelloPlugin;
// use window_plugin::WindowPlugin;


// # Project Modules
mod items;
// pub mod hello_plugin;
pub mod window_plugin;


// 
pub const CLEAR: Color = Color::rgb(0.1,0.1,0.1);


fn spawn_camera (mut commands: Commands) {
  let mut camera = Camera2dBundle::default();
  commands.spawn_bundle(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlas_res: ResMut<Assets<TextureAtlas>>
){
  
  let texture_handle = asset_server.load("Ascii.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
      texture_handle, 
      Vec2::new(9.0, 9.0), 
      16, 
      16,
      Vec2 { x: 1.0, y: 1.0 },
      Vec2 { x: 0.0, y: 0.0 }
    );
    let texture_atlas_handle = texture_atlas_res.add(texture_atlas);
  
  commands.insert_resource(AsciiSheet(texture_atlas_handle));
}

fn spawn_player(mut commands: Commands, sheet: Res<AsciiSheet>) {
  let mut sprite = TextureAtlasSprite::new(1);
  sprite.color = Color::rgb(0.3, 0.3, 0.9);
  sprite.custom_size = Some(Vec2::splat(1.0));
  
  fn make_sprite (index: usize) -> bevy::prelude::TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));
    
    return sprite;
  }
  
  commands.spawn_bundle(
    SpriteSheetBundle {
      sprite: make_sprite(1),
      texture_atlas: sheet.0.clone(),
      transform: Transform { 
        translation: Vec3 { x: 0.0, y: 0.0, z: 900.0 }, 
        ..Default::default() 
      },
      ..Default::default()
    }
  )
  .insert(Name::new("Player"));
}

fn main() {
  App::new()
    // .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
      title: "RustGame".to_string(),
      width: 900.,
      height: 600.,
      cursor_visible: true,
      ..default()
    })
    .add_plugins(DefaultPlugins)
    //.add_plugin(HelloPlugin)
    // .add_plugin(WindowPlugin)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_player)
    // .add_startup_system(setup)
    .add_startup_system_set_to_stage(
      StartupStage::PreStartup, 
      SystemSet::new()
        .with_system(load_ascii)
    )
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
