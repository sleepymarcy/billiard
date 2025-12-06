use avian3d::{dynamics::solver::SolverConfig, prelude::*};
use bevy::prelude::*;

mod config;
mod walls;

#[derive(Component)]
struct WhiteThing;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(0.01),
        ))
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_light,
                spawn_circle,
                // spawn_markers,
                walls::spawn,
            ),
        )
        .insert_resource(SolverConfig {
            restitution_threshold: 0.1,
            ..default()
        })
        .run();
}

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

fn spawn_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 3.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z);
    commands.spawn((Camera3d::default(), transform));
}

fn spawn_light(mut commands: Commands) {
    let transform = Transform::from_xyz(-1.5, 1.0, -0.5);
    commands.spawn((PointLight::default(), transform));
}

fn spawn_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(Sphere {
        radius: config::BALL_RADIUS,
    });

    let material_handle = materials.add(Color::WHITE);

    commands.spawn((
        Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            ..Default::default()
        },
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        WhiteThing,
        RigidBody::Dynamic,
        LinearVelocity(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }),
        Collider::sphere(config::BALL_RADIUS),
        Restitution {
            coefficient: 0.3,
            combine_rule: CoefficientCombine::Max,
        },
        Friction {
            dynamic_coefficient: 0.0,
            static_coefficient: 0.0,
            combine_rule: CoefficientCombine::Min,
        },
    ));
}
