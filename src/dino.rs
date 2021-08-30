use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const DINO_HEIGHT: f32 = 24.0;
pub const DINO_WIDTH: f32 = 24.0;

pub struct Animation {
    pub frames: i32,
    pub frame_duration: u64,
    pub first_sprite_index: usize,
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}

#[derive(PartialEq, Eq)]
pub enum DinoState {
    Normal,
    Bonking,
}

#[derive(PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
}

pub struct Dino {
    pub width: f32,
    pub height: f32,
    pub dx: f32,
    pub dy: f32,
    pub state: DinoState,
    pub last_state_transition: u64,
}

impl Dino {
    fn new(dx: f32, dy: f32) -> Dino {
        Dino {
            width: DINO_WIDTH,
            height: DINO_HEIGHT,
            dx, dy,
            state: DinoState::Normal,
            last_state_transition: 0,
        }
    }
}

impl Default for Dino {
    fn default() -> Dino {
        Dino::new(0., 0.)
    }
}

impl Component for Dino {
    type Storage = DenseVecStorage<Self>;
}

pub struct HealthBar {
    pub value: u32,
    pub damageableAt: u64,
    pub allegiance: Team,
    pub rect: ()
}

impl Component for HealthBar {
    type Storage = DenseVecStorage<Self>;
}

pub struct DamageEffect {
    pub value: u32,
    pub targets: Team,
    pub rect: (),
}

impl Component for DamageEffect {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Hero;

impl Component for Hero {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct DinoFight {}

impl SimpleState for DinoFight {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let doux_handle = load_sprite_sheet(world, "doux");
        let vita_handle = load_sprite_sheet(world, "vita");
        let mort_handle = load_sprite_sheet(world, "mort");
        let tard_handle = load_sprite_sheet(world, "tard");

        world.register::<Dino>();
        initialise_hero(world, doux_handle);
        initialise_dinos(world, vita_handle, mort_handle, tard_handle);
        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_hero(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut transform = Transform::default();
    transform.set_translation_xyz(DINO_WIDTH * 0.5, ARENA_HEIGHT / 2.0, 0.0);

    let animation = Animation {
        frames: 4,
        frame_duration: 10,
        first_sprite_index: 0,
    };


    world
        .create_entity()
        .with(sprite_render)
        .with(Hero::default())
        .with(Dino::default())
        .with(HealthBar {
            value: 150,
            damageableAt: 0,
            allegiance: Team::Player,
            rect: (),
        })
        .with(transform)
        .with(animation)
        .build();
}

fn initialise_dinos(world: &mut World, handle1: Handle<SpriteSheet>, _handle2: Handle<SpriteSheet>, _handle3: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(handle1, 0);
    let mut transform = Transform::default();

    let animation = Animation {
        frames: 4,
        frame_duration: 10,
        first_sprite_index: 0,
    };

    transform.set_translation_xyz(ARENA_WIDTH - DINO_WIDTH * 0.5, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Dino::default())
        .with(HealthBar {
            value: 100,
            rect: (),
            allegiance: Team::Enemy,
            damageableAt: 0,
        })
        .with(transform)
        .with(animation)
        .build();
}

fn load_sprite_sheet(world: &mut World, dino_name: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/".to_owned() + dino_name + ".png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/dino.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
