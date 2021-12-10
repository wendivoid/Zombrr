#[derive(Debug, PartialEq, Clone)]
pub struct DisplayRef {
    pub namespace: String,
    pub package: String,
    pub name: String,
}

impl Default for DisplayRef {
    fn default() -> DisplayRef {
        DisplayRef {
            namespace: "zombrr".into(),
            package: "debug".into(),
            name: "debug".into(),
        }
    }
}
