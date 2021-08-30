use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use dino::{HealthBar, Dino};
use geometry::Rectangle;

pub struct HitboxTrackingSystem {}

const DINO_HEIGHT: f32 = 18.;
const DINO_WIDTH: f32 = 8.;

impl<'s> System<'s> for HitboxTrackingSystem {
    type SystemData = (
        ReadStorage<'s, Dino>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, HealthBar>,
    );

    fn run(&mut self, (dinos, transforms, mut health_bars): Self::SystemData) {
        for (_dino, transform, health_bar) in (&dinos, &transforms, &mut health_bars).join() {
            let position = &transform.translation();
            health_bar.rect = Rectangle {
                x1: position[0] - DINO_WIDTH / 2.,
                x2: position[0] + DINO_WIDTH / 2.,
                y1: position[1] - DINO_HEIGHT / 2.,
                y2: position[1] + DINO_HEIGHT / 2.,
            };
        }
    }
}
