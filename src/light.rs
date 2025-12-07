use bevy::prelude::*;

pub fn spawn_table_light(mut commands: Commands) {
    let transform = Transform::from_xyz(-1.5, 1.0, -0.5);
    commands.spawn((PointLight::default(), transform));
}
