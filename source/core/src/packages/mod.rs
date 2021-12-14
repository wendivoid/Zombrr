mod weapons;
pub use self::weapons::{Weapon, WeaponMeta, WeaponRef, WeaponAction, Magazine};

mod displays;
pub use self::displays::{Display, DisplayMeta, DisplayRef};

mod characters;
pub use self::characters::{Character, CharacterMeta, CharacterRef};

mod map;
pub use self::map::{AmbientLight, Map, MapData, MapMeta, MapRef, SkyPreset, Sky};

mod color;
pub use self::color::Color;

mod namespace;
pub use self::namespace::{Namespace, NamespaceMeta};

mod package;
pub use self::package::{Package, PackageMeta};

#[derive(Default, Debug, PartialEq)]
pub struct ZombrrPackages(pub Vec<Namespace>);

impl ZombrrPackages {
    pub fn load<P: AsRef<std::path::Path>>(&mut self, packages_dir: P) -> Result<(), ron::Error> {
        for entry in std::fs::read_dir(packages_dir)? {
            let entry = entry?;
            self.0.push(Namespace::load(entry.path())?);
        }
        Ok(())
    }

    pub fn get_map(&self, map: &MapRef) -> Option<&Map> {
        for namespace in &self.0 {
            if namespace.name == map.namespace {
                for package in &namespace.packages {
                    if package.name == map.package {
                        for p_map in &package.maps {
                            if p_map.name == map.name {
                                return Some(p_map);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_character(&self, character: &CharacterRef) -> Option<&Character> {
        for namespace in &self.0 {
            if namespace.name == character.namespace {
                for package in &namespace.packages {
                    if package.name == character.package {
                        for p_character in &package.characters {
                            if p_character.name == character.name {
                                return Some(p_character);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_weapon(&self, weapon: &WeaponRef) -> Option<&Weapon> {
        for namespace in &self.0 {
            if namespace.name == weapon.namespace {
                for package in &namespace.packages {
                    if package.name == weapon.package {
                        for p_weapon in &package.weapons {
                            if p_weapon.name == weapon.name {
                                return Some(p_weapon);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_display(&self, display: &DisplayRef) -> Option<&Display> {
        for namespace in &self.0 {
            if namespace.name == display.namespace {
                for package in &namespace.packages {
                    if package.name == display.package {
                        for p_display in &package.displays {
                            if p_display.name == display.name {
                                return Some(p_display);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_maps(
        &self,
    ) -> impl Iterator<
        Item = (
            &str,
            impl Iterator<Item = (&str, impl Iterator<Item = &Map>)>,
        ),
    > {
        self.0.iter().map(|x| {
            (
                x.name.as_str(),
                x.packages.iter().map(|p| (p.name.as_str(), p.maps.iter())),
            )
        })
    }

    pub fn get_characters(
        &self,
    ) -> impl Iterator<
        Item = (
            &str,
            impl Iterator<Item = (&str, impl Iterator<Item = &Character>)>,
        ),
    > {
        self.0.iter().map(|x| {
            (
                x.name.as_str(),
                x.packages
                    .iter()
                    .map(|p| (p.name.as_str(), p.characters.iter())),
            )
        })
    }
}
