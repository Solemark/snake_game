mod board;
mod game;
use crate::game::Game;

fn main() {
    Game::new().run()
}
