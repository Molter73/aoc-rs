#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Position {
        Position { x, y }
    }

    pub fn update(&mut self, c: char) -> Result<(), char> {
        match c {
            '>' => self.y += 1,
            '<' => self.y -= 1,
            '^' => self.x += 1,
            'v' => self.x -= 1,
            _ => return Err(c),
        }
        Ok(())
    }
}
