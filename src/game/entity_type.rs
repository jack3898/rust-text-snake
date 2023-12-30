use crate::{characters::Characters, coordinates::Coordinates};

pub enum EntityType {
    Apple {
        coordinates: Option<Coordinates>,
        emoji: char,
    },
    Supersnake {
        coordinates: Option<Coordinates>,
        emoji: char,
    },
}

impl EntityType {
    pub fn new_apple(coordinates: Coordinates) -> Self {
        Self::Apple {
            coordinates: Some(coordinates),
            emoji: Characters::Apple.value(),
        }
    }

    pub fn new_supersnake(coordinates: Coordinates) -> Self {
        Self::Supersnake {
            coordinates: Some(coordinates),
            emoji: Characters::Supersnake.value(),
        }
    }

    pub fn get_coordinates(&self) -> Option<&Coordinates> {
        match self {
            Self::Apple { coordinates, .. } => coordinates.as_ref(),
            Self::Supersnake { coordinates, .. } => coordinates.as_ref(),
        }
    }
}
