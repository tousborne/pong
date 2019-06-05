//! The Amethyst Pong tutorial

// Standard libraries
use amethyst::{
	core::transform::TransformBundle,
	input::InputBundle,
	prelude::*,
	renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
        ui::{DrawUi, UiBundle},
	utils::application_root_dir,
};

// Project libraries
mod pong;
mod systems;


fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let config = DisplayConfig::load(format!(
		"{}/resources/display_config.ron",
		application_root_dir()
	));

	let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(format!(
		"{}/resources/bindings_config.ron",
		application_root_dir()
	))?;

	// Clear the screen to IKB.
	let pipe = Pipeline::build().with_stage(
		Stage::with_backbuffer()
			.clear_target([0.0, 0.18, 0.65, 2.0], 1.0)
			.with_pass(DrawFlat2D::new())
                        .with_pass(DrawUi::new())
	);

	let game_data = GameDataBuilder::default()
		.with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
		.with_bundle(TransformBundle::new())?
		.with_bundle(input_bundle)?
                .with_bundle(UiBundle::<String, String>::new())?
		.with(systems::PaddleSystem, "paddle_system", &["input_system"])
		.with(systems::BallMoveSystem, "move_system", &[])
		.with(
			systems::BallBounceSystem,
			"bounce_system",
			&["paddle_system", "move_system"],
		)
                .with(systems::ScoreSystem, "score_system", &["move_system"]);

	let mut game = Application::new("./", pong::Pong, game_data)?;
	game.run();

	return Ok(());
}
