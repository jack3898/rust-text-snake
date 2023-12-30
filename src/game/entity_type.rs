use crate::{characters::Characters, coordinate::Coordinate};

pub enum EntityType {
    Apple {
        coordinates: Option<Coordinate>,
        emoji: char,
    },
    Supersnake {
        coordinates: Option<Coordinate>,
        emoji: char,
    },
}

impl EntityType {
    pub fn new_apple(coordinates: Coordinate) -> Self {
        Self::Apple {
            coordinates: Some(coordinates),
            emoji: Characters::Apple.value(),
        }
    }

    pub fn new_supersnake(coordinates: Coordinate) -> Self {
        Self::Supersnake {
            coordinates: Some(coordinates),
            emoji: Characters::Supersnake.value(),
        }
    }

    pub fn get_coordinates(&self) -> Option<&Coordinate> {
        match self {
            Self::Apple { coordinates, .. } => coordinates.as_ref(),
            Self::Supersnake { coordinates, .. } => coordinates.as_ref(),
        }
    }
}
