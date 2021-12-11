pub mod packages;
pub use self::packages::ZombrrPackages;

mod state;
pub use self::state::{ArenaState, MenuState, ZombrrState};

mod options;
pub use self::options::ArenaOptions;

mod object;
pub use self::object::ZombrrObject;

pub struct PlayerRoot;
pub struct EnemyRoot;
pub struct WeaponRoot;
pub struct WeaponEntity;
pub struct CharacterRoot;
pub struct CharacterEntity;
