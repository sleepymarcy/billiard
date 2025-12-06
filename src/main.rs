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
        .add_systems(Startup, (spawn_camera, spawn_circle, walls::spawn))
        // .insert_resource(Gravity(Vec3::ZERO))
        .insert_resource(SolverConfig {
            restitution_threshold: 0.1,
            ..default()
        })
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((Camera3d::default(), transform));
    commands.spawn((PointLight::default(), transform));
}

fn spawn_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let mesh_handle = meshes.add(Circle { radius: 20.0 });
    // let white_material_handle = color_materials.add(Color::WHITE);

    commands.spawn((
        Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            ..Default::default()
        },
        // Mesh2d(mesh_handle),
        // MeshMaterial2d(white_material_handle),
        Mesh3d(meshes.add(Sphere {
            radius: config::BALL_RADIUS,
        })),
        MeshMaterial3d(materials.add(Color::WHITE)),
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
