use crate::coordinate::Coordinate;

pub trait Apple {
    fn get_apple(&self) -> Option<&Coordinate>;

    fn get_apple_no_go_zones(&self) -> &Vec<Coordinate>;

    fn set_apple(&mut self, apple: Coordinate);

    fn remove_apple(&mut self);

    fn add_apple(&mut self, max_x: usize, max_y: usize) {
        let apple = self.get_apple();

        if apple.is_some() {
            return;
        }

        loop {
            let new_apple = Coordinate::new_random(max_x, max_y);

            if !new_apple.intersects_multiple(self.get_apple_no_go_zones()) {
                self.set_apple(new_apple);

                break;
            }
        }
    }
}
