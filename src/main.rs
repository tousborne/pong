//! The Amethyst Pong tutorial

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

// The game state.
pub struct Pong;

// SimpleState implements a minimal state machine
impl SimpleState for Pong {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    // Clear the screen to IKB.
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.18, 0.65, 2.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::new("./", Pong, game_data)?;
    game.run();

    return Ok(());
}