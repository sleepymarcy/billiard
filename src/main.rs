use avian2d::prelude::*;
use bevy::prelude::*;

mod walls;

#[derive(Component)]
struct WhiteThing;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, (spawn_camera, spawn_circle, walls::spawn))
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh_handle = meshes.add(Circle { radius: 20.0 });
    let white_material_handle = color_materials.add(Color::WHITE);

    commands.spawn((
        Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            ..Default::default()
        },
        Mesh2d(mesh_handle),
        MeshMaterial2d(white_material_handle),
        WhiteThing,
        RigidBody::Dynamic,
        LinearVelocity(Vec2 { x: 320.0, y: 200.0 }),
        Collider::circle(20.0),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Max,
        },
        Friction {
            dynamic_coefficient: 0.0,
            static_coefficient: 0.0,
            combine_rule: CoefficientCombine::Min,
        },
    ));
}