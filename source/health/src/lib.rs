mod apply;

mod health;
pub use self::health::Health;

mod death;
pub use self::death::Death;

mod damage;
pub use self::damage::Damage;

mod plugin;
pub use self::plugin::HealthPlugin;

#[derive(Default)]
pub struct KillCount(pub bevy::utils::HashMap<bevy::prelude::Entity, usize>);
