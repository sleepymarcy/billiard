use avian3d::prelude::*;
use bevy::prelude::*;

use crate::config;

#[derive(Component)]
pub struct Cue;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arrow_image: Handle<Image> = asset_server.load("arrow.png");
    let arrow_material = StandardMaterial {
        base_color_texture: Some(arrow_image),
        ..default()
    };
    let arrow_handle = materials.add(arrow_material);

    let mesh_handle = meshes.add(Sphere {
        radius: config::BALL_RADIUS,
    });

    commands.spawn((
        Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            ..Default::default()
        },
        Mesh3d(mesh_handle),
        MeshMaterial3d(arrow_handle),
        Cue,
        RigidBody::Dynamic,
        Collider::sphere(config::BALL_RADIUS),
        Restitution {
            coefficient: 0.9,
            combine_rule: CoefficientCombine::Max,
        },
        Friction {
            dynamic_coefficient: 0.9,
            static_coefficient: 1.0,
            combine_rule: CoefficientCombine::Max,
        },
        LinearDamping(0.55),
        AngularDamping(0.5),
    ));
}
