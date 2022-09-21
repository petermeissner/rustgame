use bevy::{prelude::*, transform};

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
) -> bevy::prelude::Handle<bevy::prelude::TextureAtlas> {
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
    let texture_atlas_handle = texture_atlases.add(texture_atlas).clone();
    return texture_atlas_handle;
}

fn tt_render_i(
    texture_atlas_handle: bevy::prelude::Handle<bevy::prelude::TextureAtlas>,
    i: usize,
    x: f32,
    y: f32,
    scale: f32,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            rotation: Quat::from_rotation_x(0.0),
            scale: Vec3::new(scale, scale, scale),
        },
        sprite: TextureAtlasSprite {
            index: i,
            ..Default::default()
        },
        ..default()
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Scenery;

#[derive(Component)]
pub struct Name(String);

fn spawn_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture_handle = asset_server.load("tree_01.png");

    commands
        // .spawn_bundle(tt_render_i(th.clone(), 24, 0.0, 0.0, 4.0))
        .spawn_bundle(SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 100.,
                    y: 100.,
                    z: 17.,
                },
                // rotation: Quat::from_rotation_x(PI),
                scale: Vec3::new(2.0, 2.0, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Name(String::from("redTree")));

    commands
        // .spawn_bundle(tt_render_i(th.clone(), 24, 0.0, 0.0, 4.0))
        .spawn_bundle(SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 150.,
                    y: 100.,
                    z: 17.,
                },
                // rotation: Quat::from_rotation_x(PI),
                scale: Vec3::new(2.0, 2.0, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name(String::from("reddishTree")));
}

fn spawn_trees(
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let tc = TextureConfig::new(16.0, 27, 19, 1.0);
    let th = tt_texture_atlas(asset_server, texture_atlases, tc);

    commands
        .spawn_bundle(tt_render_i(th.clone(), 27 * 12 + 22, -40.0, -30.0, 4.0))
        .insert(Scenery)
        .insert(Name(String::from("Tree")));
}

pub struct PluginSpawn;

impl Plugin for PluginSpawn {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_trees);
    }
}
