// mod components;
mod input;
mod things;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use things::RedThing;
use things::WhiteThing;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EnhancedInputPlugin,
            PhysicsPlugins::default(),
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_input_context::<RedThing>()
        .add_systems(
            Startup,
            (
                camera_setup,
                things::lower_white::spawn,
                things::upper_red::spawn,
            ),
        )
        // .add_systems(Update, (
        // debug_print,
        //     apply_forces,
        // ))
        // .add_observer(things::moving::controls)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn debug_print(collisions: Collisions, static_thing: Query<Entity, With<WhiteThing>>) {
    if let Ok(this) = static_thing.single() {
        let collisions = collisions.entities_colliding_with(this);
        for other in collisions {
            println!("Collision detected between {:?} and {:?}", this, other);
        }
    }
}

fn apply_forces(mut query: Query<Forces, With<WhiteThing>>) {
    for mut forces in &mut query {
        // Apply a force of 10 N in the positive Y direction to the entity.
        forces.apply_force(Vec2::new(0.0, 10000.0));
    }
}
