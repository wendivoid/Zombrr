use bevy::prelude::*;
use bevy::utils::HashMap;

mod apply;
mod death;
mod events;
pub use self::events::*;

mod health;
pub use self::health::Health;

mod plugin;
pub use self::plugin::DamagePlugin;

mod bullet_holes;
pub use self::bullet_holes::BulletHoles;

#[derive(Default)]
pub struct KillCount(pub HashMap<Entity, usize>);

pub struct ShowsDamage;
