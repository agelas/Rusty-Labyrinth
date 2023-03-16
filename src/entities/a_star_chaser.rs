use crate::parts::position::Direction;
use super::entity_controller::EntityController;

pub struct AStarChaser {}

impl AStarChaser {
    pub fn new() -> AStarChaser {
        AStarChaser {}
    }
    fn path_extension(){} // This will eventually take in a bunch of stuff
    fn check_move() -> bool{
        true
    }
}

impl EntityController for AStarChaser {
    fn get_move_direction() -> Direction {
        Direction::RIGHT
    }

    fn is_user() -> bool {
        false
    }
}