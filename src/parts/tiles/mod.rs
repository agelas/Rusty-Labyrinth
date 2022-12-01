pub mod tiles {
    pub enum MoveResult {
        ALLOW,
        BLOCK,
    }

    trait Tile { // The 'interface' so to speak
        fn check_move_onto(&self) -> MoveResult;
        fn is_goal(&self) -> bool;
        fn get_glyph(&self) -> String;
    }    

    pub struct Wall {}
    pub struct Floor {}
    pub struct Goal {}

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

}