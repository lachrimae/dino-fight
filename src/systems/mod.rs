pub mod hero_intent;
pub mod animate_dinos;
pub mod health;
pub mod despawn;
pub mod ai_intent;
pub mod dino_motion;
pub mod track_hitboxes;

pub use self::animate_dinos::DinoAnimationSystem;
pub use self::health::HealthSystem;
pub use self::despawn::DespawnSystem;
pub use self::ai_intent::AiIntentSystem;
pub use self::hero_intent::HeroIntentSystem;
pub use self::dino_motion::DinoMotionSystem;
pub use self::track_hitboxes::HitboxTrackingSystem;
