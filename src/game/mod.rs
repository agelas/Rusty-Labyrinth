pub mod ui;

pub mod game {
    use crate::{parts::{maze::Maze, position::Position}, entities::entity::Entity};


    pub struct Game {
        maze: Maze,
        entity_vector: Vec<Box<Entity>>,
        // UI
        // GameRules
    }

    impl Game {
        pub fn new(maze: Maze) -> Game {
            Game{maze: maze, entity_vector: Vec::new()}
        }

        pub fn set_maze(&mut self, maze: Maze) {
            self.maze = maze;
        }

        pub fn get_maze(&self) -> &Maze {
            &self.maze
        }

        pub fn add_entity(& mut self, entity: Entity) {
            let boxed_entity = Box::new(entity);
            self.entity_vector.push(boxed_entity);
        }

        pub fn get_entity_at(&self, position: Position) -> Option<&Entity> {
            // @return Option<Entity> because there might not be an entity that matches, so Option<> allows for None
            for entity in self.entity_vector.iter() {
                if entity.has_property('m'.to_string()) && entity.get_position() == &position {
                    return Some(entity); // Some() is an enum variant that needs to be used w/Option
                }
            }
            None
        }

        pub fn get_entities_with_property(&self, property: String) -> Vec<&Entity> {
            let mut matching_entities = vec![];

            for entity in self.entity_vector.iter() {
                if entity.has_property(property.clone()) {
                    matching_entities.push(entity.as_ref());
                }
            }
            matching_entities
        }

        pub fn get_entities(&self) -> &Vec<Box<Entity>> {
            &self.entity_vector
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{parts::position::Position, entities::entity::Entity, parts::maze::Maze, game::game::Game};

    macro_rules! setup_game {
        ($file_path:expr) => {{
            let t_maze: Maze = Maze::read(&$file_path.to_string());
            let t_game = Game::new(t_maze);
            t_game
        }};
    }

    #[test]
    fn test_game_creation() {
        let file_path = "maze1.txt".to_string();
        let t_maze: Maze = Maze::read(&file_path);
        let t_game = Game::new(t_maze);
    }

    #[test]
    fn test_add_and_get_entity() {
        let mut t_game: Game = setup_game!("maze1.txt");
        let p: Position = Position::new(3, 3);
        let mut test_e: Entity = Entity::new();
        test_e.set_position(p);
        test_e.set_properties('m'.to_string());
        t_game.add_entity(test_e);
        let t_entity = t_game.get_entity_at(Position::new(3, 3));
        assert_eq!(*t_entity.unwrap().get_position(), Position::new(3, 3))
    }

    #[test]
    fn test_get_entities() {
        let mut t_game: Game = setup_game!("maze1.txt");
        let p1: Position = Position::new(3, 3);
        let p2: Position = Position::new(4, 4);
        let p3: Position = Position::new(5, 5);
        let mut test_e_1: Entity = Entity::new();
        let mut test_e_2: Entity = Entity::new();
        let mut test_e_3: Entity = Entity::new();
        test_e_1.set_position(p1);
        test_e_2.set_position(p2);
        test_e_3.set_position(p3);
        test_e_1.set_properties("mh".to_string());
        test_e_2.set_properties('m'.to_string());
        test_e_3.set_properties("hs".to_string());
        t_game.add_entity(test_e_1);
        t_game.add_entity(test_e_2);
        t_game.add_entity(test_e_3);

        let t_vec: Vec<&Entity> = t_game.get_entities_with_property('m'.to_string());
        assert_eq!(t_vec.len(), 2);
    }
}