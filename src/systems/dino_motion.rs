use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    core::math::Vector3,
};

use std::f32::consts::PI;

use dino::{DinoIntent, Dino, DinoState, VectorKind, ARENA_HEIGHT, ARENA_WIDTH};

const MAX_NORMAL_DINO_ACCELERATION: f32 = 0.12;
const MAX_NORMAL_DINO_VELOCITY: f32 = 0.9;
const MAX_BOOSTING_DINO_ACCELERATION: f32 = 0.18;
const MAX_BOOSTING_DINO_VELOCITY: f32 = 2.;

pub struct DinoMotionSystem {}

impl<'s> System<'s> for DinoMotionSystem {
    type SystemData = (
        WriteStorage<'s, Dino>,
        ReadStorage<'s, DinoIntent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    // TODO: make this time / frame based instead of tick-based
    fn run(&mut self, (mut dino, dino_intents, mut transforms, time): Self::SystemData) {
        for (mut dino, dino_intent, mut transform) in (&mut dino, &dino_intents, &mut transforms).join() {
            if dino_intent.state == DinoState::Bonking {
                if dino.state != DinoState::Bonking {
                    dino.state = DinoState::Bonking;
                    dino.last_state_transition = time.frame_number();
                }
                dino.last_change_in_loc[0] = 0.;
                dino.last_change_in_loc[1] = 0.;
            } else {
                dino.state = dino_intent.state;
                let old_loc = transform.translation().clone();
                match dino_intent.vec_kind {
                    VectorKind::Position => {
                        let acceleration = (
                            dino_intent.requested_vec - old_loc
                        )
                            .try_normalize(0.)
                            .unwrap_or(Vector3::new(0., 0., 0.))
                            * MAX_NORMAL_DINO_ACCELERATION;
                        update_velocity(&acceleration, &mut dino, &mut transform);
                    },
                    VectorKind::Velocity => {
                        let velocity_delta = dino_intent.requested_vec - dino.last_change_in_loc;
                        update_velocity(&velocity_delta, &mut dino, &mut transform);
                    },
                    VectorKind::Acceleration => {
                        update_velocity(&dino_intent.requested_vec, &mut dino, &mut transform);
                    },
                }
            }
        }
    }
}

fn update_velocity(acceleration: &Vector3<f32>, dino: &mut Dino, transform: &mut Transform) {
    let max_dino_acceleration = match dino.state {
        DinoState::Normal => MAX_NORMAL_DINO_ACCELERATION,
        DinoState::Bonking => MAX_NORMAL_DINO_ACCELERATION,
        DinoState::Boosting => MAX_BOOSTING_DINO_ACCELERATION,
    };
    let max_dino_velocity = match dino.state {
        DinoState::Normal => MAX_NORMAL_DINO_VELOCITY,
        DinoState::Bonking => MAX_NORMAL_DINO_VELOCITY,
        DinoState::Boosting => MAX_BOOSTING_DINO_VELOCITY,
    };
    // handle acceleration constraints
    {
        // we're going to be mutating this
        let mut acceleration = acceleration.clone();
        let magnitude = acceleration.magnitude();
        if magnitude > max_dino_acceleration {
            acceleration = (max_dino_acceleration / magnitude) * acceleration
        }
        dino.last_change_in_loc = dino.last_change_in_loc + acceleration;
    }

    // handle velocity constraints
    {
        let original_change_in_loc = dino.last_change_in_loc.clone();
        let magnitude = dino.last_change_in_loc.norm();
        if magnitude > max_dino_velocity {
            dino.last_change_in_loc = dino.last_change_in_loc * (max_dino_velocity / magnitude);
        }
    }


    // handle position constraints
    {
        // TODO: these `/ 4.` expressions should be division by two instead.
        // but the default dino height and width values are wrong, i.e., they
        // don't line up with the actual dino sprite.
        let minimum_x = dino.width / 4.;
        let maximum_x = ARENA_WIDTH - dino.width / 4.;
        let minimum_y = dino.height / 4.;
        let maximum_y = ARENA_HEIGHT - dino.height / 4.;
        let pos = transform.translation();
        let hypothetical_pos = pos + dino.last_change_in_loc;
        dino.last_change_in_loc[0] = if hypothetical_pos[0] < minimum_x {
            minimum_x - pos[0]
        } else if hypothetical_pos[0] > maximum_x {
            maximum_x - pos[0]
        } else {
            hypothetical_pos[0] - pos[0]
        };
        dino.last_change_in_loc[1] = if hypothetical_pos[1] < minimum_y {
            minimum_y - pos[1]
        } else if hypothetical_pos[1] > maximum_y {
            maximum_y - pos[1]
        } else {
            hypothetical_pos[1] - pos[1]
        };
    }

    if dino.last_change_in_loc[0] < 0. {
        transform.set_rotation_y_axis(PI);
    } else if dino.last_change_in_loc[0] > 0. {
        transform.set_rotation_y_axis(0.);
    }

    transform.prepend_translation(dino.last_change_in_loc);
}
