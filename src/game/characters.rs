#[derive(Clone, Copy)]
pub enum Characters {
    SnakeHead,
    SnakeBody,
    SnakeBodySuper,
    Obstacle,
    Grass,
    Apple,
    SupersnakePwrup,
    SnakeBodySlow,
    SlowdownPwrup,
}

impl Characters {
    pub fn value(&self) -> char {
        match self {
            Characters::SnakeHead => '👀',
            Characters::SnakeBody => '🟩',
            Characters::SnakeBodySuper => '🟦',
            Characters::SnakeBodySlow => '🟨',
            Characters::Obstacle => '🟥',
            Characters::Grass => '➕',
            Characters::Apple => '🍏',
            Characters::SupersnakePwrup => '🐍',
            Characters::SlowdownPwrup => '🐌',
        }
    }
}
