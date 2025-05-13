use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn angle_to(&self, other: &Position) -> f64 {
        // Calculate the vector from self to other
        let dx = other.x as f64 - self.x as f64;
        let dy = other.y as f64 - self.y as f64;

        // Calculate the angle in radians using atan2
        let radians = dy.atan2(dx);

        // Convert to degrees and normalize to 0-360 range
        let degrees = radians.to_degrees();

        // Normalize to 0-360 range
        if degrees < 0.0 {
            degrees + 360.0
        } else {
            degrees
        }
    }
}
