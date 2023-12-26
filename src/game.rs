pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Snake = Vec<(usize, usize)>;

pub struct Game {
    snake: Snake, // A list of the snakes parts using x/y coords
    eaten_apple: bool,
    direction: Direction,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: vec![(1, 0), (2, 0), (3, 0)],
            eaten_apple: false,
            direction: Direction::Right,
        }
    }

    pub fn next(&mut self) {
        self.add_snake_head();

        if !self.eaten_apple {
            self.remove_snake_tail()
        }

        self.eaten_apple = false;
    }

    fn add_snake_head(&mut self) {
        let (snake_head_x, snake_head_y) = self.snake.last().unwrap();

        match self.direction {
            Direction::Right => self.snake.push((snake_head_x + 1, *snake_head_y)),
            Direction::Left => self.snake.push((snake_head_x - 1, *snake_head_y)),
            Direction::Up => self.snake.push((*snake_head_x, snake_head_y - 1)),
            Direction::Down => self.snake.push((*snake_head_x, snake_head_y + 1)),
        };
    }

    fn remove_snake_tail(&mut self) {
        self.snake.remove(0);
    }

    pub fn eat_apple(&mut self) {
        self.eaten_apple = true
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Direction;

    use super::Game;

    #[test]
    fn should_initialise_snake() {
        let game = Game::new();
        let snake = game.get_snake();

        assert_eq!(snake, &vec![(0, 0)]);
    }

    #[test]
    fn should_move_snake() {
        let mut game = Game::new();

        game.next();
        game.next();
        game.next();

        let snake = game.get_snake();

        assert_eq!(matches!(game.direction, Direction::Right), true);
        assert_eq!(snake, &vec![(3, 0)]);
        assert_eq!(snake.len(), 1);
    }

    #[test]
    fn should_extend_snake_when_apple_eaten() {
        let mut game = Game::new();

        game.next();

        game.eat_apple();

        game.next();
        game.next();

        let snake = game.get_snake();

        assert_eq!(matches!(game.direction, Direction::Right), true);
        assert_eq!(snake, &vec![(2, 0), (3, 0)]);
        assert_eq!(snake.len(), 2);
    }
}
