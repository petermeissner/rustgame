// adaption of source: 
// Shows how to render simple primitive shapes with a single color.


use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    // Circle
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
    
    
    for yy in 0..2 {
      for xx in 0..2 {
        // Rectangle with texture
        commands.spawn_bundle(SpriteBundle {
          texture: asset_server.load("flor.png"),
          transform: 
            Transform::from_translation(
              Vec3::new(
                100.0*(xx as f32)+200.0, 
                100.0*(yy as f32), 
                0.
              )
            ),
          sprite: Sprite {
              color: Color::rgb(0.25, 0.25, 0.75),
              custom_size: Some(Vec2::new(100.0, 100.0)),
              ..default()
          },
          ..default()
        });
      }
    }
    
    
}
