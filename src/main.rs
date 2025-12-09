use avian3d::{dynamics::solver::SolverConfig, prelude::*};
use bevy::prelude::*;

mod ball;
mod camera;
mod config;
mod light;
mod table;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(
            Startup,
            (
                camera::spawn,
                light::spawn_table_light,
                ball::spawn,
                // spawn_markers,
                table::spawn,
                spawn_mouse_controller,
            ),
        )
        .add_systems(Update, (camera::follow_cue_ball, get_mouse_delta))
        .insert_resource(SolverConfig {
            restitution_threshold: 0.01,
            ..default()
        })
        .run();
}

#[derive(Component)]
struct LmbPast {
    pub pressed: bool,
}

#[derive(Component)]
struct MouseVector {
    pub beginning: Vec2,
    pub end: Vec2,
}

// TODO: improve spawn_mouse_controller
// 1) use Resource to store intermediate state instead of spawning an entity for this purpose
fn spawn_mouse_controller(mut commands: Commands) {
    commands.spawn((
        LmbPast { pressed: false },
        MouseVector {
            beginning: Vec2::ZERO,
            end: Vec2::ZERO,
        },
    ));
}

// TODO: improve get_mouse_delta
// 1) instead of Window and cursor position use AccumulatedMouseMotion to get mouse delta
// 2) use just_pressed instead of pressed - eliminates the need to store past LMB state
fn get_mouse_delta(
    mouse_controller: Single<(&mut LmbPast, &mut MouseVector)>,
    window: Single<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let lmb_pressed = mouse_buttons.pressed(MouseButton::Left);
    mouse_buttons.just_pressed(MouseButton::Left);
    let (mut lmb_past, mut mouse_vector) = mouse_controller.into_inner();
    let mouse_position = window.cursor_position();

    if lmb_pressed && !lmb_past.pressed && mouse_position.is_some() {
        mouse_vector.beginning = mouse_position.unwrap();
    }

    if !lmb_pressed && lmb_past.pressed && mouse_position.is_some() {
        mouse_vector.end = mouse_position.unwrap();
        let mouse_delta = mouse_vector.end - mouse_vector.beginning;
        println!("{mouse_delta:?}");
    }

    lmb_past.pressed = lmb_pressed;
}

#[allow(dead_code)]
fn spawn_markers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_length(0.5));

    let north = materials.add(Color::srgb(0.298, 0.757, 0.827));
    let south = materials.add(Color::srgb(0.749, 0.725, 0.055));
    let east = materials.add(Color::srgb(0.682, 0.039, 0.039));
    let west = materials.add(Color::srgb(0.11, 0.502, 0.02));

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(north),
        Transform::from_xyz(0.0, 0.0, -0.5),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(south),
        Transform::from_xyz(0.0, 0.0, 0.5),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(east),
        Transform::from_xyz(0.5, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(west),
        Transform::from_xyz(-0.5, 0.0, 0.0),
    ));
}
