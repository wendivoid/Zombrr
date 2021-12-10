mod one;

mod plugin;
pub use self::plugin::ModesPlugin;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    None,
    OneEnemy,
    Custom(String),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::None
    }
}
