use std::fs::File;
use std::path::{PathBuf, Path};

use super::CharacterMeta;

#[derive(Debug, PartialEq, Clone)]
pub struct Character {
    pub name: String,
    pub path: PathBuf,
    pub meta: CharacterMeta
}

impl Character {

    pub fn load<P: AsRef<Path>>(character_path: P) -> Result<Character, ron::Error> {
        let name = character_path.as_ref().file_name().unwrap().to_string_lossy().to_string();
        let path = character_path.as_ref().to_path_buf();
        tracing::debug!("Loading character: `{}` {:?}", name, path);
        let character = Character {
            name,
            path,
            meta: Self::load_meta(&character_path)?
        };
        Ok(character)
    }

    pub fn load_meta<P: AsRef<Path>>(path: P) -> Result<CharacterMeta, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("meta.ron");
        let reader = File::open(&path)?;
        ron::de::from_reader(reader)
    }

    pub fn text_label(&self) -> &str {
        self.meta.label.as_ref().unwrap_or(&self.name)
    }
}