use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, WriteStorage, System},
    renderer::SpriteRender,
};

use crate::dino::{Team, Dino, HealthBar, DamageEffect};

pub struct HealthSystem {}

impl<'s> System<'s> for HealthSystem {
    type SystemData = (
        ReadStorage<'s, DamageEffect>,
        WriteStorage<'s, HealthBar>,
        WriteStorage<'s, Dino>,
        Read<'s, Time>
    );

    fn run(&mut self, (damageEffects, mut healthBars, mut dinos, time): Self::SystemData) {
        for damageEffect in (&damageEffects).join() {
            for (healthBar, _dino) in (&mut healthBars, &mut dinos).join() {
                let overlapping = overlapRect(damageEffect.rect, healthBar.rect);
                let hostile = damageEffect.targets == healthBar.allegiance;
                if overlapping && hostile {
                    if healthBar.damageableAt <= time.frame_number() {
                        if damageEffect.value >= healthBar.value {
                            healthBar.value = 0;
                            // delete the dino
                        } else {
                            healthBar.value = std::cmp::max(0, healthBar.value - damageEffect.value);
                        }
                    }
                }
            }
        }
    }
}

fn overlapRect(a: (), b: ()) -> bool {
    true
}
