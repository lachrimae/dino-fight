use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use dino::{AiIntent, Dino, DinoState, HealthBar, Team};
use geometry;

use std::cmp::Ordering;

pub struct AiIntentSystem {}

impl<'s> System<'s> for AiIntentSystem {
    type SystemData = (
        ReadStorage<'s, Dino>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HealthBar>,
        WriteStorage<'s, AiIntent>,
    );

    fn run(&mut self, (dinos, transforms, health_bars, mut ai_intents): Self::SystemData) {
        for (dino, transform, health_bar, ai_intent) in (&dinos, &transforms, &health_bars, &mut ai_intents).join() {
            let position = &transform.translation().as_slice();
            match (&health_bars)
                .join()
                .filter(|adversary| adversary.allegiance == Team::Player)
                .min_by(|adversary1, adversary2|
                    geometry::distance(position, &adversary1.rect).magnitude.abs().partial_cmp(
                        &geometry::distance(position, &adversary2.rect).magnitude.abs()
                    ).unwrap_or(Ordering::Greater)
                ) {
                None => {
                    ai_intent.state = DinoState::Normal;
                    ai_intent.rx = position[0];
                    ai_intent.ry = position[1];
                },
                Some(adversary) => {
                    let vector_to_adversary = geometry::distance(transform.translation().as_slice(), &adversary.rect);
                    let (magnitude, direction) = (vector_to_adversary.magnitude, vector_to_adversary.direction);
                    if health_bar.value > 50 {
                        // The attack policy
                        if magnitude > 2. {
                            ai_intent.state = DinoState::Normal;
                            ai_intent.rx = magnitude * direction.x;
                            ai_intent.ry = magnitude * direction.y;
                        } else {
                            ai_intent.state = DinoState::Bonking;
                            ai_intent.rx = position[0];
                            ai_intent.ry = position[1];
                        }
                    } else {
                        // the run away policy
                        ai_intent.state = DinoState::Normal;
                        ai_intent.rx = -magnitude * direction.x;
                        ai_intent.ry = -magnitude * direction.y;
                    }
                }
            }
            println!("intent: {:?}", ai_intent);
        }
    }
}
