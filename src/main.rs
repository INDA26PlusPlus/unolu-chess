
use tjack::Game;
//use ggez::;

fn main() {
    // init gameboard
    let gameboard = tjack::Game::new();
    // get board representation
    gameboard.get_matrix_board_repr();
}
