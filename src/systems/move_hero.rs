use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    core::math::Vector3,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::dino::{Hero, Dino, DinoState, ARENA_HEIGHT, ARENA_WIDTH};

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

            let rd_pos = if dino.state == DinoState::Bonking {
                Vector3::new(0., 0., 0.)
            } else {
                let rdx = input.axis_value("hero_x").unwrap_or(0.);
                let rdy = input.axis_value("hero_y").unwrap_or(0.);
                Vector3::new(rdx, rdy, 0.)
            };

            let d_pos = rd_pos.try_normalize(0.).unwrap_or(Vector3::new(0., 0., 0.));
            // bounds checking
            let translation = {
                let &pos = transform.translation();
                let (x, y) = (pos[0], pos[1]);
                let (dx, dy) = (d_pos[0], d_pos[1]);
                let dx = if x + dx < 0. {
                    -x
                } else if x + dx > ARENA_WIDTH {
                    ARENA_WIDTH - x
                } else {
                    dx
                };

                let dy = if y + dy < 0. {
                    -y
                } else if y + dy > ARENA_HEIGHT {
                    ARENA_HEIGHT - y
                } else {
                    dy
                };
                Vector3::new(dx, dy, 0.)
            };
            //
            // make character face correct direction
            if translation[0] < 0. {
                transform.set_rotation_y_axis(std::f32::consts::PI);
            } else if translation[0] > 0. {
                transform.set_rotation_y_axis(0.);
            }

            transform.prepend_translation(translation);
            dino.last_change_in_loc = translation;
        }
    }
}
