use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Move;
