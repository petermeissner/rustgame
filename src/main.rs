
use bevy::{prelude::*, render::texture::ImageSettings};


pub const CLEAR: Color = Color::rgb(0.1,0.1,0.1);

fn setup(
  mut commands: Commands,
) {
  
  commands.insert_resource(ClearColor(CLEAR));
  
  commands
    .spawn_bundle(Camera2dBundle::default());
}

fn render (
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut commands: Commands,
){
  let texture_handle = asset_server.load("Tilemap/dungeon.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
      texture_handle, 
      Vec2::new(16.0, 16.0), 
      29, 
      18,
      Vec2 { x: 1.0, y: 1.0 },
      Vec2 { x: 0.0, y: 0.0 }
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
  commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            sprite:TextureAtlasSprite { 
              index: 111, 
              ..Default::default()
            },
            ..default()
        });
}



fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(render)
        .run();
}


