pub struct BulletHoles {
    pub disappear_after: Option<u64>
}

impl Default for BulletHoles {
    fn default() -> BulletHoles {
        BulletHoles {
            disappear_after: Some(5)
        }
    }
}
