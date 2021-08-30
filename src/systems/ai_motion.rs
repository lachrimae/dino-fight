use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    core::math::Vector3,
};

use dino::{AiIntent, Dino, DinoState, VectorKind};

const MAX_DINO_ACCELERATION: f32 = 1.0;

pub struct AiMotionSystem {}

impl<'s> System<'s> for AiMotionSystem {
    type SystemData = (
        WriteStorage<'s, Dino>,
        ReadStorage<'s, AiIntent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    // TODO: make this time / frame based instead of tick-based
    fn run(&mut self, (mut dino, ai_intents, mut transforms, time): Self::SystemData) {
        for (mut dino, ai_intent, mut transform) in (&mut dino, &ai_intents, &mut transforms).join() {
            if ai_intent.state == DinoState::Bonking {
                dino.last_change_in_loc = Vector3::new(0., 0., 0.);
            } else {
                let old_loc = transform.translation().clone();
                match ai_intent.vec_kind {
                    VectorKind::Position => {
                        let acceleration = (old_loc
                            - ai_intent
                            .requested_vec
                            .try_normalize(0.)
                            .unwrap_or(Vector3::new(0., 0., 0.)))
                            * MAX_DINO_ACCELERATION;
                        update_velocity(&acceleration, &mut dino, &mut transform);
                    },
                    VectorKind::Velocity => {
                        let velocity_delta = ai_intent.requested_vec - dino.last_change_in_loc;
                        update_velocity(&velocity_delta, &mut dino, &mut transform);
                    },
                    VectorKind::Acceleration => {
                        update_velocity(&ai_intent.requested_vec, &mut dino, &mut transform);
                    },
                }
            }
        }
    }
}

fn update_velocity(acceleration: &Vector3<f32>, dino: &mut Dino, transform: &mut Transform) {
    let mut acceleration = acceleration.clone();
    let magnitude = acceleration.magnitude();
    if magnitude > MAX_DINO_ACCELERATION {
        acceleration = (MAX_DINO_ACCELERATION / magnitude) * acceleration
    }
    dino.last_change_in_loc[0] += acceleration[0];
    dino.last_change_in_loc[1] += acceleration[1];
    transform.append_translation(dino.last_change_in_loc);
}
