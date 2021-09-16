use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
    core::math::Vector3,
};

use dino::{Ai, DinoIntent, Dino, DinoState, HealthBar, Team, VectorKind};
use geometry;

use std::cmp::Ordering;
use std::slice::SliceIndex;

pub struct AiIntentSystem {}

const NEIGHBOURHOOD_SIZE: u8 = 4;
const COMFORTABLE_WITH_NEIGHBOURS_THRESHOLD: f32 = 0.5;

impl<'s> System<'s> for AiIntentSystem {
    type SystemData = (
        ReadStorage<'s, Ai>,
        ReadStorage<'s, Dino>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HealthBar>,
        WriteStorage<'s, DinoIntent>,
    );

    fn run(&mut self, (ai, dinos, transforms, health_bars, mut ai_intents): Self::SystemData) {
        for (_ai, dino, transform, health_bar, ai_intent) in (&ai, &dinos, &transforms, &health_bars, &mut ai_intents).join() {
            let ai_position = transform.translation();
            if dino.state != DinoState::Bonking {
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
                            let dino_locations = (&dinos, &transforms).join();
                            let closest = get_k_smallest_by(NEIGHBOURHOOD_SIZE, dino_locations, |(_neighbour_dino, neighbour_transform)| {
                                let neighbour_position = transform.translation();
                                let delta = (ai_position - neighbour_position).norm();
                                delta
                            });
                            let num_friendly = closest
                                .iter()
                                .filter(|(neighbour_dino, _neighbour_transform)| 
                                   neighbour_dino.allegiance == dino.allegiance
                                )
                                .count();
                            if (num_friendly as f32) / (NEIGHBOURHOOD_SIZE as f32) >= COMFORTABLE_WITH_NEIGHBOURS_THRESHOLD {
                                // the defend policy
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
    }
}

// Here we iterate over the I to find the k smallest entries,
// measured by the quantify function that gets passed in.
// 
// Currently this was only used in one place, but it's a bit of a doozy
// so I decided to write it on its own.
//
// If the length of this function's input is shorter than k then weird things
// will happen.
fn get_k_smallest_by<I, A, F>(k: u8, it: I, quantify: F) -> Vec<A>
where
    I: Iterator<Item = A>,
    F: Fn(&A) -> f32,
{
    let mut pushed = 0;
    let mut smallest: Vec<A> = Vec::new();
    for a in it {
        if pushed < k {
            smallest.push(a);
            pushed += 1;
        } else {
            let mut index_at_max: Option<usize> = None;
            for (n, entry) in smallest.iter().enumerate() {
                if quantify(&entry) > quantify(&a) { 
                    match index_at_max {
                        None => { index_at_max = Some(n); },
                        Some(index) => {
                            if quantify(&entry) > quantify(&smallest[index]) {
                                index_at_max = Some(n);
                            }
                        },
                    }
                }
            }
            if let Some(index) = index_at_max {
                smallest[index] = a;
            }
        }
    }
    smallest
}
