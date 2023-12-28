use rand::Rng;

use crate::coordinate::Coordinate;

pub trait Apple {
    fn get_apple(&self) -> Option<Coordinate>;

    fn set_apple(&mut self, apple: Option<Coordinate>);

    fn add_apple(&mut self, max_x: usize, max_y: usize) {
        let apple = self.get_apple();

        if apple.is_none() {
            self.set_apple(Some(Coordinate::new(
                rand::thread_rng().gen_range(0..max_x),
                rand::thread_rng().gen_range(0..max_y),
            )));
        }
    }
}
