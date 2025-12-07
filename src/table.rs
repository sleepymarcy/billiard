use avian3d::prelude::*;
use bevy::prelude::*;

use crate::config;

pub fn spawn(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut factory = WallFactory::new(
        config::TABLE_WIDTH,
        config::TABLE_LENGHT,
        config::WALL_THICKNESS,
        config::WALL_COLOR,
        meshes,
        materials,
    );

    // WallBundleFactory::get(&mut factory, WallId::Left);
    // factory.get(WallId::Left);

    commands.spawn(factory.get(WallId::East));
    commands.spawn(factory.get(WallId::West));
    commands.spawn(factory.get(WallId::North));
    commands.spawn(factory.get(WallId::South));
    commands.spawn(factory.get(WallId::Floor));
}

enum WallId {
    North,
    South,
    West,
    East,
    Floor,
}

struct WallFactory {
    table_width: f32,
    table_lenght: f32,
    thickness: f32,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<StandardMaterial>,
}
impl WallFactory {
    fn new(
        table_width: f32,
        table_lenght: f32,
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
            table_width,
            table_lenght,
            thickness,
            mesh_handle,
            material_handle,
        }
    }

    fn get(&mut self, wall_id: WallId) -> impl Bundle {
        let transform = match wall_id {
            WallId::North => Transform::from_xyz(0.0, 0.0, -self.table_lenght / 2.0),
            WallId::South => Transform::from_xyz(0.0, 0.0, self.table_lenght / 2.0),
            WallId::East => Transform::from_xyz(self.table_width / 2.0, 0.0, 0.0),
            WallId::West => Transform::from_xyz(-self.table_width / 2.0, 0.0, 0.0),
            WallId::Floor => Transform::from_xyz(0.0, -self.thickness, 0.0),
        };

        let (collider, transform) = match wall_id {
            WallId::North | WallId::South => (
                Collider::cuboid(self.table_width, self.thickness, self.thickness),
                transform.with_scale(Vec3::new(self.table_width / self.thickness, 1.0, 1.0)),
            ),
            WallId::East | WallId::West => (
                Collider::cuboid(self.thickness, self.table_lenght, self.thickness),
                transform.with_scale(Vec3::new(1.0, 1.0, self.table_lenght / self.thickness)),
            ),
            WallId::Floor => (
                Collider::cuboid(self.table_width, self.thickness, self.table_lenght),
                transform.with_scale(Vec3::new(
                    self.table_width / self.thickness,
                    1.0,
                    self.table_lenght / self.thickness,
                )),
            ),
        };

        let mesh = Mesh3d(self.mesh_handle.clone());
        let material = MeshMaterial3d(self.material_handle.clone());

        (mesh, material, collider, transform, RigidBody::Static)
    }
}
