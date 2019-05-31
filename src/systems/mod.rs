//! ::systems namespace management.

mod ball_bounce;
mod ball_move;
mod paddle;

pub use self::{ball_bounce::BallBounceSystem, ball_move::BallMoveSystem, paddle::PaddleSystem};
