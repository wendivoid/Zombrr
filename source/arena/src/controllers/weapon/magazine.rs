use bevy::reflect::Reflect;

#[derive(Debug, Reflect, Copy, Clone)]
pub struct Magazine {
    pub count: usize,
    pub length: usize,
    pub used: usize,
}

impl Magazine {
    pub fn fire(&mut self) -> bool {
        if self.count * self.length <= self.used {
            false
        } else {
            self.used += 1;
            true
        }
    }
}
