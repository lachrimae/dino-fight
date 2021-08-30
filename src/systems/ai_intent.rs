use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::dino::{Dino, DinoState, Team};
use std::cmp;
use geometry;

pub struct HeroMovementSystem {}

impl<'s> System<'s> for HeroMovementSystem {
    type SystemData = (
        ReadStorage<'s, Dino>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HealthBar>,
        WriteStorage<'s, AiIntent>
        Read<'s, Time>,
    );

    fn run(&mut self, (dinos, transforms, time, mut ai_intents): Self::SystemData) {
        for (dino, transform, health_bar, ai_intent) in (&dinos, &transforms, &health_bars, &time, &mut ai_intent).join() {
            let position = transform.translation();
            match (&health_bars).join().filter(|victim| victim.allegiance == Team::Player).min_by(|victim| geometry::distance(position, victim.rect)) {
                None => {
                    ai_intent.state = DinoState::Normal,
                    ai_intent.rx = position[0];
                    ai_intent.ry = position[1];
                },
                Some(victim) => {
                    let (magnitude, direction) = geometry::distance(transform.translation(), potential_victim);
                    if health_bar.value > 50 {
                        // The attack policy
                        if magnitude > 10 {
                            ai_intent.state = DinoState::Normal,
                            ai_intent.rx = magnitude * direction[0];
                            ai_intent.ry = magnitude * direction[1];
                        } else {
                            ai_intent.state = DinoState::Bonking;
                            ai_intent.rx = position[0];
                            ai_intent.ry = position[1];
                        }
                    } else {
                        ai_intent.state = DinoState::Normal,
                        ai_intent.rx = -magnitude * direction[0];
                        ai_intent.ry = -magnitude * direction[1];
                    }
                }
            }
        }
    }
}
