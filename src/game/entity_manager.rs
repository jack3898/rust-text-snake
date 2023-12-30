use std::collections::HashMap;

pub struct EntityManager<'a, E> {
    pub entities: HashMap<&'a str, E>,
}

impl<'a, E> EntityManager<'a, E> {
    pub fn new() -> Self {
        let entities = HashMap::new();

        Self { entities }
    }

    pub fn get_entity(&self, entity_type: &str) -> Option<&E> {
        self.entities.get(entity_type)
    }

    pub fn add_entity(&mut self, name: &'a str, entity_type: E) {
        self.entities.insert(name, entity_type);
    }

    pub fn remove_entity(&mut self, name: &str) {
        self.entities.remove(name);
    }
}
