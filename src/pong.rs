//! Game libraries.

// Standard libraries
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata
    },
};

// Constants
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;

// Enumeration of sides of the game world.
#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

// The game state.
pub struct Pong;

// SimpleState implements a minimal state machine
impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet = load_sprite_sheet(world);

        world.register::<Paddle>();

        initialize_paddles(world, sprite_sheet);
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
            width: 1.0,
            height: 1.0,
        }
    }
}

// The paddle is associated with the game engine and the storage type defined.
impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

// Add the initial cameras to the world.
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(0.0, ARENA_WIDTH, 0.0, ARENA_HEIGHT)))
        .with(transform)
        .build();
}

// Add the initial paddles to the world.
fn initialize_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
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
