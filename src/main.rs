use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct WhiteThing;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, (spawn_camera, spawn_circle, spawn_walls))
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

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh_handle = meshes.add(Rectangle {
        // half_size: Vec2 { x: 10.0, y: 400.0 },
        half_size: Vec2 { x: 10.0, y: 10.0 },
    });
    let material_handle = color_materials.add(Color::BLACK);

    commands.spawn((
        Transform::from_xyz(640.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 40.0, 1.0)),
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        RigidBody::Static,
        Collider::rectangle(20.0, 800.0),
    ));

    commands.spawn((
        Transform::from_xyz(-640.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 40.0, 1.0)),
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        RigidBody::Static,
        Collider::rectangle(20.0, 800.0),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 360.0, 0.0).with_scale(Vec3::new(64.0, 1.0, 1.0)),
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        RigidBody::Static,
        Collider::rectangle(1280.0, 20.0),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, -360.0, 0.0).with_scale(Vec3::new(64.0, 1.0, 1.0)),
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        RigidBody::Static,
        Collider::rectangle(1280.0, 20.0),
    ));
}