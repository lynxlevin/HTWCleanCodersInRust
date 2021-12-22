pub mod connection {
    use crate::direction::direction::Direction;

    pub struct Connection {
        from: String,
        to: String,
        direction: Direction,
    }

    impl Connection {
        pub fn new(from: &str, to: &str, &direction: &Direction) -> Connection {
            Connection {
                from: String::from(from),
                to: String::from(to),
                direction,
            }
        }

        pub fn from(&self) -> &str {
            &self.from
        }

        pub fn to(&self) -> &str {
            &self.to
        }

        pub fn direction(&self) -> &Direction {
            &self.direction
        }
    }
}
