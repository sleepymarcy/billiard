use bevy::prelude::*;

use crate::ball::Cue;

pub fn spawn(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 3.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z);
    commands.spawn((Camera3d::default(), transform));
}

pub fn follow_cue_ball(
    mut camera_transform: Single<&mut Transform, (With<Camera3d>, Without<Cue>)>,
    ball_transform: Single<&Transform, With<Cue>>,
) {
    camera_transform.translation.x = ball_transform.translation.x;
    camera_transform.translation.z = ball_transform.translation.z;
}

