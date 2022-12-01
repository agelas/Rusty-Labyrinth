#[derive(PartialEq, Debug)]
pub enum MoveResult {
    ALLOW,
    BLOCK,
}

pub trait Tile { // The 'interface' so to speak
    fn check_move_onto(&self) -> MoveResult;
    fn is_goal(&self) -> bool;
    fn get_glyph(&self) -> String;
}    

pub struct Wall {}
pub struct Floor {}
pub struct Goal {}

impl Wall {
    pub fn new() -> Wall {
        Wall {}
    }
}

impl Tile for Wall {
    fn check_move_onto(&self) -> MoveResult {
        MoveResult::BLOCK
    }

    fn is_goal(&self) -> bool {
        false
    }

    fn get_glyph(&self) -> String {
        String::from("#")
    }
}

impl Floor {
    pub fn new() -> Floor {
        Floor {}
    }
}

impl Tile for Floor {
    fn check_move_onto(&self) -> MoveResult {
        MoveResult::ALLOW
    }

    fn is_goal(&self) -> bool {
        false
    }

    fn get_glyph(&self) -> String {
        String::from(".")
    }
}

impl Goal {
    pub fn new() -> Goal {
        Goal {}
    }
}
impl Tile for Goal {
    fn check_move_onto(&self) -> MoveResult {
        MoveResult::ALLOW
    }

    fn is_goal(&self) -> bool {
        true
    }

    fn get_glyph(&self) -> String {
        String::from("<")
    }
}


#[cfg(test)]
mod tests {
    use crate::parts::tiles::{Floor, Wall, Goal, MoveResult};

    use super::Tile;

    #[test]
    fn test_tile_types() {
        let test_floor: Floor = Floor::new();
        let test_wall: Wall = Wall::new();
        let test_goal: Goal = Goal::new();

        assert_eq!(test_floor.get_glyph(), ".");
        assert_eq!(test_wall.get_glyph(), "#");
        assert_eq!(test_goal.get_glyph(), "<");
    }

    #[test]
    fn test_move() {
        let test_floor: Floor = Floor::new();
        let test_wall: Wall = Wall::new();
        let test_goal: Goal = Goal::new();

        assert_eq!(test_floor.check_move_onto(), MoveResult::ALLOW);
        assert_eq!(test_wall.check_move_onto(), MoveResult::BLOCK);
        assert_eq!(test_goal.check_move_onto(), MoveResult::ALLOW);
    }
}
