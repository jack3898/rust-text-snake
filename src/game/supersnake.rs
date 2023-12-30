use crate::coordinate::Coordinate;

pub trait Supersnake {
    fn get_supersnake(&self) -> Option<&Coordinate>;

    fn get_supersnake_no_go_zones(&self) -> &Vec<Coordinate>;

    fn set_supersnake(&mut self, apple: Coordinate);

    fn remove_supersnake(&mut self);

    fn add_supersnake(&mut self, max_x: usize, max_y: usize) {
        let supersnake = self.get_supersnake();

        if supersnake.is_some() {
            return;
        }

        loop {
            let new_supersnake = Coordinate::new_random(max_x, max_y);

            if !new_supersnake.intersects_multiple(self.get_supersnake_no_go_zones()) {
                self.set_supersnake(new_supersnake);

                break;
            }
        }
    }
}
