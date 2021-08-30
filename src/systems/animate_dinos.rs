use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, WriteStorage, System},
    renderer::SpriteRender,
};

use crate::dino::{Animation, Dino, DinoState};

pub struct DinoAnimationSystem {}

impl<'s> System<'s> for DinoAnimationSystem {
    type SystemData = (
        WriteStorage<'s, Animation>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Dino>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut animations, mut renders, mut dinos, time): Self::SystemData) {
        // possible optimization:
        // Here we write to every dino's animation every loop.
        // It may be worth tracking state transitions in a
        // state enum field and only writing the other fields
        // if they need to change.
        for (animation, sprite, dino) in (&mut animations, &mut renders, &mut dinos).join() {
            if dino.state == DinoState::Bonking {
                animation.frames = 4;
                animation.frame_duration = 10;
                animation.first_sprite_index = 10;
            } else if dino.dx != 0. || dino.dy != 0. {
                animation.frames = 6;
                animation.frame_duration = 10;
                animation.first_sprite_index = 4;
            } else {
                animation.frames = 4;
                animation.frame_duration = 10;
                animation.first_sprite_index = 0;
            }

            let elapsed_time = time.frame_number();
            let frame = {
                let last_state_transition = dino.last_state_transition;
                let delta = elapsed_time - last_state_transition;
                if (delta / animation.frame_duration) as i32 >= animation.frames {
                    dino.state = DinoState::Normal;
                    dino.last_state_transition = elapsed_time;
                    (delta / animation.frame_duration) as i32 % animation.frames
                } else {
                    (delta / animation.frame_duration) as i32 % animation.frames
                }
            };
            sprite.sprite_number = animation.first_sprite_index + frame as usize;
        }
    }
}
