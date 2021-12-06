use std::fs::File;
use std::path::{PathBuf, Path};

use super::WeaponMeta;

#[derive(Debug, PartialEq, Clone)]
pub struct Weapon {
    pub name: String,
    pub path: PathBuf,
    pub meta: WeaponMeta
}

impl Weapon {

    pub fn load<P: AsRef<Path>>(weapon_path: P) -> Result<Weapon, ron::Error> {
        let name = weapon_path.as_ref().file_name().unwrap().to_string_lossy().to_string();
        let path = weapon_path.as_ref().to_path_buf();
        let weapon = Weapon {
            name,
            path,
            meta: Self::load_meta(&weapon_path)?
        };
        tracing::debug!("Loading Weapon `{}`\n\t-> Name = {}\n\t-> Path = {:?}",
            weapon.name,
            weapon.name,
            weapon.path,
        );
        Ok(weapon)
    }

    pub fn load_meta<P: AsRef<Path>>(path: P) -> Result<WeaponMeta, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("meta.ron");
        let reader = File::open(&path)?;
        ron::de::from_reader(reader)
    }

    pub fn text_label(&self) -> &str {
        self.meta.label.as_ref().unwrap_or(&self.name)
    }

    pub fn scene_file(&self) -> String {
        let mut path = self.path.clone();
        path.push(&self.meta.scene);
        format!("{}#Scene0", path.to_str().unwrap())
    }
}
