use avian3d::{dynamics::solver::SolverConfig, prelude::*};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

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
        .add_systems(
            Update,
            (accumulate_drag_input, ball::apply_velocity).chain(), // ensures that systems run in order //
        )
        .insert_resource(SolverConfig {
            restitution_threshold: 0.01,
            ..default()
        })
        .insert_resource(MouseDelta { delta: Vec2::ZERO })
        .run();
}

#[derive(Resource)]
pub struct MouseDelta {
    delta: Vec2,
}

fn accumulate_drag_input(
    acumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut drag_accumulator: ResMut<MouseDelta>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if mouse_buttons.pressed(MouseButton::Left) {
        drag_accumulator.delta += acumulated_mouse_motion.delta;
    }
}
