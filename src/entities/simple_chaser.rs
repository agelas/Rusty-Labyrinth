use crate::parts::position::Direction;
use super::entity_controller::EntityController;

pub struct SimpleChaser {}

impl SimpleChaser {
    pub fn new() -> SimpleChaser {
        SimpleChaser {}
    }
}

impl EntityController for SimpleChaser {
    fn get_move_direction() -> Direction {
        Direction::LEFT
    }

    fn is_user() -> bool {
        false
    }
}