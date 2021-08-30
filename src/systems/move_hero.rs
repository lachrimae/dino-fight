use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::dino::{Hero, Dino, DinoState};

pub struct HeroMovementSystem {}

impl<'s> System<'s> for HeroMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Dino>,
        ReadStorage<'s, Hero>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut dinos, heroes, input, time): Self::SystemData) {
        for (_hero, dino, transform) in (&heroes, &mut dinos, &mut transforms).join() {

            let r_bonk = input.action_is_down("bonk").unwrap_or(false);
            if r_bonk && dino.state == DinoState::Normal {
                dino.state = DinoState::Bonking;
                dino.last_state_transition = time.frame_number();
            }

            let (rx, ry) = if dino.state == DinoState::Bonking {
                (0., 0.)
            } else {
                let rx = input.axis_value("hero_x").unwrap_or(0.);
                let ry = input.axis_value("hero_y").unwrap_or(0.);
                (rx, ry)
            };

            if rx != 0.0 || ry != 0.0 {
                let denominator = (rx.powf(2.) + ry.powf(2.)).sqrt();

                let dx = rx / denominator;
                let dy = ry / denominator;


                transform.prepend_translation_x(dx);
                transform.prepend_translation_y(dy);

                if dx < 0. {
                    transform.set_rotation_y_axis(std::f32::consts::PI);
                } else if dx > 0. {
                    transform.set_rotation_y_axis(0.);
                }

                dino.dx = dx;
                dino.dy = dy;
            } else {
                dino.dx = 0.;
                dino.dy = 0.;
            }
        }
    }
}
