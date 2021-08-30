use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, ReadStorage, System},
};

use crate::dino::DespawnTimer;

pub struct DespawnSystem {}

impl<'s> System<'s> for DespawnSystem {
    type SystemData = (
        ReadStorage<'s, DespawnTimer>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (despawners, time, entities): Self::SystemData) {
        for (despawner, entity) in (&despawners, &*entities).join() {
            if despawner.deadline <= time.frame_number() {
                entities.delete(entity);
            }
        }
    }
}
