//! System to move the paddles every frame.

// Standard libraries
use amethyst::{
	core::Transform,
	ecs::{Join, Read, ReadStorage, System, WriteStorage},
	input::InputHandler,
};

// Project libraries
use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// Constants
const MOVEMENT_SCALE: f32 = 1.5;


// System to move the paddle.
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Paddle>,
		Read<'s, InputHandler<String, String>>,
	);

	// Move the paddle a fixed scaled amount.
	fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
		for (transform, paddle) in (&mut transforms, &paddles).join() {
			let movement = match paddle.side {
				Side::Left => input.axis_value("left_paddle"),
				Side::Right => input.axis_value("right_paddle"),
			};

			if let Some(mv_amount) = movement {
				let scaled_amount = MOVEMENT_SCALE * mv_amount as f32;
				let paddle_y = transform.translation().y;

				transform.set_y(
					(paddle_y + scaled_amount)
						.min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
						.max(PADDLE_HEIGHT * 0.5),
				);
			}
		}
	}
}
