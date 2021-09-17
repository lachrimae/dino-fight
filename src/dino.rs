use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, VecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::geometry::Rectangle;

pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 200.0;

pub const DINO_HEIGHT: f32 = 24.0;
pub const DINO_WIDTH: f32 = 24.0;

pub struct Ai {}

impl Component for Ai {
    type Storage = VecStorage<Self>;
}

pub struct Animation {
    pub frames: i32,
    pub frame_duration: u64,
    pub first_sprite_index: usize,
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DinoState {
    Normal,
    Bonking,
    Boosting,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VectorKind {
    Position,
    Velocity,
    Acceleration,
}

#[derive(PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
}

impl Team {
    pub fn not(team: &Team) -> Team {
        match team {
            Team::Player => Team::Enemy,
            Team::Enemy => Team::Player,
        }
    }
}

#[derive(Debug)]
pub struct DinoIntent {
    pub state: DinoState,
    pub vec_kind: VectorKind,
    pub requested_vec: Vector3<f32>,
}

impl Component for DinoIntent {
    type Storage = VecStorage<Self>;
}

pub struct DespawnTimer {
    pub deadline: u64,
}

impl Component for DespawnTimer {
    type Storage = VecStorage<Self>;
}

pub struct Dino {
    pub width: f32,
    pub height: f32,
    pub last_change_in_loc: Vector3<f32>,
    pub state: DinoState,
    pub last_state_transition: u64,
    pub allegiance: Team,
}

impl Dino {
    fn new(allegiance: Team) -> Dino {
        Dino {
            width: DINO_WIDTH,
            height: DINO_HEIGHT,
            last_change_in_loc: Vector3::new(0., 0., 0.),
            state: DinoState::Normal,
            last_state_transition: 0,
            allegiance,
        }
    }
}

impl Default for Dino {
    fn default() -> Dino {
        Dino::new(Team::Enemy)
    }
}

impl Component for Dino {
    type Storage = DenseVecStorage<Self>;
}

pub struct HealthBar {
    pub value: u32,
    pub damageable_at: u64,
    pub allegiance: Team,
    pub rect: Rectangle,
}

impl Component for HealthBar {
    type Storage = DenseVecStorage<Self>;
}

pub struct DamageEffect {
    pub value: u32,
    pub targets: Team,
    pub rect: Rectangle,
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

        // Hero we have to somehow pass the hero we initialise into the
        // HeroIntentSystem.
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

    let intent = DinoIntent {
        state: DinoState::Normal,
        requested_vec: Vector3::new(0., 0., 0.),
        vec_kind: VectorKind::Velocity,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Hero::default())
        .with(intent)
        .with(Dino::new(Team::Player))
        .with(HealthBar {
            value: 150,
            damageable_at: 0,
            allegiance: Team::Player,
            rect: Rectangle {
                x1: 0., x2: ARENA_WIDTH,
                y1: 0., y2: ARENA_HEIGHT,
            },
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

    let pos = transform.translation();
    let intent = DinoIntent {
        state: DinoState::Normal,
        requested_vec: pos.clone(),
        vec_kind: VectorKind::Velocity,
    };

    transform.set_translation_xyz(ARENA_WIDTH - DINO_WIDTH * 0.5, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Dino::default())
        .with(Ai {})
        .with(HealthBar {
            value: 100,
            rect: Rectangle {
                x1: 0., x2: ARENA_WIDTH,
                y1: 0., y2: ARENA_HEIGHT,
            },
            allegiance: Team::Enemy,
            damageable_at: 0,
        })
        .with(transform)
        .with(intent)
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
