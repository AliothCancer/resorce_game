use crate::object::{player::Player, world::World};

pub struct Game {
    player: Player,
    world: World,
}

impl Game {
    pub fn new(player: Player, world: World) -> Game {
        Game { player, world }
    }

    pub fn start() {}

    fn input_handler() {}
}
