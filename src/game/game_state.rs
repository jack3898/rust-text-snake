pub enum GameState {
    Playing,
    GameOver { score: usize, message: String },
    Paused,
}
