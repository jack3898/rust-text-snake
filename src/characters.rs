#[derive(Clone, Copy)]
pub enum Characters {
    SnakeHead,
    SnakeBody,
    SnakeBodySuper,
    Grass,
    Apple,
    Supersnake,
}

impl Characters {
    pub fn value(&self) -> char {
        match self {
            Characters::SnakeHead => '👀',
            Characters::SnakeBody => '🟩',
            Characters::SnakeBodySuper => '🟦',
            Characters::Grass => '➕',
            Characters::Apple => '🍎',
            Characters::Supersnake => '🐍',
        }
    }
}
