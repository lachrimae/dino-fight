use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::dino::{Hero, Dino, DinoState, ARENA_HEIGHT, ARENA_WIDTH};

pub struct HeroMovementSystem {
    pub hero: Entity
}

impl<'s> System<'s> for HeroMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Dino>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut dinos, heroes, input, time): Self::SystemData) {
        let dino = dinos.get_mut(self.hero).unwrap();
        let transform = transforms.get_mut(self.hero).unwrap();

        let r_bonk = input.action_is_down("bonk").unwrap_or(false);
        if r_bonk && dino.state == DinoState::Normal {
            dino.state = DinoState::Bonking;
            dino.last_state_transition = time.frame_number();
        }

        let (rdx, rdy) = if dino.state == DinoState::Bonking {
            (0., 0.)
        } else {
            let rdx = input.axis_value("hero_x").unwrap_or(0.);
            let rdy = input.axis_value("hero_y").unwrap_or(0.);
            (rdx, rdy)
        };

        if rdx != 0.0 || rdy != 0.0 {
            let denominator = (rdx.powf(2.) + rdy.powf(2.)).sqrt();

            let rdx = rdx / denominator;
            let rdy = rdy / denominator;

            let (dx, dy) = {
                let &vec = transform.translation();
                let (x, y) = (vec[0], vec[1]);
                let dx = if x + rdx < 0. {
                    -x
                } else if x + rdx > ARENA_WIDTH {
                    ARENA_WIDTH - x
                } else {
                    rdx
                };

                let dy = if y + rdy < 0. {
                    -y
                } else if y + rdy > ARENA_HEIGHT {
                    ARENA_HEIGHT - y
                } else {
                    rdy
                };
                (dx, dy)
            };

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
