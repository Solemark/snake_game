mod board;
mod game;
mod snake;
use crate::game::Game;
/**
PLAN:
* Make snake food
* Make snake grow when eating food
* Impliment death
*/
fn main() {
    Game::new().run()
}
