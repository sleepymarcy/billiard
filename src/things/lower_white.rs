use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct WhiteThing;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const RADIUS: f32 = 20.0;

    commands.spawn((
        WhiteThing,
        RigidBody::Dynamic,
        Mesh2d(meshes.add(Circle::new(RADIUS))),
        Collider::circle(RADIUS),
        MeshMaterial2d(materials.add(Color::default())),
        LinearVelocity(Vector::new(0.0, -1.0)),
        Transform::default(),
    ));
}
