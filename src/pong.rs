//! Game libraries.

// Standard libraries
use amethyst::{
	assets::{AssetStorage, Loader},
	core::transform::Transform,
	ecs::prelude::{Component, DenseVecStorage},
	prelude::*,
	renderer::{
		Camera,
		Flipped,
		PngFormat,
		Projection,
		SpriteRender,
		SpriteSheet,
		SpriteSheetFormat,
		SpriteSheetHandle,
		Texture,
		TextureMetadata,
	},
};

// Constants
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const BALL_VELOCITY_X: f32 = 50.0;
pub const BALL_VELOCITY_Y: f32 = 30.0;
pub const BALL_RADIUS: f32 = 2.0;


// Enumeration of sides of the game world.
#[derive(PartialEq, Eq)]
pub enum Side {
	Left,
	Right,
}


// The game state.
pub struct Pong;

// SimpleState implements a minimal state machine
impl SimpleState for Pong {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		let sprite_sheet = load_sprite_sheet(world);

		world.register::<Ball>();

		initialize_ball(world, &sprite_sheet);
		initialize_paddles(world, &sprite_sheet);
		initialize_camera(world);
	}
}


// Represents a player controlled paddle.
pub struct Paddle {
	pub side: Side,
	pub width: f32,
	pub height: f32,
}

impl Paddle {
	fn new(side: Side) -> Paddle {
		return Paddle {
			side: side,
			width: PADDLE_WIDTH,
			height: PADDLE_HEIGHT,
		};
	}
}

// Define the paddle's storage as a game component.
impl Component for Paddle {
	type Storage = DenseVecStorage<Self>;
}


// Represents the pong ball.
pub struct Ball {
	pub velocity: [f32; 2],
	pub radius: f32,
}

// Define the ball's storage as a game component.
impl Component for Ball {
	type Storage = DenseVecStorage<Self>;
}


// Add the initial ball sprite to the world.
fn initialize_ball(world: &mut World, sprite_sheet: &SpriteSheetHandle) {
	let mut local_transform = Transform::default();
	local_transform.set_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet.clone(),
		sprite_number: 1,
	};

	world
		.create_entity()
		.with(sprite_render)
		.with(Ball {
			radius: BALL_RADIUS,
			velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
		})
		.with(local_transform)
		.build();
}


// Add the initial cameras to the world.
fn initialize_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_z(1.0);

	world
		.create_entity()
		.with(Camera::from(Projection::orthographic(
			0.0,
			ARENA_WIDTH,
			0.0,
			ARENA_HEIGHT,
		)))
		.with(transform)
		.build();
}


// Add the initial paddles' sprites to the world.
fn initialize_paddles(world: &mut World, sprite_sheet: &SpriteSheetHandle) {
	let mut left_transform = Transform::default();
	let mut right_transform = Transform::default();

	let y = ARENA_HEIGHT * 0.5;
	left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
	right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet.clone(),
		sprite_number: 0,
	};

	// Add the left paddle to the world.
	world
		.create_entity()
		.with(sprite_render.clone())
		.with(Paddle::new(Side::Left))
		.with(left_transform)
		.build();

	// Add the right paddle to the world.
	world
		.create_entity()
		.with(sprite_render.clone())
		.with(Flipped::Horizontal)
		.with(Paddle::new(Side::Right))
		.with(right_transform)
		.build();
}


// Load the sprite sheets.
fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			"texture/pong_spritesheet.png",
			PngFormat,
			TextureMetadata::srgb_scale(),
			(),
			&texture_storage,
		)
	};

	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
		"texture/pong_spritesheet.ron",
		SpriteSheetFormat,
		texture_handle,
		(),
		&sprite_sheet_store,
	)
}
