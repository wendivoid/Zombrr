use bevy::prelude::*;
use bevy::utils::HashMap;

mod apply;
mod death;
mod events;
pub use self::events::*;
mod components;
pub use self::components::*;

mod plugin;
pub use self::plugin::DamagePlugin;

#[derive(Default)]
pub struct KillCount(HashMap<Entity, usize>);
