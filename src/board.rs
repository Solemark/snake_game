use crate::snake::Snake;

pub struct Board {
    board: [[char; 10]; 10],
    snake: Snake,
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
            snake: Snake::new(),
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();
        for row in {
            let mut b = self.board;
            b[self.snake.head.1][self.snake.head.0] = 'S';
            b
        } {
            for i in row {
                output.push(i);
            }
            output.push('\n');
        }
        output
    }

    pub fn parse(&mut self, res: String) {
        match res.trim() {
            "W" => self.validate_move((self.snake.head.0, self.snake.head.1 - 1)),
            "S" => self.validate_move((self.snake.head.0, self.snake.head.1 + 1)),
            "A" => self.validate_move((self.snake.head.0 - 1, self.snake.head.1)),
            "D" => self.validate_move((self.snake.head.0 + 1, self.snake.head.1)),
            _ => println!("invalid move"),
        }
    }

    fn validate_move(&mut self, head: (usize, usize)) {
        if self.board[head.1][head.0] != 'W' {
            self.snake.head = head
        }
    }
}
