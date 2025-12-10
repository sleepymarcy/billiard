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
                table::spawn,
            ),
        )
        .add_systems(Update, (camera::follow_cue_ball, get_mouse_delta))
        .insert_resource(SolverConfig {
            restitution_threshold: 0.01,
            ..default()
        })
        .insert_resource(MouseController {
            lmb_past: LmbPast { pressed: false },
            mouse_vector: MouseVector {
                beginning: Vec2::ZERO,
                end: Vec2::ZERO,
            },
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

#[derive(Resource)]
struct MouseController {
    lmb_past: LmbPast,
    mouse_vector: MouseVector,
}


// TODO: improve get_mouse_delta
// 1) instead of Window and cursor position use AccumulatedMouseMotion to get mouse delta
// 2) use just_pressed instead of pressed - eliminates the need to store past LMB state

fn get_mouse_delta(
    mut mouse_controller: ResMut<MouseController>,
    window: Single<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let lmb_pressed = mouse_buttons.pressed(MouseButton::Left);
    mouse_buttons.just_pressed(MouseButton::Left);
    let mouse_position = window.cursor_position();

    if lmb_pressed && !mouse_controller.lmb_past.pressed && mouse_position.is_some() {
        mouse_controller.mouse_vector.beginning = mouse_position.unwrap();
    }

    if !lmb_pressed && mouse_controller.lmb_past.pressed && mouse_position.is_some() {
        mouse_controller.mouse_vector.end = mouse_position.unwrap();
        let mouse_delta = mouse_controller.mouse_vector.end - mouse_controller.mouse_vector.beginning;
        println!("{mouse_delta:?}");
    }

    mouse_controller.lmb_past.pressed = lmb_pressed;
}
