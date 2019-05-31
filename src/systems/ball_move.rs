//! System to move the ball every frame.

// Standard libraries
use amethyst::{
	core::{timing::Time, transform::Transform},
	ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

// Project libraries
use crate::pong::Ball;

// System to move the ball.
pub struct BallMoveSystem;

impl<'s> System<'s> for BallMoveSystem {
	type SystemData = (
		ReadStorage<'s, Ball>,
		WriteStorage<'s, Transform>,
		Read<'s, Time>,
	);

	// Move the balls based on their speed and time elapsed.
	fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
		for (ball, local) in (&balls, &mut locals).join() {
			local.translate_x(ball.velocity[0] * time.delta_seconds());
			local.translate_y(ball.velocity[1] * time.delta_seconds());
		}
	}
}
