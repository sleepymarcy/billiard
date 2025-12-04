use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::input::Move;

#[derive(Component)]
pub struct RedThing;

pub fn controls(movement: On<Fire<Move>>, mut moving_thing: Query<&mut Transform, With<RedThing>>) {
    const THING_SPEED: f32 = 1.0;

    if let Ok(mut thing_transform) = moving_thing.single_mut() {
        thing_transform.translation += movement.value.normalize_or_zero().extend(0.0) * THING_SPEED;
    }
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const RADIUS: f32 = 20.0;

    commands.spawn((
        RedThing,
        RigidBody::Dynamic,
        Mesh2d(meshes.add(Circle::new(RADIUS))),
        Collider::circle(RADIUS),
        // CollisionEventsEnabled,
        MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
        LinearVelocity(Vector::new(0.0, -1.0)),
        // Restitution::new(0.4),
        Transform::from_translation(Vec3::new(0.0, 20.0, 0.0)),
        actions!(
            RedThing[(
                Action::<Move>::new(),
                Bindings::spawn(Cardinal::wasd_keys()) // don't know what to do here :(
            )]
        ),
        // Actions::<MovingThing>::spawn(Spawn((
        //     Action::<Move>::new(),
        //     Bindings::spawn(Cardinal::wasd_keys()),
        // ))),
    ));
}
