use avian2d::prelude::*;
use bevy::prelude::*;

pub fn spawn(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    color_materials: ResMut<Assets<ColorMaterial>>,
) {
    const WALL_THICKNESS: f32 = 20.0;
    const WINDOW_WIDTH: f32 = 1280.0;
    const WINDOW_HEIGHT: f32 = 720.0;
    const WALL_COLOR: Color = Color::srgb(0., 0., 0.);

    let mut factory = WallBundleFactory::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WALL_THICKNESS,
        WALL_COLOR,
        meshes,
        color_materials,
    );

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
    thickness: f32,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}
impl WallBundleFactory {
    fn new(
        window_width: f32,
        window_height: f32,
        thickness: f32,
        color: Color,
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let mesh_handle = meshes.add(Rectangle {
            half_size: Vec2 {
                x: thickness / 2.0,
                y: thickness / 2.0,
            },
        });

        let material_handle = color_materials.add(color);

        Self {
            window_width,
            window_height,
            thickness,
            mesh_handle,
            material_handle,
        }
    }

    fn get(&mut self, wall: WallId) -> impl Bundle {
        let collider = match wall {
            WallId::Upper | WallId::Lower => Collider::rectangle(self.window_width, self.thickness),
            WallId::Left | WallId::Right => Collider::rectangle(self.thickness, self.window_height),
        };

        let transform = match wall {
            WallId::Upper => Transform::from_xyz(0.0, self.window_height / 2.0, 0.0),
            WallId::Lower => Transform::from_xyz(0.0, -self.window_height / 2.0, 0.0),
            WallId::Left => Transform::from_xyz(-self.window_width / 2.0, 0.0, 0.0),
            WallId::Right => Transform::from_xyz(self.window_width / 2.0, 0.0, 0.0),
        };

        let transform = match wall {
            WallId::Upper | WallId::Lower => {
                transform.with_scale(Vec3::new(self.window_width / self.thickness, 1.0, 1.0))
            }
            WallId::Left | WallId::Right => {
                transform.with_scale(Vec3::new(1.0, self.window_height / self.thickness, 1.0))
            }
        };

        let mesh = Mesh2d(self.mesh_handle.clone());
        let material = MeshMaterial2d(self.material_handle.clone());

        (mesh, material, collider, transform, RigidBody::Static)
    }
}
