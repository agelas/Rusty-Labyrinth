pub mod tiles {
    enum MoveResult {
        ALLOW,
        BLOCK,
    }

    pub trait Tile { // let me just say I'm so confused if this is how "interfaces" work
        fn check_move_onto(&self, entity: Entity, fromPosition: Position, tilePosition: Position) -> MoveResult;
        fn is_goal(&self) -> bool;
        fn get_glyph(&self) -> String;
    }    

}