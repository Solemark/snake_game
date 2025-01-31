use crate::board::Board;
use std::io::{stdin, stdout, Write};

pub fn play() {
    let mut board = Board::new();
    loop {
        // TODO - TUI instead of reprint
        println!("{}", board.display());
        print!("enter move (wasd): ");
        board.parse(read().to_uppercase());
    }
}

// TODO - Capture keyboard instead of this
fn read() -> String {
    let mut out = String::new();
    stdout().flush().expect("failed to flush");
    stdin().read_line(&mut out).expect("failed to read");
    out
}
