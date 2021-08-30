use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, ReadStorage, WriteStorage, System},
};

use crate::dino::{HealthBar, DamageEffect};
use crate::geometry;

pub struct HealthSystem {}

impl<'s> System<'s> for HealthSystem {
    type SystemData = (
        ReadStorage<'s, DamageEffect>,
        WriteStorage<'s, HealthBar>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (damage_effects, mut health_bars, time, entities): Self::SystemData) {
        for damage_effect in (&damage_effects).join() {
            for (health_bar, entity) in (&mut health_bars, &*entities).join() {
                let overlapping = geometry::rects_overlap(&damage_effect.rect, &health_bar.rect);
                let hostile = damage_effect.targets == health_bar.allegiance;
                if overlapping && hostile {
                    let frame_num = time.frame_number();
                    if health_bar.damageable_at <= frame_num {
                        if damage_effect.value >= health_bar.value {
                            match entities.delete(entity) {
                                Ok(_) => {},
                                Err(err) => println!("{:?}", err),
                            }
                        } else {
                            health_bar.value = health_bar.value - damage_effect.value;
                            health_bar.damageable_at = frame_num + 100;
                            println!("A health bar dropped to {:?}", health_bar.value);
                        }
                    }
                }
            }
        }
    }
}
