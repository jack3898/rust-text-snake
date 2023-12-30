use std::collections::HashMap;

use crate::coordinates::Coordinates;

use super::{
    entity_type::EntityType,
    game_state::GameState,
    powerup::PowerupType,
    traits::{
        entity::Entity,
        snake::{Snake, SnakeDirection},
    },
};

pub struct Game {
    entities: HashMap<Coordinates, EntityType>,
    snake: Vec<Coordinates>,
    score: usize,
    playfield_x: usize,
    playfield_y: usize,
    current_direction: SnakeDirection, // Only updates next game tick
    next_direction: Vec<SnakeDirection>, // Queues up for next game tick
    state: GameState,
    current_powerup: PowerupType,
}

impl Game {
    pub fn new(playfield_x: usize, playfield_y: usize) -> Self {
        Self {
            entities: HashMap::new(),
            snake: Vec::from([Coordinates::new(0, 0)]),
            score: 0,
            playfield_x: playfield_x,
            playfield_y: playfield_y,
            current_direction: SnakeDirection::Right,
            next_direction: Vec::from([SnakeDirection::Right]),
            state: GameState::Intro,
            current_powerup: PowerupType::None,
        }
    }

    pub fn next(&mut self) -> &GameState {
        match self.state {
            GameState::Playing => {
                self.process_next_game_tick();
                self.get_state()
            }
            GameState::GameOver { .. } => self.get_state(),
            GameState::Intro => self.get_state(),
        }
    }

    fn process_next_game_tick(&mut self) -> &GameState {
        if self.next_direction.len() > 0 {
            let removed = self.next_direction.remove(0);

            self.current_direction = removed;
        }

        match self.snake_add_head(self.playfield_x, self.playfield_y) {
            Ok(_) => {}
            Err(error) => {
                self.state = GameState::GameOver {
                    score: self.score,
                    message: format!("{} press [R] to go back to the main menu.", error),
                };

                return &self.state;
            }
        }

        if !matches!(self.current_powerup, PowerupType::Supersnake { .. })
            && self.snake_eating_itself()
        {
            self.state = GameState::GameOver {
                score: self.score,
                message: "You ate yourself! Press [R] to restart.".to_string(),
            };

            return &self.state;
        }

        self.process_active_powerup();
        self.handle_eat_entity();
        self.generate_entities();

        &self.state
    }

    fn get_apple(&self) -> Option<&EntityType> {
        self.entities.iter().find_map(|(_, entity)| match entity {
            EntityType::Apple { .. } => Some(entity),
            _ => None,
        })
    }

    /// Identify if the snake is on a powerup and award it to the player on a match
    fn handle_eat_entity(&mut self) {
        match self.snake_on_entity() {
            Some(EntityType::Supersnake { coordinates, .. }) => {
                // Unwrapped because we know the snake has a head that is sitting on a powerup so it should always be Some
                self.entities.remove(&coordinates.unwrap());
                self.current_powerup = PowerupType::Supersnake { duration: 100 };
            }
            Some(EntityType::Apple { coordinates, .. }) => {
                self.entities.remove(&coordinates.unwrap());
                self.score += 1;
            }
            None => {
                self.snake_remove_tail();
            }
        };
    }

    /// Generate new powerups and apples under certain conditions
    fn generate_entities(&mut self) {
        let mut new_entities = vec![];

        if self.score % 20 == 0 && self.score > 0 {
            self.add_entity(
                |coords| {
                    new_entities.push(EntityType::new_supersnake(coords));
                },
                self.playfield_x,
                self.playfield_y,
            );
            self.score += 1;
        };

        if self.get_apple().is_none() {
            self.add_entity(
                |coords| {
                    new_entities.push(EntityType::new_apple(coords));
                },
                self.playfield_x,
                self.playfield_y,
            );
        };

        for entity in new_entities {
            let coordinates = &entity.get_coordinates().unwrap().clone();

            self.entities.insert(*coordinates, entity);
        }
    }

    // If there are any active powerups, process them and remove them if they have expired.
    fn process_active_powerup(&mut self) {
        match self.current_powerup {
            PowerupType::Supersnake { ref mut duration } => {
                *duration -= 1;

                if *duration == 0 {
                    self.current_powerup = PowerupType::None;
                }
            }
            PowerupType::None => {}
        }
    }

    /// Get the entity that the snake is currently on
    fn snake_on_entity(&self) -> Option<&EntityType> {
        let snake_head_coords = self.snake_get_head().expect("Snake has no head!");

        self.entities.get(snake_head_coords)
    }

    pub fn get_current_powerup(&self) -> &PowerupType {
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

    pub fn play(&mut self) {
        self.state = GameState::Playing;
    }
}

impl Snake for Game {
    fn get_snake(&self) -> &Vec<Coordinates> {
        &self.snake
    }

    fn get_snake_mut(&mut self) -> &mut Vec<Coordinates> {
        &mut self.snake
    }

    fn snake_get_direction(&self) -> &SnakeDirection {
        &self.current_direction
    }

    fn snake_set_direction(&mut self, direction: SnakeDirection) {
        self.next_direction.push(direction);
    }
}

impl Entity for Game {
    fn get_all_entities(&self) -> Vec<&EntityType> {
        self.entities
            .iter()
            .filter_map(|(_, entity)| match entity {
                EntityType::Apple { .. } => Some(entity),
                EntityType::Supersnake { .. } => Some(entity),
            })
            .collect()
    }

    fn remove_entity(&mut self) {
        let snake_head_coords = self.snake_get_head().unwrap().clone();

        self.entities.remove(&snake_head_coords);
    }

    fn get_entity_no_go_zones(&self) -> &Vec<Coordinates> {
        &self.snake
    }
}
