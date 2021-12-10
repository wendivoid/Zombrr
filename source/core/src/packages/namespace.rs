use serde::{Deserialize, Serialize};

use std::fs::{read_dir, File};
use std::path::Path;

use super::Package;

#[derive(Debug, PartialEq, Clone)]
pub struct Namespace {
    pub name: String,
    pub meta: Option<NamespaceMeta>,
    pub packages: Vec<Package>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamespaceMeta {
    pub label: Option<String>,
}

impl Namespace {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Namespace, ron::Error> {
        let name = path
            .as_ref()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let mut meta = None;
        let mut packages = vec![];
        for namespace in read_dir(path)? {
            let entry = namespace?;
            if entry.file_name() == "meta.ron" {
                let reader = File::open(entry.path())?;
                meta = Some(ron::de::from_reader(reader)?);
            } else if entry.metadata()?.is_dir() {
                packages.push(Package::load(entry.path())?);
            }
        }
        Ok(Namespace {
            meta,
            packages,
            name,
        })
    }
}
