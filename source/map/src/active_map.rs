use zombrr_core::packages::Map;
use zombrr_core::{ZombrrPackages, ArenaOptions};

use bevy_ecs::prelude::*;
use bevy_ecs::system::{SystemParam, SystemParamState, SystemState, SystemParamFetch};

pub struct ActiveMap<'a>(pub &'a Map);

pub struct ActiveMapState;

impl<'a> SystemParam for ActiveMap<'a> {
    type Fetch = ActiveMapState;
}

unsafe impl SystemParamState for ActiveMapState {
    type Config = ();

    fn init(
        _world: &mut World,
        _system_state: &mut SystemState,
        _config: Self::Config
    ) -> Self {
        ActiveMapState
    }

    fn default_config() -> Self::Config {}
}

impl<'w> SystemParamFetch<'w> for ActiveMapState {
    type Item = ActiveMap<'w>;
    unsafe fn get_param(
        _state: &'w mut Self,
        _system_meta: &'w SystemState,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item {
        let options = world.get_resource::<ArenaOptions>().expect("Failed to get ArenaOptions resource");
        let packages = world.get_resource::<ZombrrPackages>().expect("Failed to get ZombrrPackages resources");
        match packages.get_map(&options.map) {
            None => panic!("Unable to fetch active map."),
            Some(map) => ActiveMap(map)
        }
    }
}
