//! System to check for bounce collisions every frame.

// Standard libraries
use amethyst::{
	core::transform::Transform,
	ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

// Project libraries
use crate::pong::{Ball, Paddle, Side, ARENA_HEIGHT, ARENA_WIDTH};

// System to bounce the ball.
pub struct BallBounceSystem;

impl<'s> System<'s> for BallBounceSystem {
	type SystemData = (
		WriteStorage<'s, Ball>,
		ReadStorage<'s, Paddle>,
		ReadStorage<'s, Transform>,
	);

	// Checks for collisions and updates velocities.
	fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
		for (ball, transform) in (&mut balls, &transforms).join() {
			let ball_x = transform.translation().x;
			let ball_y = transform.translation().y;

			// Bounce off the arena walls.
			if ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0 {
				ball.velocity[1] = -ball.velocity[1];
			}
			else if ball_y <= ball.radius && ball.velocity[1] < 0.00 {
				ball.velocity[1] = -ball.velocity[1];
			}

			if ball_x >= ARENA_WIDTH - ball.radius && ball.velocity[0] > 0.0 {
				ball.velocity[0] = -ball.velocity[0] * 0.5;
				ball.velocity[1] = ball.velocity[1] * 0.5;
			}
			else if ball_x <= ball.radius && ball.velocity[0] < 0.00 {
				ball.velocity[0] = -ball.velocity[0] * 0.5;
				ball.velocity[1] = ball.velocity[1] * 0.5;
			}

			// Bounce off the paddles.
			for (paddle, paddle_transform) in (&paddles, &transforms).join() {
				if paddle_collision(ball_x, ball_y, &ball, &paddle, &paddle_transform) {
					if paddle.side == Side::Left && ball.velocity[0] < 0.0 {
						ball.velocity[0] = -ball.velocity[0];
					}
					else if paddle.side == Side::Right && ball.velocity[0] > 0.0 {
						ball.velocity[0] = -ball.velocity[0];
					}
				}
			}
		}
	}
}

// Check if a ball has collided with a paddle.
fn paddle_collision(
	ball_x: f32,
	ball_y: f32,
	ball: &Ball,
	paddle: &Paddle,
	transform: &Transform,
) -> bool {
	let paddle_x = transform.translation().x - paddle.width * 0.5;
	let paddle_y = transform.translation().y - paddle.height * 0.5;

	let left = paddle_x - ball.radius;
	let right = paddle_x + paddle.width + ball.radius;
	let top = paddle_y + paddle.height + ball.radius;
	let bottom = paddle_y - ball.radius;

	return (ball_x >= left) & (ball_x <= right) & (ball_y >= bottom) & (ball_y <= top);
}
