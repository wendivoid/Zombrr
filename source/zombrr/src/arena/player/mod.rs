mod init;
mod cleanup;
mod cameras;

mod bundle;
pub use self::bundle::PlayerBundle;

mod plugin;
pub use self::plugin::PlayerPlugin;

pub struct PlayerRoot;
