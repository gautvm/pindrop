use nalgebra::{Quaternion, Translation3, UnitQuaternion};
use std::collections::HashMap;

use crate::Pose3;
pub enum Game {
    ChargedUp,
}

pub struct Field {
    april_tags: HashMap<i32, Pose3<f64>>,
    length: f64,
    width: f64,
}

impl Field {
    pub fn new(game: Game) -> Self {
        let mut april_tags = HashMap::new();
        match game {
            Game::ChargedUp => {
                april_tags.insert(
                    1,
                    Pose3::from_parts(
                        Translation3::new(15.513558, 1.071626, 0.462788),
                        UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 0.0, 1.0)),
                    ),
                );
                april_tags.insert(
                    2,
                    Pose3::from_parts(
                        Translation3::new(15.513558, 2.748026, 0.462788),
                        UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 0.0, 1.0)),
                    ),
                );
                april_tags.insert(
                    3,
                    Pose3::from_parts(
                        Translation3::new(15.513558, 4.424426, 0.462788),
                        UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 0.0, 1.0)),
                    ),
                );
                april_tags.insert(
                    4,
                    Pose3::from_parts(
                        Translation3::new(16.178784, 6.749796, 0.695452),
                        UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 0.0, 1.0)),
                    ),
                );
                april_tags.insert(
                    5,
                    Pose3::from_parts(
                        Translation3::new(0.36195, 6.749796, 0.695452),
                        UnitQuaternion::from_quaternion(Quaternion::new(1.0, 0.0, 0.0, 0.0)),
                    ),
                );
                april_tags.insert(
                    6,
                    Pose3::from_parts(
                        Translation3::new(1.02743, 4.424426, 0.462788),
                        UnitQuaternion::from_quaternion(Quaternion::new(1.0, 0.0, 0.0, 0.0)),
                    ),
                );
                april_tags.insert(
                    7,
                    Pose3::from_parts(
                        Translation3::new(1.02743, 2.748026, 0.462788),
                        UnitQuaternion::from_quaternion(Quaternion::new(1.0, 0.0, 0.0, 0.0)),
                    ),
                );
                april_tags.insert(
                    8,
                    Pose3::from_parts(
                        Translation3::new(1.02743, 1.071626, 0.462788),
                        UnitQuaternion::from_quaternion(Quaternion::new(1.0, 0.0, 0.0, 0.0)),
                    ),
                );
            }
        }

        let (length, width) = match game {
            Game::ChargedUp => (16.54175, 8.0137),
        };

        Self {
            april_tags,
            length,
            width,
        }
    }

    pub fn get_april_tags(&self) -> &HashMap<i32, Pose3<f64>> {
        &self.april_tags
    }

    pub fn get_april_tag_by_id(&self, id: i32) -> Option<&Pose3<f64>> {
        self.april_tags.get(&id)
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }
}
