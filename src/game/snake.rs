use thiserror::Error;

use crate::coordinate::Coordinate;

#[derive(PartialEq, Copy, Clone)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Error, Debug)]
pub enum SnakeError {
    #[error("Snake hit a wall!")]
    HitWall,
    #[error("No head found for the snake.")]
    NoHead,
}

pub trait Snake {
    fn get_snake(&mut self) -> &mut Vec<Coordinate>;

    fn snake_get_head(&mut self) -> Option<&Coordinate> {
        self.get_snake().last()
    }

    fn snake_get_body(&mut self) -> Vec<Coordinate> {
        let snake = self.get_snake();

        if snake.len() == 0 {
            return vec![];
        }

        snake[..snake.len() - 1].to_vec()
    }

    fn snake_eating_itself(&mut self) -> bool {
        let snake_head = self.snake_get_head().unwrap().clone();
        let snake_body = self.snake_get_body();

        snake_head.intersects_multiple(&snake_body)
    }

    fn get_length(&mut self) -> usize {
        self.get_snake().len()
    }

    fn snake_get_direction(&self) -> &SnakeDirection;

    fn snake_set_direction(&mut self, direction: SnakeDirection);

    fn snake_add_head(&mut self, max_x: usize, max_y: usize) -> Result<(), SnakeError> {
        let snake_direction = self.snake_get_direction().clone();
        let (snake_head_x, snake_head_y) =
            self.snake_get_head().ok_or(SnakeError::NoHead)?.as_tuple();
        let snake = self.get_snake();

        let new_head_location = match snake_direction {
            SnakeDirection::Right if snake_head_x < max_x - 1 => {
                Some(Coordinate::new(snake_head_x + 1, snake_head_y))
            }
            SnakeDirection::Left if snake_head_x > 0 => {
                Some(Coordinate::new(snake_head_x - 1, snake_head_y))
            }
            SnakeDirection::Up if snake_head_y > 0 => {
                Some(Coordinate::new(snake_head_x, snake_head_y - 1))
            }
            SnakeDirection::Down if snake_head_y < max_y - 1 => {
                Some(Coordinate::new(snake_head_x, snake_head_y + 1))
            }
            _ => None,
        };

        if let Some(new_head_location) = new_head_location {
            snake.push(new_head_location);

            return Ok(());
        }

        return Err(SnakeError::HitWall);
    }

    fn snake_remove_tail(&mut self) {
        let snake = self.get_snake();

        if snake.len() > 0 {
            snake.remove(0);
        }
    }
}
