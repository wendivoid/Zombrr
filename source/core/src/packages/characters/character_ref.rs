#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRef {
    pub namespace: String,
    pub package: String,
    pub name: String
}

impl Default for CharacterRef {
    fn default() -> CharacterRef {
        CharacterRef {
            namespace: "zombrr".into(),
            package: "debug".into(),
            name: "ybot".into()
        }
    }
}
