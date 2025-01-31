// TODO - Snake body grows
pub struct Snake {
    // head: x, y
    pub head: (usize, usize),
    length: u8,
}
impl Snake {
    pub fn new() -> Self {
        Snake {
            head: (3, 5),
            length: 1,
        }
    }
}
