mod dino;
mod systems;
mod geometry;

extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
};

use crate::dino::DinoFight;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::DespawnSystem {}, "despawn_system", &[])
        .with(systems::HitboxTrackingSystem {}, "hitbox_tracking_system", &[])
        .with(systems::DinoAnimationSystem {}, "dino_animation_system", &["despawn_system"])
        .with(systems::AiIntentSystem {}, "ai_intent_system", &["hitbox_tracking_system"])
        .with(systems::DinoMotionSystem {}, "dino_motion_system", &["ai_intent_system", "dino_animation_system"])
        .with(
            systems::HeroIntentSystem {},
            "hero_intent_system",
            &["input_system"]
        )
        .with(systems::HealthSystem {}, "health_system", &["dino_animation_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, DinoFight::default(), game_data)?;
    game.run();

    Ok(())
}
