pub struct Player {
    name: String,
    position: (u32, u32),
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            position: (0, 0),
        }
    }
}
