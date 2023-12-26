use rand::Rng;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Snake = Vec<(usize, usize)>;
type Apple = Option<(usize, usize)>;

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
            snake: vec![(1, 0), (2, 0), (3, 0)],
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
        self.add_snake_head();

        if self.apple.is_some() {
            self.remove_snake_tail()
        }

        let apple_coords = self.get_apple();

        if let Some(apple_coords) = apple_coords {
            if apple_coords.0 == self.get_snake().last().unwrap().0
                && apple_coords.1 == self.get_snake().last().unwrap().1
            {
                self.apple = None;
                self.score += 1;
            }
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

    fn add_snake_head(&mut self) {
        let (snake_head_x, snake_head_y) = self.snake.last().unwrap();

        let new_head_location = match self.direction {
            Direction::Right => (snake_head_x + 1, *snake_head_y),
            Direction::Left => (snake_head_x - 1, *snake_head_y),
            Direction::Up => (*snake_head_x, snake_head_y - 1),
            Direction::Down => (*snake_head_x, snake_head_y + 1),
        };

        self.snake.push(new_head_location);
    }

    fn remove_snake_tail(&mut self) {
        self.snake.remove(0);
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn get_apple(&self) -> &Apple {
        &self.apple
    }

    pub fn get_score(&self) -> usize {
        self.score
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

        let snake = game.get_snake();

        assert_eq!(matches!(game.direction, Direction::Right), true);
        assert_eq!(snake, &vec![(3, 0)]);
        assert_eq!(snake.len(), 1);
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
}
