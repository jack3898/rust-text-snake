use std::collections::HashMap;

use crate::coordinate::Coordinate;

pub struct EntityManager<E> {
    pub entities: HashMap<Coordinate, E>,
}

impl<'a, E> EntityManager<E> {
    pub fn new() -> Self {
        let entities = HashMap::new();

        Self { entities }
    }

    pub fn get_entity(&self, coordinate: Coordinate) -> Option<&E> {
        self.entities.get(&coordinate)
    }

    pub fn add_entity(&mut self, coordinates: &Coordinate, entity_type: E) {
        self.entities.insert(coordinates.clone(), entity_type);
    }

    pub fn remove_entity(&mut self, coordinates: &Coordinate) {
        self.entities.remove(coordinates);
    }
}
