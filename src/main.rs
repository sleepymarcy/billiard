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
    const WALL_THICKNESS: f32 = 20.0;
    const WINDOW_WIDTH: f32 = 1280.0;
    const WINDOW_HEIGHT: f32 = 720.0;

    // TODO: Move to a factory constructor
    let mesh_handle = meshes.add(Rectangle {
        half_size: Vec2 {
            x: WALL_THICKNESS / 2.0,
            y: WALL_THICKNESS / 2.0,
        },
    });
    let material_handle = color_materials.add(Color::BLACK);

    let mut factory = WallBundleFactory {
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        wall_thickness: WALL_THICKNESS,
        wall_mesh: mesh_handle,
        wall_material: material_handle,
    };

    commands.spawn(factory.get(WallId::Right));
    commands.spawn(factory.get(WallId::Left));
    commands.spawn(factory.get(WallId::Upper));
    commands.spawn(factory.get(WallId::Lower));
}

enum WallId {
    Upper,
    Lower,
    Left,
    Right,
}

struct WallBundleFactory {
    window_width: f32,
    window_height: f32,
    wall_thickness: f32,
    wall_mesh: Handle<Mesh>,
    wall_material: Handle<ColorMaterial>,
}
impl WallBundleFactory {
    fn get(&mut self, wall: WallId) -> impl Bundle {
        let collider = match wall {
            WallId::Upper | WallId::Lower => {
                Collider::rectangle(self.window_width, self.wall_thickness)
            }
            WallId::Left | WallId::Right => {
                Collider::rectangle(self.wall_thickness, self.window_height)
            }
        };

        let transform = match wall {
            WallId::Upper => Transform::from_xyz(0.0, self.window_height / 2.0, 0.0),
            WallId::Lower => Transform::from_xyz(0.0, -self.window_height / 2.0, 0.0),
            WallId::Left => Transform::from_xyz(-self.window_width / 2.0, 0.0, 0.0),
            WallId::Right => Transform::from_xyz(self.window_width / 2.0, 0.0, 0.0),
        };

        let transform = match wall {
            WallId::Upper | WallId::Lower => {
                transform.with_scale(Vec3::new(self.window_width / self.wall_thickness, 1.0, 1.0))
            }
            WallId::Left | WallId::Right => transform.with_scale(Vec3::new(
                1.0,
                self.window_height / self.wall_thickness,
                1.0,
            )),
        };

        let mesh = Mesh2d(self.wall_mesh.clone());
        let material = MeshMaterial2d(self.wall_material.clone());

        (mesh, material, collider, transform, RigidBody::Static)
    }
}
