use crate::{coordinates::Coordinates, game::entity_type::EntityType};

pub trait Entity {
    fn get_all_entities(&self) -> Vec<&EntityType>;

    /// Returns a vector of coordinates that entities cannot spawn on, for example, the snake
    fn get_entity_no_go_zones(&self) -> Vec<Coordinates>;

    fn remove_entity(&mut self);

    fn add_entity<F>(&mut self, new_coords: F, max_x: usize, max_y: usize)
    where
        F: FnOnce(Coordinates),
    {
        loop {
            let new_entity_location = Coordinates::new_random(max_x, max_y);

            if !new_entity_location.intersects_multiple(&self.get_entity_no_go_zones()) {
                new_coords(new_entity_location);

                break;
            }
        }
    }
}
