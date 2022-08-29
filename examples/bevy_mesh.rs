// source: https://github.com/bevyengine/bevy/blob/latest/examples/2d/mesh2d_vertex_color_texture.rs

// Shows how to render a polygonal [`Mesh`], generated from a [`Quad`] primitive, in a 2D scene.
// Adds a texture and colored vertices, giving per-vertex tinting.

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
    // Load the Bevy logo as a texture
    let texture_handle = asset_server.load("flor.png");
    
    // Build a default quad mesh-
    let mesh = Mesh::from(shape::RegularPolygon::default());
    
    // Spawn
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(texture_handle)),
        ..default()
    });
}
