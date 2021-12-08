use std::fs::File;
use std::path::{PathBuf, Path};

use super::MapMeta;

#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    pub name: String,
    pub path: PathBuf,
    pub meta: MapMeta
}

impl Map {

    pub fn load<P: AsRef<Path>>(map_path: P) -> Result<Map, ron::Error> {
        let name = map_path.as_ref().file_name().unwrap().to_string_lossy().to_string();
        let path = map_path.as_ref().to_path_buf();
        let map = Map { name, path, meta: Self::load_meta(&map_path)? };
        tracing::debug!("Loading Map `{}`\n\t-> Name = {}\n\t-> Path = {:?}\n\t-> Priority = {}",
            map.name,
            map.name,
            map.path,
            map.meta.priority
        );
        Ok(map)
    }

    pub fn load_meta<P: AsRef<Path>>(path: P) -> Result<MapMeta, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("meta.ron");
        let reader = File::open(&path)?;
        ron::de::from_reader(reader)
    }

    pub fn text_label(&self) -> &str {
        self.meta.label.as_ref().unwrap_or(&self.name)
    }
}
