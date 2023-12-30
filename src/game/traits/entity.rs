use crate::{coordinate::Coordinate, game::entity_type::EntityType};

pub trait Entity {
    fn get_all_entities(&self) -> Vec<&EntityType>;

    /// Returns a vector of coordinates that entities cannot spawn on, for example, the snake
    fn get_entity_no_go_zones(&self) -> &Vec<Coordinate>;

    fn remove_entity(&mut self);

    fn add_entity<F>(&mut self, new_coords: F, max_x: usize, max_y: usize)
    where
        F: FnOnce(Coordinate),
    {
        loop {
            let new_powerup_location = Coordinate::new_random(max_x, max_y);

            if !new_powerup_location.intersects_multiple(self.get_entity_no_go_zones()) {
                new_coords(new_powerup_location);

                break;
            }
        }
    }
}
