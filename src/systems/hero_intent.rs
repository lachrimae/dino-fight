use amethyst::{
    core::math::Vector3,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::dino::{Hero, DinoState, VectorKind, DinoIntent};

pub struct HeroIntentSystem {}

impl<'s> System<'s> for HeroIntentSystem {
    type SystemData = (
        ReadStorage<'s, Hero>,
        WriteStorage<'s, DinoIntent>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (heroes, mut dino_intents, input): Self::SystemData) {
        for (_hero, intent) in (&heroes, &mut dino_intents).join() {
            let r_bonk = input.action_is_down("bonk").unwrap_or(false);
            let r_boost = input.action_is_down("boost").unwrap_or(false);
            let rdx = input.axis_value("hero_x").unwrap_or(0.);
            let rdy = input.axis_value("hero_y").unwrap_or(0.);
            if r_bonk  {
                intent.state = DinoState::Bonking;
                intent.requested_vec = Vector3::new(0., 0., 0.);
                intent.vec_kind = VectorKind::Velocity;
            } else if r_boost {
                intent.state = DinoState::Boosting;
                intent.requested_vec = Vector3::new(400. * rdx, 400. * rdy, 0.);
                intent.vec_kind = VectorKind::Acceleration;
            } else {
                intent.state = DinoState::Normal;
                intent.requested_vec = Vector3::new(rdx, rdy, 0.);
                intent.vec_kind = VectorKind::Velocity;
            }
        }
    }
}
