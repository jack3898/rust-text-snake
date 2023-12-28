use rand::Rng;

use crate::coordinate::Coordinate;

pub trait Apple {
    fn get_apple(&self) -> Option<Coordinate>;

    fn get_apple_no_go_zones(&self) -> &Vec<Coordinate>;

    fn set_apple(&mut self, apple: Option<Coordinate>);

    fn add_apple(&mut self, max_x: usize, max_y: usize) {
        let apple = self.get_apple();

        if apple.is_some() {
            return;
        }

        loop {
            let new_apple = Coordinate::new(
                rand::thread_rng().gen_range(0..max_x),
                rand::thread_rng().gen_range(0..max_y),
            );

            if !self.get_apple_no_go_zones().contains(&new_apple) {
                self.set_apple(Some(new_apple));

                break;
            }
        }
    }
}
