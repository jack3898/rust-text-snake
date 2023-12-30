use crate::coordinate::Coordinate;

use super::{
    apple::Apple,
    entity_manager::EntityManager,
    entity_type::EntityType,
    game_state::GameState,
    powerup::PowerupType,
    snake::{Snake, SnakeDirection},
    supersnake::Supersnake,
};

pub struct Game<'a> {
    entity_manager: EntityManager<'a, EntityType>,
    snake: Vec<Coordinate>,
    score: usize,
    playfield_x: usize,
    playfield_y: usize,
    current_direction: SnakeDirection, // Only updates next game tick
    next_direction: SnakeDirection,    // Queues up for next game tick
    state: GameState,
    current_powerup: PowerupType,
}

impl Game<'_> {
    pub fn new(playfield_x: usize, playfield_y: usize) -> Self {
        Self {
            entity_manager: EntityManager::new(),
            snake: vec![Coordinate::new(0, 0)],
            playfield_x: playfield_x,
            playfield_y: playfield_y,
            current_direction: SnakeDirection::Right,
            next_direction: SnakeDirection::Right,
            state: GameState::Playing,
            score: 0,
            current_powerup: PowerupType::None,
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

        match self.current_powerup {
            PowerupType::Supersnake { ref mut duration } => {
                *duration -= 1;

                if *duration == 0 {
                    self.current_powerup = PowerupType::None;
                }
            }
            PowerupType::None => {}
        }

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

        if self.snake_eating_itself()
            && !matches!(self.current_powerup, PowerupType::Supersnake { .. })
        {
            self.state = GameState::GameOver {
                score: self.score,
                message: "You ate yourself! Press 'r' to restart.".to_string(),
            };

            return &self.state;
        }

        if self.get_apple().is_some() {
            self.snake_remove_tail();
        }

        if &self.score % 20 == 0 && self.score >= 20 {
            self.add_supersnake(self.playfield_x, self.playfield_y);
            self.score += 1;
        }

        if self.snake_eating_supersnake() {
            self.remove_supersnake();
            self.current_powerup = PowerupType::Supersnake { duration: 100 };
        }

        if self.snake_eating_apple() {
            self.remove_apple();
            self.score += 1;
        } else {
            if self.get_apple().is_none() {
                self.add_apple(self.playfield_x, self.playfield_y);
            }
        }

        self.state = GameState::Playing;

        &self.state
    }

    fn snake_eating_apple(&mut self) -> bool {
        let apple = self.get_apple();

        apple
            .map(|apple| {
                self.snake_get_head()
                    .map(|head| head.intersects(apple))
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    fn snake_eating_supersnake(&mut self) -> bool {
        let supersnake = self.get_supersnake();

        supersnake
            .map(|supersnake| {
                self.snake_get_head()
                    .map(|head| head.intersects(supersnake))
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    pub fn get_powerup(&self) -> &PowerupType {
        &self.current_powerup
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn start_over(&mut self) {
        *self = Self::new(self.playfield_x, self.playfield_y);
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

impl Snake for Game<'_> {
    fn get_snake(&self) -> &Vec<Coordinate> {
        &self.snake
    }

    fn get_snake_mut(&mut self) -> &mut Vec<Coordinate> {
        &mut self.snake
    }

    fn snake_get_direction(&self) -> &SnakeDirection {
        &self.current_direction
    }

    fn snake_set_direction(&mut self, direction: SnakeDirection) {
        self.next_direction = direction;
    }
}

impl Apple for Game<'_> {
    fn get_apple(&self) -> Option<&Coordinate> {
        self.entity_manager
            .get_entity("apple")
            .map(|apple| match apple {
                EntityType::Apple { coordinates, .. } => coordinates.as_ref().unwrap(),
                _ => panic!("Expected apple"),
            })
    }

    fn set_apple(&mut self, coordinates: Coordinate) {
        self.entity_manager
            .add_entity("apple", EntityType::new_apple(coordinates));
    }

    fn remove_apple(&mut self) {
        self.entity_manager.remove_entity("apple");
    }

    fn get_apple_no_go_zones(&self) -> &Vec<Coordinate> {
        &self.snake
    }
}

impl Supersnake for Game<'_> {
    fn get_supersnake(&self) -> Option<&Coordinate> {
        self.entity_manager
            .get_entity("supersnake")
            .map(|supersnake| match supersnake {
                EntityType::Supersnake { coordinates, .. } => coordinates.as_ref().unwrap(),
                _ => panic!("Expected supersnake"),
            })
    }

    fn set_supersnake(&mut self, coordinates: Coordinate) {
        self.entity_manager
            .add_entity("supersnake", EntityType::new_supersnake(coordinates));
    }

    fn remove_supersnake(&mut self) {
        self.entity_manager.remove_entity("supersnake");
    }

    fn get_supersnake_no_go_zones(&self) -> &Vec<Coordinate> {
        &self.snake
    }
}
