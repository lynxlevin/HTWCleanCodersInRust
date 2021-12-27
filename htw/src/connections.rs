pub mod connections {
    use crate::connection::connection::Connection;
    use crate::htw_game::htw_game::Caverns;
    use crate::Direction;
    use rand::Rng;

    #[derive(Debug, PartialEq)]
    pub struct Connections {
        pub connections: Vec<Connection>,
    }

    impl Connections {
        pub fn new(connections: Vec<Connection>) -> Connections {
            Connections { connections }
        }

        // TODO: change cavern to String
        pub fn report_nearby(&self, cavern: &String, target_caverns: &Caverns) -> bool {
            let mut result = false;
            for c in &self.connections {
                result = result
                    || (c.from() == cavern && target_caverns.contains(&String::from(c.to())));
            }
            result
        }

        // TODO: change cavern to String
        pub fn find_destination(&self, cavern: &String, direction: &Direction) -> Option<String> {
            for c in &self.connections {
                if c.from() == cavern && c.direction() == direction {
                    return Some(c.to().to_string());
                }
            }
            None
        }

        pub fn report_available_directions(&self, cavern: &String) -> Vec<Direction> {
            let mut result = Vec::new();
            for c in &self.connections {
                if c.from() == cavern {
                    result.push(*c.direction());
                }
            }
            result
        }

        fn is_connectable_cavern(&self, this: &str, other: &str, direction: &Direction) -> bool {
            let mut result = true;
            for c in &self.connections {
                if c.from() == this {
                    result = result && c.to() != other && c.direction() != direction;
                }
            }
            result
        }

        // FIXME: cannot break into several functions due to error below
        // "cannot borrow `*self` as mutable because it is also borrowed as immutable"
        pub fn connect_caverns(&mut self, caverns: &Caverns) {
            let directions = vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ];
            for cavern in caverns {
                for direction in &directions {
                    if rand::thread_rng().gen_range(0..10) > 2 {
                        let other = any_other(cavern, caverns);
                        if self.is_connectable_cavern(cavern, &other, direction) {
                            self.connections
                                .push(Connection::new(cavern, &other, direction));
                        }
                        if self.is_connectable_cavern(&other, cavern, &direction.opposite()) {
                            self.connections.push(Connection::new(
                                &other,
                                cavern,
                                &direction.opposite(),
                            ));
                        }
                    }
                }
            }
        }
    }

    pub fn any_cavern(caverns: &Caverns) -> String {
        let vector = Vec::from_iter(caverns);
        let n = vector.len();
        let choice = rand::thread_rng().gen_range(0..n);
        vector[choice].to_string()
    }

    pub fn any_other(cavern: &str, caverns: &Caverns) -> String {
        let mut other = String::from(cavern);
        while other == String::from(cavern) {
            other = any_cavern(caverns);
        }
        other
    }

    #[cfg(test)]
    mod tests_for_connections {
        use super::*;
        use std::collections::HashSet;

        fn type_of<T>(_: &T) -> &str {
            std::any::type_name::<T>()
        }

        fn set_up() -> Connections {
            let connections = vec![
                Connection::new("cavern", "cavern_w", &Direction::West),
                Connection::new("cavern_w", "cavern", &Direction::East),
                Connection::new("cavern", "cavern_e", &Direction::East),
                Connection::new("cavern_e", "cavern", &Direction::West),
                Connection::new("cavern", "cavern_n", &Direction::North),
                Connection::new("cavern_n", "cavern", &Direction::South),
                Connection::new("cavern", "cavern_s", &Direction::South),
                Connection::new("cavern_s", "cavern", &Direction::North),
                Connection::new("cavern_n", "cavern_nn", &Direction::North),
                Connection::new("cavern_nn", "cavern_n", &Direction::South),
            ];
            Connections::new(connections)
        }

        #[test]
        fn test_report_nearby() {
            let connections = set_up();
            let cavern = String::from("cavern");
            let target_caverns = HashSet::from([String::from("cavern_e")]);
            assert!(connections.report_nearby(&cavern, &target_caverns));

            let target_caverns = HashSet::from([String::from("cavern_nn")]);
            assert!(!connections.report_nearby(&cavern, &target_caverns));
        }

        #[test]
        fn test_find_destination() {
            let connections = set_up();
            let cavern = String::from("cavern");
            let result = connections.find_destination(&cavern, &Direction::South);
            assert_eq!(result, Some(String::from("cavern_s")));

            let cavern = String::from("cavern_s");
            let result = connections.find_destination(&cavern, &Direction::South);
            assert_eq!(result, None);
        }

        #[test]
        fn test_report_available_directions() {
            let connections = set_up();
            let cavern = String::from("cavern_n");
            let result = connections.report_available_directions(&cavern);
            // TODO: change this so order does not matter.
            assert_eq!(Vec::from([Direction::South, Direction::North]), result);
        }

        #[test]
        fn test_any_cavern() {
            let caverns = HashSet::from([
                String::from("cavern"),
                String::from("cavern_w"),
                String::from("cavern_e"),
                String::from("cavern_n"),
                String::from("cavern_s"),
                String::from("cavern_nn"),
            ]);
            let result = any_cavern(&caverns);
            assert_eq!("alloc::string::String", type_of(&result));
            assert!(caverns.contains(&result));
        }

        #[test]
        fn test_any_other() {
            let caverns = HashSet::from([
                String::from("cavern"),
                String::from("cavern_w"),
                String::from("cavern_e"),
                String::from("cavern_n"),
                String::from("cavern_s"),
                String::from("cavern_nn"),
            ]);
            assert_ne!("cavern", any_other("cavern", &caverns));
        }

        #[test]
        fn test_connections_equality() {
            let connections1 = set_up();
            let connections2 = set_up();
            assert_eq!(connections1, connections2);
        }

        #[test]
        fn test_is_connectable_cavern() {
            let connections = Connections::new(vec![Connection::new(
                "cavern",
                "cavern_n",
                &Direction::North,
            )]);
            let this = "cavern";
            let north = Direction::North;
            let west = Direction::West;
            // can connect to cavern_w but not to cavern_n
            assert!(connections.is_connectable_cavern(this, "cavern_w", &west));
            assert!(!connections.is_connectable_cavern(this, "cavern_n", &west));

            // cannot connect to North anymore
            assert!(!connections.is_connectable_cavern(this, "cavern_n", &north));
            assert!(!connections.is_connectable_cavern(this, "cavern_w", &north));
        }

        #[test]
        fn test_connect_caverns() {
            let mut connections = Connections::new(vec![]);
            let caverns = HashSet::from([
                String::from("cavern"),
                String::from("cavern_w"),
                String::from("cavern_e"),
                String::from("cavern_n"),
                String::from("cavern_s"),
                String::from("cavern_nn"),
            ]);
            connections.connect_caverns(&caverns);
            assert_ne!(0, connections.connections.len());
            // assert_eq!(Connections::new(vec![]), connections);
            // TODO: assert that unavailable_other and unavailable_direction are less than 2 in all connections
        }
    }
}
