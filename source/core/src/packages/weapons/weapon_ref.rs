#[derive(Debug, PartialEq, Clone)]
pub struct WeaponRef {
    pub namespace: String,
    pub package: String,
    pub name: String
}

impl Default for WeaponRef {
    fn default() -> WeaponRef {
        WeaponRef {
            namespace: "zombrr".into(),
            package: "zombrr".into(),
            name: "m4a1".into()
        }
    }
}
