use crate::board::Board;
use crossterm::event::{read, Event};
use ratatui::text::Text;

pub struct Game {
    board: Board,
}
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
        }
    }
    pub fn run(&mut self) {
        let mut terminal = ratatui::init();
        loop {
            // Clear before drawing to prevent artifacting
            terminal.clear().expect("failed to render");
            terminal
                .draw(|f| {
                    let text = Text::raw(self.board.display());
                    f.render_widget(text, f.area());
                })
                .expect("failed to draw frame");
            self.get_input();
        }
    }

    /// Waits for a keypress, is blocking the main thread
    fn get_input(&mut self) {
        match read().unwrap() {
            Event::Key(e) => self.board.parse(e),
            _ => (),
        }
    }
}
