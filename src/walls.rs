use avian3d::prelude::*;
use bevy::prelude::*;

use crate::config::{TABLE_HEIGHT, TABLE_WIDTH, WALL_COLOR, WALL_THICKNESS};

pub fn spawn(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    // color_materials: ResMut<Assets<ColorMaterial>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    // const WINDOW_WIDTH: f32 = 1280.0;
    // const WINDOW_HEIGHT: f32 = 720.0;

    let mut factory = WallBundleFactory::new(
        TABLE_WIDTH,
        TABLE_HEIGHT,
        WALL_THICKNESS,
        WALL_COLOR,
        meshes,
        materials,
    );

    // WallBundleFactory::get(&mut factory, WallId::Left);
    // factory.get(WallId::Left);

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
    material_handle: Handle<StandardMaterial>,
}
impl WallBundleFactory {
    fn new(
        window_width: f32,
        window_height: f32,
        thickness: f32,
        color: Color,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mesh_handle = meshes.add(Cuboid {
            half_size: Vec3::ONE * thickness / 2.0,
        });

        let material_handle = materials.add(color);

        Self {
            window_width,
            window_height,
            thickness,
            mesh_handle,
            material_handle,
        }
    }

    fn get(&mut self, wall_id: WallId) -> impl Bundle {
        let collider = match wall_id {
            WallId::Upper | WallId::Lower => Collider::cuboid(self.window_width, self.thickness, self.thickness),
            WallId::Left | WallId::Right => Collider::cuboid(self.thickness, self.window_height, self.thickness),
        };

        let transform = match wall_id {
            WallId::Upper => Transform::from_xyz(0.0, self.window_height / 2.0, 0.0),
            WallId::Lower => Transform::from_xyz(0.0, -self.window_height / 2.0, 0.0),
            WallId::Left => Transform::from_xyz(-self.window_width / 2.0, 0.0, 0.0),
            WallId::Right => Transform::from_xyz(self.window_width / 2.0, 0.0, 0.0),
        };

        let transform = match wall_id {
            WallId::Upper | WallId::Lower => {
                transform.with_scale(Vec3::new(self.window_width / self.thickness, 1.0, 1.0))
            }
            WallId::Left | WallId::Right => {
                transform.with_scale(Vec3::new(1.0, self.window_height / self.thickness, 1.0))
            }
        };

        let mesh = Mesh3d(self.mesh_handle.clone());
        let material = MeshMaterial3d(self.material_handle.clone());

        (mesh, material, collider, transform, RigidBody::Static)
    }
}
