use serde::{Deserialize, Serialize};

use crate::types::position::Position;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserState {
    pub position: Position,
    pub direction_deg: u64,
}

pub const MAX_SPEED: i64 = 25;

impl UserState {
    pub fn new() -> Self {
        UserState {
            position: Position { x: 0, y: 0 },
            direction_deg: 0,
        }
    }

    pub fn update(&mut self, time_diff_ms: u64) {
        // Calculate distance to move based on time difference
        // MAX_SPEED is in units/sec, time_diff is in ms
        let distance = MAX_SPEED * time_diff_ms as i64 / 1000;

        // Convert direction to radians
        let radians = (self.direction_deg as f64).to_radians();

        // Calculate x and y components with rounding
        let dx = (distance as f64 * radians.cos()).round() as i64;
        let dy = (distance as f64 * radians.sin()).round() as i64;

        self.position.x += dx;
        self.position.y += dy;
    }
}
//
// impl Display for UserState {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "UserState(x: {}, y: {}, direction_deg: {})",
//             self.x, self.y, self.direction_deg
//         )
//     }
// }
