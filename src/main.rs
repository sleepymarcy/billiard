use avian3d::{dynamics::solver::SolverConfig, prelude::*};
use bevy::prelude::*;

mod ball;
mod camera;
mod config;
mod light;
mod table;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(
            Startup,
            (
                camera::spawn,
                light::spawn_table_light,
                ball::spawn,
                // spawn_markers,
                table::spawn,
            ),
        )
        .add_systems(Update, camera::follow_cue_ball)
        .insert_resource(SolverConfig {
            restitution_threshold: 0.01,
            ..default()
        })
        .run();
}

#[allow(dead_code)]
fn spawn_markers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_length(0.5));

    let north = materials.add(Color::srgb(0.298, 0.757, 0.827));
    let south = materials.add(Color::srgb(0.749, 0.725, 0.055));
    let east = materials.add(Color::srgb(0.682, 0.039, 0.039));
    let west = materials.add(Color::srgb(0.11, 0.502, 0.02));

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(north),
        Transform::from_xyz(0.0, 0.0, -0.5),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(south),
        Transform::from_xyz(0.0, 0.0, 0.5),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(east),
        Transform::from_xyz(0.5, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(west),
        Transform::from_xyz(-0.5, 0.0, 0.0),
    ));
}
