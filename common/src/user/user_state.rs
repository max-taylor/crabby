use serde::{Deserialize, Serialize};

use crate::types::position::Position;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ControlEvent {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Space,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserState {
    pub position: Position,
    pub control_event: Option<ControlEvent>,
    pub rotation_deg: f64,
}

pub const MAX_MOVEMENT_SPEED: i64 = 100;
pub const MAX_ROTATION_SPEED: f64 = 30.0;

impl UserState {
    pub fn new() -> Self {
        UserState {
            position: Position { x: 0, y: 0 },
            control_event: None,
            rotation_deg: 0.0,
        }
    }

    pub fn move_in_direction(&mut self, distance: i64) {
        // Convert direction to radians
        let radians = (self.rotation_deg as f64).to_radians();

        // Calculate x and y components with rounding
        let dx = (distance as f64 * radians.cos()).round() as i64;
        let dy = (distance as f64 * radians.sin()).round() as i64;

        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn rotate(&mut self, angle: f64) {
        // Update the rotation angle
        self.rotation_deg += angle;

        // Normalize the angle to be within 0-360 degrees
        if self.rotation_deg < 0.0 {
            self.rotation_deg += 360.0;
        } else if self.rotation_deg >= 360.0 {
            self.rotation_deg -= 360.0;
        }
    }

    pub fn update(&mut self, time_diff_ms: u64) {
        // Calculate distance to move based on time difference
        // MAX_SPEED is in units/sec, time_diff is in ms
        let distance = MAX_MOVEMENT_SPEED * time_diff_ms as i64 / 1000;

        if let Some(event) = &self.control_event {
            match event {
                ControlEvent::ArrowUp => self.move_in_direction(distance),
                ControlEvent::ArrowDown => self.move_in_direction(-distance),
                ControlEvent::ArrowLeft => {
                    self.rotate(-MAX_ROTATION_SPEED * time_diff_ms as f64 / 1000.0)
                }
                ControlEvent::ArrowRight => {
                    self.rotate(MAX_ROTATION_SPEED * time_diff_ms as f64 / 1000.0)
                }
                ControlEvent::Space => {}
            }
        }
    }
}
