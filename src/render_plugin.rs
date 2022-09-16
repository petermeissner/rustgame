

use bevy::prelude::*;



struct TextureConfig {
  tile_size: f32,
  columns: usize,
  rows: usize,
  padding: f32,
}
impl TextureConfig {
  fn new(tile_size: f32, columns: usize, rows: usize, padding: f32) -> TextureConfig {
      return TextureConfig {
          tile_size: tile_size,
          columns: columns,
          rows: rows,
          padding: padding,
      };
  }
}



fn tt_texture_atlas(
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  tc: TextureConfig,
) 
-> bevy::prelude::Handle<bevy::prelude::TextureAtlas> 
{
  let texture_handle = asset_server.load("Tilemap/tilemap.png");
  let texture_atlas = TextureAtlas::from_grid_with_padding(
      texture_handle,
      Vec2::new(tc.tile_size, tc.tile_size),
      tc.columns,
      tc.rows,
      Vec2 {
          x: tc.padding,
          y: tc.padding,
      },
      Vec2 { x: 0.0, y: 0.0 },
  );
  let texture_atlas_handle = texture_atlases.add(texture_atlas);
  return texture_atlas_handle;
}



fn tt_render_i(
  texture_atlas_handle: bevy::prelude::Handle<bevy::prelude::TextureAtlas>,
  i: usize,
  x: f32,
  y: f32,
  scale: f32
) 
-> SpriteSheetBundle 
{
  SpriteSheetBundle {
      texture_atlas: texture_atlas_handle,
      transform: Transform {
        translation: Vec3::new(x, y, 0.0),
        rotation: Quat::from_rotation_x(0.0),
        scale: Vec3::new(scale, scale,scale),
    },
      sprite: TextureAtlasSprite {
          index: i,
          ..Default::default()
      },
      ..default()
  }
}



fn render(
  asset_server: Res<AssetServer>,
  texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut commands: Commands,
) {
  let tc = TextureConfig::new(16.0, 27, 19, 1.0);

  let th = tt_texture_atlas(asset_server, texture_atlases, tc);

  commands.spawn_bundle(tt_render_i(th, 24, 0.0,0.0,4.0));
}


pub struct RenderPlugin;

impl Plugin for RenderPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(render);
  }
}


