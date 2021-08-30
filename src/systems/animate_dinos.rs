use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{Entities, Join, Read, ReadStorage, WriteStorage, System},
    renderer::SpriteRender,
};

use crate::dino::{Animation, DamageEffect, DespawnTimer, Dino, DinoState, Team};
use crate::geometry::Rectangle;

pub struct DinoAnimationSystem {}

impl<'s> System<'s> for DinoAnimationSystem {
    type SystemData = (
        WriteStorage<'s, Animation>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Dino>,
        WriteStorage<'s, DamageEffect>,
        WriteStorage<'s, DespawnTimer>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
        Entities<'s>
    );

    fn run(&mut self, (mut animations, mut renders, mut dinos, mut damage_effects, mut despawn_timers, transforms, time, entities): Self::SystemData) {
        // possible optimization:
        // Here we write to every dino's animation every loop.
        // It may be worth tracking state transitions in a
        // state enum field and only writing the other fields
        // if they need to change.
        for (animation, sprite, dino, transform) in (&mut animations, &mut renders, &mut dinos, &transforms).join() {
            if dino.state == DinoState::Bonking {
                animation.frames = 4;
                animation.frame_duration = 5;
                animation.first_sprite_index = 10;
            } else if dino.last_change_in_loc.norm() > 0. {
                animation.frames = 6;
                animation.frame_duration = 5;
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

                // Tying state transitions to animation is very
                // bad. But I'm doing it anyways.
                if dino.state == DinoState::Bonking {
                    if (delta / animation.frame_duration) as i32 >= 2 {
                        let width = 4.;
                        let height = 4.;
                        let pos = transform.translation();
                        entities.build_entity()
                            .with(
                                DamageEffect {
                                    value: 15,
                                    targets: Team::Enemy,
                                    rect: Rectangle {
                                        x1: pos[0] - width / 2.,
                                        x2: pos[0] + width / 2.,
                                        y1: pos[1] - height / 2.,
                                        y2: pos[1] + height / 2.,
                                    },
                                },
                                &mut damage_effects,
                            )
                            .with(
                                DespawnTimer {
                                    deadline: elapsed_time + 6,
                                },
                                &mut despawn_timers,
                            )
                            .build();
                    }
                }

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
