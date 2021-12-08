use serde::{ Serialize, Deserialize };

use std::fs::{read_dir, File};
use std::path::Path;

use super::{Map, Character, Weapon, Display};

#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    pub name: String,
    pub meta: Option<PackageMeta>,
    pub maps: Vec<Map>,
    pub characters: Vec<Character>,
    pub weapons: Vec<Weapon>,
    pub displays: Vec<Display>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PackageMeta {
    pub priority: usize
}

impl Package {

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Package, ron::Error> {
        let name = path.as_ref().file_name().unwrap().to_string_lossy().into_owned();
        Ok(Package {
            name,
            meta: Self::load_meta(&path)?,
            maps: Self::load_maps(&path)?,
            characters: Self::load_characters(&path)?,
            weapons: Self::load_weapons(&path)?,
            displays: Self::load_displays(path)?
        })
    }

    pub fn load_meta<P: AsRef<Path>>(path: P) -> Result<Option<PackageMeta>, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("meta.ron");
        if path.exists() {
          let reader = File::open(&path)?;
          Ok(Some(ron::de::from_reader(reader)?))
        } else {
          Ok(None)
        }
    }

    pub fn load_maps<P: AsRef<Path>>(path: P) -> Result<Vec<Map>, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("maps");
        let mut maps = vec![];
        if path.exists() {
            for entry in read_dir(&path)? {
                let entry = entry?;
                maps.push(Map::load(entry.path())?);
            }
        }
        Ok(maps)
    }

    pub fn load_weapons<P: AsRef<Path>>(path: P) -> Result<Vec<Weapon>, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("weapons");
        let mut weapons = vec![];
        if path.exists() {
            for entry in read_dir(&path)? {
                let entry = entry?;
                weapons.push(Weapon::load(entry.path())?);
            }
        }
        Ok(weapons)
    }

    pub fn load_displays<P: AsRef<Path>>(path: P) -> Result<Vec<Display>, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("displays");
        let mut displays = vec![];
        if path.exists() {
            for entry in read_dir(&path)? {
                let entry = entry?;
                displays.push(Display::load(entry.path())?);
            }
        }
        Ok(displays)
    }

    pub fn load_characters<P: AsRef<Path>>(path: P) -> Result<Vec<Character>, ron::Error> {
        let mut path = path.as_ref().to_path_buf();
        path.push("characters");
        let mut characters = vec![];
        if path.exists() {
            for entry in read_dir(&path)? {
                let entry = entry?;
                characters.push(Character::load(entry.path())?);
            }
        }
        Ok(characters)
    }
}
