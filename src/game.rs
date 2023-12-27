use rand::Rng;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type XYCoord = (usize, usize);

type Snake = Vec<XYCoord>;
type Apple = Option<XYCoord>;

pub struct Game {
    snake: Snake, // A list of the snakes parts using x/y coords
    apple: Apple,
    score: usize,
    playfield_x: usize,
    playfield_y: usize,
    direction: Direction,
}

impl Game {
    pub fn new(playfield_x: usize, playfield_y: usize) -> Self {
        Self {
            snake: vec![(0, 0)],
            apple: Some((
                rand::thread_rng().gen_range(0..playfield_x),
                rand::thread_rng().gen_range(0..playfield_y),
            )),
            playfield_x: playfield_x,
            playfield_y: playfield_y,
            direction: Direction::Right,
            score: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.add_snake_head() {
            panic!("Out of bounds!");
        }

        if self.snake_eating_itself() {
            panic!("Will handle this better another time. But game over!");
        }

        if self.apple.is_some() {
            self.remove_snake_tail()
        }

        if self.snake_eating_apple() {
            self.apple = None;
            self.score += 1;
        } else {
            self.add_apple();
        }
    }

    pub fn add_apple(&mut self) {
        if self.apple.is_none() {
            self.apple = Some((
                rand::thread_rng().gen_range(0..self.playfield_x),
                rand::thread_rng().gen_range(0..self.playfield_y),
            ))
        }
    }

    fn add_snake_head(&mut self) -> bool {
        let (snake_head_x, snake_head_y) = self.snake.last().unwrap();

        let new_head_location = match self.direction {
            Direction::Right if *snake_head_x < self.playfield_x - 1 => {
                Some((snake_head_x + 1, *snake_head_y))
            }
            Direction::Left if *snake_head_x > 0 => Some((snake_head_x - 1, *snake_head_y)),
            Direction::Up if *snake_head_y > 0 => Some((*snake_head_x, snake_head_y - 1)),
            Direction::Down if *snake_head_y < self.playfield_y - 1 => {
                Some((*snake_head_x, snake_head_y + 1))
            }
            _ => None,
        };

        if let Some(new_head_location) = new_head_location {
            self.snake.push(new_head_location);

            return true;
        }

        return false;
    }

    fn remove_snake_tail(&mut self) {
        self.snake.remove(0);
    }

    pub fn snake_eating_itself(&self) -> bool {
        let snake_body_coords = &self.get_snake()[..self.snake.len() - 1];

        snake_body_coords.iter().any(|body_part_coords| {
            let head_coords = self.snake.last().unwrap();

            *body_part_coords == *head_coords
        })
    }

    pub fn snake_eating_apple(&self) -> bool {
        let snake_head_coords = self.get_snake().last().unwrap();
        let apple_coords_opt = self.get_apple();

        if let Some(apple_coords) = apple_coords_opt {
            return *snake_head_coords == apple_coords;
        }

        false
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn get_apple(&self) -> Apple {
        self.apple
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn set_snake_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Direction;

    use super::Game;

    #[test]
    fn should_initialise_snake() {
        let game = Game::new(5, 5);
        let snake = game.get_snake();

        assert_eq!(snake, &vec![(0, 0)]);
    }

    #[test]
    fn should_move_snake() {
        let mut game = Game::new(5, 5);

        game.next();
        game.next();
        game.next();

        let snake = game.get_snake().last();

        assert_eq!(matches!(game.direction, Direction::Right), true);
        assert_eq!(matches!(snake, Some(&(3, 0))), true);
    }

    #[test]
    fn should_extend_snake_when_apple_eaten() {
        let mut game = Game::new(5, 5);

        game.next();

        game.apple = None;

        game.next();
        game.next();

        let snake = game.get_snake();

        assert_eq!(matches!(game.direction, Direction::Right), true);
        assert_eq!(snake, &vec![(2, 0), (3, 0)]);
        assert_eq!(snake.len(), 2);
    }

    #[test]
    fn should_detect_snake_eating_itsef() {
        let mut game = Game::new(5, 5);

        game.snake = vec![(0, 0), (1, 0), (2, 0), (2, 1), (1, 1), (1, 0)];

        assert_eq!(game.snake_eating_itself(), true);
    }

    #[test]
    fn should_detect_snake_not_eating_itsef() {
        let mut game = Game::new(5, 5);

        game.snake = vec![(0, 0), (1, 0), (2, 0), (2, 1), (1, 1)];

        assert_eq!(game.snake_eating_itself(), false);
    }
}
