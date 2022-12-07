pub mod entities{
    use crate::parts::position::Position;
    use crate::parts::tiles;

    /*
        An Entity is meant to represent a generic game
        entity like the hero or minotaur.
    */
    pub struct Entity {
        glyph: String,
        properties: String,
        position: Position,
    }

    pub trait EntityController {
        fn get_move_direction() -> MoveResult;
        fn is_user() -> bool;

    }

    impl Entity {

        pub fn new() -> Entity {
            Entity{glyph: String::from(""), properties: String::from(""), position: Position::new(0, 0)}
        }

        pub fn set_glyph(& mut self, glyph: String) {
            self.glyph = glyph;
        }

        pub fn get_glyph(&self) -> String {
            self.glyph.clone() // Idk why 
        }

        pub fn set_properties(& mut self, props: String) {
            self.properties = props;
        }

        pub fn get_properties(&self) -> String {
            self.properties.clone()
        }

        pub fn has_property(&self, prop: String) -> bool {
            self.properties.contains(&prop)
        }

        pub fn set_position(& mut self, pos: Position) {
            self.position = pos;
        }

        pub fn get_position(&self) -> &Position {
            &self.position // I guess we "borrow" the Entity's Position struct??
        }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::parts::position::Position;
    use super::entities::Entity;

    #[test]
    fn test_creation() {
        let mut test_e: Entity = Entity::new();
        test_e.set_glyph(String::from('m'));
        test_e.set_properties(String::from('s'));
        assert_eq!(test_e.get_glyph(), 'm'.to_string());
        assert_eq!(test_e.get_properties(), 's'.to_string());
    }

    #[test]
    fn test_properties() {
        let mut test_e: Entity = Entity::new();
        test_e.set_properties("smh".to_string());
        assert_eq!(test_e.has_property("m".to_string()), true);
    }

    #[test]
    fn test_position() {
        let p: Position = Position::new(1, 1);
        let p_test: Position = Position::new(1, 1);
        let mut test_e: Entity = Entity::new();
        test_e.set_position(p);
        assert_eq!(test_e.get_position(), &p_test);
    }
}