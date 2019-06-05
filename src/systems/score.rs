use amethyst::{
	core::transform::Transform,
	ecs::prelude::{Join, System, WriteStorage},
};

use crate::pong::{Ball, ARENA_WIDTH};


pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
	type SystemData = (WriteStorage<'s, Ball>, WriteStorage<'s, Transform>);

	fn run(&mut self, (mut balls, mut transforms): Self::SystemData) {
		for (ball, transform) in (&mut balls, &mut transforms).join() {
			let ball_x = transform.translation().x;

			if ball_x <= ball.radius {
				println!("Player 2 has scored");
				reset_ball(ball, transform);
			}

			if ball_x >= ARENA_WIDTH - ball.radius {
				println!("Player 1 has scored");
				reset_ball(ball, transform);
			}
		}
	}
}

fn reset_ball(ball: &mut Ball, transform: &mut Transform) {
	ball.velocity[0] = -ball.velocity[0];
	transform.set_x(ARENA_WIDTH * 0.5);
}
