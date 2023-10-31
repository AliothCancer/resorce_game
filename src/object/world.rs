pub struct World {
    size: (u32, u32),
}

impl World {
    pub fn new(size: (u32, u32)) -> World {
        World { size }
    }
}
