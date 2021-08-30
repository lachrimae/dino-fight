use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
    core::math::Vector3,
};

use dino::{AiIntent, Dino, DinoState, HealthBar, Team, VectorKind};
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
        for (_dino, transform, health_bar, ai_intent) in (&dinos, &transforms, &health_bars, &mut ai_intents).join() {
            let ai_position = transform.translation();
            match (&health_bars)
                .join()
                .filter(|adversary| adversary.allegiance == Team::Player)
                .min_by(|adversary1, adversary2|
                    // f32 only implements a partial order due to weirdness of floats.
                    // The `unwrap_or(Ordering::Greater)` ensures that infinities
                    // or NANs are never chosen as the minimum when there is another
                    // option.
                    geometry::distance(ai_position, &adversary1.rect).partial_cmp(
                        &geometry::distance(ai_position, &adversary2.rect)
                    ).unwrap_or(Ordering::Greater)
                ) {
                None => {
                    ai_intent.state = DinoState::Normal;
                    ai_intent.vec_kind = VectorKind::Position;
                    ai_intent.requested_vec = *ai_position;
                },
                Some(adversary) => {
                    let adversary_closest_point = geometry::closest_point_on_rect(&ai_position, &adversary.rect);
                    let distance = (adversary_closest_point - ai_position).magnitude();
                    if health_bar.value > 50 {
                        // The attack policy
                        if distance > 1. {
                            ai_intent.state = DinoState::Normal;
                            ai_intent.vec_kind = VectorKind::Position;
                            ai_intent.requested_vec = adversary_closest_point; 
                        } else {
                            ai_intent.state = DinoState::Bonking;
                            ai_intent.vec_kind = VectorKind::Position;
                            ai_intent.requested_vec = *ai_position;
                        }
                    } else {
                        // the run away policy
                        ai_intent.state = DinoState::Normal;
                        ai_intent.vec_kind = VectorKind::Velocity;
                        if distance == 0. {
                            ai_intent.requested_vec = Vector3::new(1., 1., 0.);
                        } else {
                            ai_intent.requested_vec = *ai_position - adversary_closest_point;
                        }
                    }
                }
            }
        }
    }
}
