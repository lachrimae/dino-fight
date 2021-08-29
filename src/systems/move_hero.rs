use amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::dino::Hero;

pub struct HeroMovementSystem {}

impl<'s> System<'s> for HeroMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Hero>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, heroes, input): Self::SystemData) {
        for (hero, transform) in (&heroes, &mut transforms).join() {
            let rx = input.axis_value("hero_x").unwrap_or(0.);
            let ry = input.axis_value("hero_y").unwrap_or(0.);

            if rx != 0.0 || ry != 0.0 {
                let denominator = (rx.powf(2.) + ry.powf(2.)).sqrt();

                let dx = rx / denominator;
                let dy = ry / denominator;

                transform.prepend_translation_x(dx);
                transform.prepend_translation_y(dy);

//                hero.dx = dx;
//                hero.dy = dy;
            }
        }
    }
}
