mod game_logic;
mod object;

use game_logic::Game;
use object::{player::Player, world::World};

fn main() {
    let player = Player::new("Jahanjir".to_string());
    let world = World::new((100, 100));

    let game = Game::new(player, world);

    game.start();
}
