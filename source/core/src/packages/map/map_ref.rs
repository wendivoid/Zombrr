#[derive(Debug, PartialEq, Clone)]
pub struct MapRef {
    pub namespace: String,
    pub package: String,
    pub name: String,
}

impl Default for MapRef {
    fn default() -> MapRef {
        MapRef {
            namespace: "zombrr".into(),
            package: "debug".into(),
            name: "enclosed".into(),
        }
    }
}
