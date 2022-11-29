pub mod tiles {
    enum MoveResult {
        ALLOW,
        BLOCK,
    }

    pub trait Tile { // The 'interface' so to speak
        fn check_move_onto(&self, entity: Entity, fromPosition: Position, tilePosition: Position) -> MoveResult;
        fn is_goal(&self) -> bool;
        fn get_glyph(&self) -> String;
    }    

    pub struct Wall {}
    pub struct Floor {}
    pub struct Goal {}

    impl Tile for Wall {
        fn check_move_onto(&self, entity: Entity, fromPosition: Position, tilePosition: Position) -> MoveResult {
            MoveResult::BLOCK
        }

        fn is_goal(&self) -> bool {
            false
        }

        fn get_glyph(&self) -> String {
            std::String('#');
        }
    }

    impl Tile for Floor {
        fn check_move_onto(&self, entity: Entity, fromPosition: Position, tilePosition: Position) -> MoveResult {
            MoveResult::ALLOW
        }

        fn is_goal(&self) -> bool {
            false
        }

        fn get_glyph(&self) -> String {
            std::String('.');
        }
    }

    impl Goal for Floor {
        fn check_move_onto(&self, entity: Entity, fromPosition: Position, tilePosition: Position) -> MoveResult {
            MoveResult::ALLOW
        }

        fn is_goal(&self) -> bool {
            true
        }

        fn get_glyph(&self) -> String {
            std::String('<');
        }
    }

}