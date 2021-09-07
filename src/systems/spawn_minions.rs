use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, ReadStorage, System},
};

use crate::dino::DespawnTimer;

pub struct DespawnSystem {}

impl<'s> System<'s> for DespawnSystem {
    type SystemData = (
        ReadStorage<'s, Dino>,
        Entities<'s>,
    );

    fn run(&mut self, (mut dinos): Self::SystemData) {
        for dino in (&mut dinos).join() {
            if dino.state == DinoState::Summoning {
                entities.build_entity()
                    .with(sprite_render)
                    .with(Dino::default())
                    .with(HealthBar::default())
                    .with(transform)
                    .with(intent)
                    .with(animation)
                    .build();
            }
        }
    }
}
