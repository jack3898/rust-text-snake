use rand::Rng;

use crate::coordinate::Coordinate;

pub trait Supersnake {
    fn get_supersnake(&self) -> Option<&Coordinate>;

    fn get_supersnake_no_go_zones(&self) -> &Vec<Coordinate>;

    fn set_supersnake(&mut self, apple: Coordinate);

    fn remove_supersnake(&mut self);

    fn add_supersnake(&mut self, max_x: usize, max_y: usize) {
        let apple = self.get_supersnake();

        if apple.is_some() {
            return;
        }

        loop {
            let new_apple = Coordinate::new(
                rand::thread_rng().gen_range(0..max_x),
                rand::thread_rng().gen_range(0..max_y),
            );

            if !self.get_supersnake_no_go_zones().contains(&new_apple) {
                self.set_supersnake(new_apple);

                break;
            }
        }
    }
}
