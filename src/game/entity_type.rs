use crate::coordinates::Coordinates;

use super::Characters;

pub enum EntityType {
    Apple {
        coordinates: Option<Coordinates>,
        emoji: char,
    },
    SupersnakePwrup {
        coordinates: Option<Coordinates>,
        emoji: char,
    },
    Obstacle {
        coordinates: Option<Coordinates>,
        emoji: char,
    },
    SlowdownPwrup {
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
        Self::SupersnakePwrup {
            coordinates: Some(coordinates),
            emoji: Characters::SupersnakePwrup.value(),
        }
    }

    pub fn new_obstacle(coordinates: Coordinates) -> Self {
        Self::Obstacle {
            coordinates: Some(coordinates),
            emoji: Characters::Obstacle.value(),
        }
    }

    pub fn new_slowdown(coordinates: Coordinates) -> Self {
        Self::SlowdownPwrup {
            coordinates: Some(coordinates),
            emoji: Characters::SlowdownPwrup.value(),
        }
    }

    pub fn get_coordinates(&self) -> Option<&Coordinates> {
        match self {
            Self::Apple { coordinates, .. } => coordinates.as_ref(),
            Self::SupersnakePwrup { coordinates, .. } => coordinates.as_ref(),
            Self::Obstacle { coordinates, .. } => coordinates.as_ref(),
            Self::SlowdownPwrup { coordinates, .. } => coordinates.as_ref(),
        }
    }
}
