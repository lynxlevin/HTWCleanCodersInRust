pub mod direction {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }

    impl Direction {
        pub fn opposite(&self) -> Direction {
            match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            }
        }

        pub fn name(&self) -> &str {
            match self {
                Direction::North => "North",
                Direction::South => "South",
                Direction::East => "East",
                Direction::West => "West",
            }
        }
    }
    #[cfg(test)]
    mod tests_for_direction {
        use super::*;

        #[test]
        fn name_of_north() {
            let north = Direction::North;
            assert_eq!("North", north.name());
        }

        #[test]
        fn name_of_south() {
            let south = Direction::South;
            assert_eq!("South", south.name());
        }

        #[test]
        fn name_of_east() {
            let east = Direction::East;
            assert_eq!("East", east.name());
        }

        #[test]
        fn name_of_west() {
            let west = Direction::West;
            assert_eq!("West", west.name());
        }

        #[test]
        fn opposite_of_north() {
            let north = Direction::North;
            assert_eq!(Direction::South, north.opposite());
        }

        #[test]
        fn opposite_of_south() {
            let south = Direction::South;
            assert_eq!(Direction::North, south.opposite());
        }

        #[test]
        fn opposite_of_east() {
            let east = Direction::East;
            assert_eq!(Direction::West, east.opposite());
        }

        #[test]
        fn opposite_of_west() {
            let west = Direction::West;
            assert_eq!(Direction::East, west.opposite());
        }
    }
}
