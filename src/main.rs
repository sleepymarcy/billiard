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
            (accumulate_drag_input, apply_drag_to_cue_ball).chain(), // ensures that systems run in order //
        )
        .insert_resource(SolverConfig {
            restitution_threshold: 0.01,
            ..default()
        })
        .insert_resource(MouseDelta { delta: Vec2::ZERO })
        .run();
}

#[derive(Resource)]
struct MouseDelta {
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

fn apply_drag_to_cue_ball(
    mut drag_accumulator: ResMut<MouseDelta>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    ball_velocity: Single<&mut LinearVelocity, With<ball::Cue>>,
) {
    if mouse_buttons.just_released(MouseButton::Left) {
        let scaled_delta = drag_accumulator.delta / 100.0;
        let velocity = -scaled_delta.extend(0.0).xzy();

        *ball_velocity.into_inner() = LinearVelocity(velocity);

        println!("{:?}", drag_accumulator.delta);
        drag_accumulator.delta = Vec2::ZERO;
    }
}
