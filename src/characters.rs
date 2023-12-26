#[derive(Clone, Copy)]
pub enum Characters {
    SnakeHead,
    SnakeBody,
    Grass,
    Apple,
}

impl Characters {
    pub fn value(&self) -> char {
        match self {
            Characters::SnakeHead => '❎',
            Characters::SnakeBody => '🟩',
            Characters::Grass => '🟫',
            Characters::Apple => '🍎',
        }
    }
}
