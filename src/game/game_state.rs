pub enum GameState {
    Intro,
    Playing,
    GameOver { score: usize, message: String },
    Paused,
}
