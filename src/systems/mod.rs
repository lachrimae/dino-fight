pub mod move_hero;
pub mod animate_dinos;
pub mod health;
pub mod despawn;

pub use self::move_hero::HeroMovementSystem;
pub use self::animate_dinos::DinoAnimationSystem;
pub use self::health::HealthSystem;
pub use self::despawn::DespawnSystem;
