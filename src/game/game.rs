use rand::Rng;

use crate::coordinate::Coordinate;

use super::{
    game_state::GameState,
    snake::{Snake, SnakeDirection},
};

pub struct Game {
    snake: Vec<Coordinate>,
    apple: Option<Coordinate>,
    score: usize,
    playfield_x: usize,
    playfield_y: usize,
    current_direction: SnakeDirection, // Only updates next game tick
    next_direction: SnakeDirection,    // Queues up for next game tick
    state: GameState,
}

impl Game {
    pub fn new(playfield_x: usize, playfield_y: usize) -> Self {
        Self {
            snake: vec![Coordinate::new(0, 0)],
            apple: Some(Coordinate::new(
                rand::thread_rng().gen_range(0..playfield_x),
                rand::thread_rng().gen_range(0..playfield_y),
            )),
            playfield_x: playfield_x,
            playfield_y: playfield_y,
            current_direction: SnakeDirection::Right,
            next_direction: SnakeDirection::Right,
            state: GameState::Playing,
            score: 0,
        }
    }

    pub fn next(&mut self) -> &GameState {
        match self.state {
            GameState::Playing => {
                self.next_playing();
                self.get_state()
            }
            GameState::Paused => self.get_state(),
            GameState::GameOver { .. } => self.get_state(),
        }
    }

    fn next_playing(&mut self) -> &GameState {
        self.current_direction = self.next_direction;

        match self.snake_add_head(self.playfield_x, self.playfield_y) {
            Ok(_) => {}
            Err(error) => {
                self.state = GameState::GameOver {
                    score: self.score,
                    message: format!("{} press 'r' to restart.", error),
                };

                return &self.state;
            }
        }

        if self.snake_eating_itself() {
            self.state = GameState::GameOver {
                score: self.score,
                message: "You ate yourself! Press 'r' to restart.".to_string(),
            };

            return &self.state;
        }

        if self.apple.is_some() {
            self.snake_remove_tail();
        }

        if self.snake_eating_apple() {
            self.apple = None;
            self.score += 1;
        } else {
            self.add_apple();
        }

        self.state = GameState::Playing;

        &self.state
    }

    fn add_apple(&mut self) {
        if self.apple.is_none() {
            self.apple = Some(Coordinate::new(
                rand::thread_rng().gen_range(0..self.playfield_x),
                rand::thread_rng().gen_range(0..self.playfield_y),
            ))
        }
    }

    fn snake_eating_apple(&mut self) -> bool {
        if let Some(apple) = self.get_apple() {
            let snake_head = self.snake_get_head();

            if let Some(snake_head) = snake_head {
                snake_head.intersects(&apple)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_apple(&self) -> Option<Coordinate> {
        self.apple
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn start_over(&mut self) {
        self.snake = vec![Coordinate::new(0, 0)];
        self.score = 0;
        self.current_direction = SnakeDirection::Right;
        self.next_direction = SnakeDirection::Right;
        self.state = GameState::Playing;

        self.add_apple();
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn pause(&mut self) {
        self.state = GameState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = GameState::Playing;
    }
}

impl Snake for Game {
    fn get_snake(&mut self) -> &mut Vec<Coordinate> {
        &mut self.snake
    }

    fn snake_get_direction(&self) -> &SnakeDirection {
        &self.current_direction
    }

    fn snake_set_direction(&mut self, direction: SnakeDirection) {
        self.next_direction = direction;
    }
}
