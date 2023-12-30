use rand::Rng;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl Coordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn new_random(max_x: usize, max_y: usize) -> Self {
        Self {
            x: rand::thread_rng().gen_range(0..max_x),
            y: rand::thread_rng().gen_range(0..max_y),
        }
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Returns true if the coordinate intersects with the other coordinate
    pub fn intersects(&self, other: &Coordinates) -> bool {
        self.x == other.x && self.y == other.y
    }

    /// Returns true if the coordinate intersects with any of the coordinates in the vector
    pub fn intersects_multiple(&self, others: &[Coordinates]) -> bool {
        for other in others {
            if self.intersects(other) {
                return true;
            }
        }

        false
    }
}
