pub mod packages;
pub use self::packages::ZombrrPackages;

mod state;
pub use self::state::{ArenaState, MenuState, ZombrrState};

mod options;
pub use self::options::ArenaOptions;

mod object;
pub use self::object::ZombrrObject;

pub struct BulletHoles {
    pub disappear_after: Option<u64>
}

impl Default for BulletHoles {
    fn default() -> BulletHoles {
        BulletHoles {
            disappear_after: Some(5)
        }
    }
}
