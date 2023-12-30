#[derive(Clone, Copy)]
pub enum Characters {
    SnakeHead,
    SnakeBody,
    SnakeBodySuper,
    Obstacle,
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
            Characters::Obstacle => '🟥',
            Characters::Grass => '➕',
            Characters::Apple => '🍏',
            Characters::Supersnake => '🐍',
        }
    }
}
