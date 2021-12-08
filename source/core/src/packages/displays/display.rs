use std::fs::File;
use std::path::{PathBuf, Path};

use super::DisplayMeta;

#[derive(Debug, PartialEq, Clone)]
pub struct Display {
    pub name: String,
    pub path: PathBuf,
    pub meta: DisplayMeta
}

impl Display {
    pub fn load<P: AsRef<Path>>(environment_path: P) -> Result<Display, ron::Error> {
        let name = environment_path.as_ref().file_name().unwrap().to_string_lossy().to_string();
        let path = environment_path.as_ref().to_path_buf();
        let environment = Display {
            name,
            path,
            meta: Self::load_meta(&environment_path)?
        };
        tracing::debug!("Loading Environment `{}`\n\t-> Name = {}\n\t-> Path = {:?}",
            environment.name,
            environment.name,
            environment.path,
        );
        Ok(environment)
    }

    pub fn load_meta<P: AsRef<Path>>(path: P) -> Result<DisplayMeta, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("meta.ron");
        let reader = File::open(&path)?;
        ron::de::from_reader(reader)
    }
}
