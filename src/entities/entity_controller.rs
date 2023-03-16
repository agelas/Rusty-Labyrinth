use crate::parts::position::Direction;

pub trait EntityController {
    fn get_move_direction() -> Direction;
    fn is_user() -> bool;
}