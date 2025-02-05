use std::process::exit;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::prelude::*;

pub struct Board {
    board: [[char; 10]; 10],
    /// snake: (head: (x, y), length)
    pub snake: (usize, usize, usize),
    /// previous head locations * TODO - Body grows infinately?
    body: Vec<(usize, usize)>,
    /// should a new fruit be generated?
    new_fruit: bool,
    /// location of current fruit
    fruit: (usize, usize),
}
impl Board {
    pub fn new() -> Self {
        Board {
            board: [
                ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', 'W'],
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', 'W'],
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', 'W'],
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', 'W'],
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', 'W'],
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', 'W'],
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', 'W'],
                ['W', '_', '_', '_', '_', '_', '_', '_', '_', 'W'],
                ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
            ],
            snake: (3, 5, 0),
            body: Vec::new(),
            new_fruit: true,
            fruit: (6, 5),
        }
    }

    /// display current board state
    pub fn display(&mut self) -> String {
        let mut output = String::new();
        for row in self.generate_board() {
            for i in row {
                output.push(i);
            }
            output.push('\n');
        }
        if self.body.len() > 10 {
            self.trim_body();
        }
        output
    }

    /// trim the body to prevent the Vec<(usize, usize)> growing infinately
    fn trim_body(&mut self) {
        self.body = self
            .body
            .clone()
            .into_iter()
            .rev()
            .take(11)
            .collect::<Vec<(usize, usize)>>()
            .into_iter()
            .rev()
            .collect();
    }

    /// generate the current state of the board
    fn generate_board(&mut self) -> [[char; 10]; 10] {
        let mut b = self.board;
        //where is the snake?
        b[self.snake.1][self.snake.0] = 'S';
        let mut i = self.snake.2;
        for j in self.body.iter().rev() {
            if i <= 0 {
                break;
            } else {
                b[j.1][j.0] = 'S';
                i -= 1;
            }
        }
        //where is the fruit?
        if self.new_fruit {
            self.new_fruit = false;
            self.fruit = self.generate_fruit(&b);
        }
        b[self.fruit.1][self.fruit.0] = '@';
        b
    }

    /// generate new fruit location
    fn generate_fruit(&self, b: &[[char; 10]; 10]) -> (usize, usize) {
        loop {
            let x = rand::rng().random_range(1..9);
            let y = rand::rng().random_range(1..9);
            // cannot place on existing fruit location, cannot place on existing body, must be blank space
            if (x, y) != self.fruit && b[y][x] == '_' && !self.body.contains(&(x, y)) {
                return (x, y);
            }
        }
    }

    /// parse input
    pub fn parse(&mut self, res: KeyEvent) {
        if res.code == KeyCode::Char('c') && res.modifiers == KeyModifiers::CONTROL {
            exit(0);
        }
        match res.code.to_string().trim() {
            "w" => self.handle_move((self.snake.0, self.snake.1 - 1)),
            "s" => self.handle_move((self.snake.0, self.snake.1 + 1)),
            "a" => self.handle_move((self.snake.0 - 1, self.snake.1)),
            "d" => self.handle_move((self.snake.0 + 1, self.snake.1)),
            _ => println!("invalid move"),
        }
    }

    /// update the snake, board and fruit
    fn handle_move(&mut self, head: (usize, usize)) {
        self.snake.0 = head.0;
        self.snake.1 = head.1;
        self.body.push((self.snake.0, self.snake.1));
        if (self.snake.0, self.snake.1) == self.fruit {
            self.new_fruit = true;
            self.snake.2 += 1;
        }
    }

    /// is the board in a loss state?
    pub fn game_loss(&self) -> bool {
        if self.board[self.snake.0][self.snake.1] == 'W' {
            return true;
        }
        let mut l = self.snake.2;
        for i in self.body.iter().rev() {
            if l <= 0 {
                break;
            }
            if *i == (self.snake.0, self.snake.1) {
                return true;
            }
            l -= 1;
        }
        false
    }

    /// is the board in a win state?
    pub fn game_win(&self) -> bool {
        if self.snake.2 >= 10 {
            true
        } else {
            false
        }
    }
}
