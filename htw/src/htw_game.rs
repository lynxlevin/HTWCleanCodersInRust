pub mod htw_game {
    use crate::commands::commands::{MoveCommand, RestCommand, ShootCommand};
    use crate::connections::connections::Connections;
    use crate::Direction;
    use crate::HtwMessageReceiver;
    use crate::HuntTheWumpus;
    use rand::Rng;
    use std::collections::{HashMap, HashSet};

    pub type Caverns = HashSet<String>;
    pub type BatCaverns = HashSet<String>;
    pub type PitCaverns = HashSet<String>;
    pub type ArrowsIn = HashMap<String, u32>;

    pub trait Command {
        fn process_command(
            &self,
            message_receiver: &Box<dyn HtwMessageReceiver>,
            connections: &Connections,
            caverns: &Caverns,
            player_cavern: &String,
            wumpus_cavern: &String,
            pit_caverns: &PitCaverns,
            bat_caverns: &BatCaverns,
            quiver: u32,
            arrows_in: &ArrowsIn,
        ) -> (Option<String>, Option<u32>, Option<ArrowsIn>, Option<u32>);
    }

    pub struct HuntTheWumpusGame {
        connections: Connections,
        caverns: Caverns,
        player_cavern: String,
        message_receiver: Box<dyn HtwMessageReceiver>,
        bat_caverns: BatCaverns,
        pit_caverns: PitCaverns,
        wumpus_cavern: String,
        quiver: u32,
        arrows_in: ArrowsIn,
        command: Box<dyn Command>,
        hit_points: u32,
    }

    impl HuntTheWumpusGame {
        pub fn new(
            message_receiver: Box<dyn HtwMessageReceiver>,
            caverns: Caverns,
        ) -> Box<dyn HuntTheWumpus> {
            Box::new(HuntTheWumpusGame {
                connections: Connections::new(vec![]),
                caverns,
                player_cavern: String::from("None"),
                message_receiver,
                bat_caverns: HashSet::new(),
                pit_caverns: HashSet::new(),
                wumpus_cavern: String::from("None"),
                quiver: 0,
                arrows_in: HashMap::new(),
                command: Box::new(RestCommand {}),
                hit_points: 10,
            }) as Box<dyn HuntTheWumpus>
        }

        // TODO: write test
        fn report_status(&self) {
            let directions = self
                .connections
                .report_available_directions(&self.player_cavern);
            for direction in directions {
                self.message_receiver.passage(&direction);
            }

            if self
                .connections
                .report_nearby(&self.player_cavern, &self.bat_caverns)
            {
                self.message_receiver.hear_bats();
            }
            if self
                .connections
                .report_nearby(&self.player_cavern, &self.pit_caverns)
            {
                self.message_receiver.hear_pit();
            }
            if self.connections.report_nearby(
                &self.player_cavern,
                &HashSet::from([String::from(&self.wumpus_cavern)]),
            ) {
                self.message_receiver.smell_wumpus();
            }
        }

        fn move_wumpus(&mut self) {
            let mut wumpus_choices = vec![];
            for c in &self.connections.connections {
                if self.wumpus_cavern == c.from() {
                    wumpus_choices.push(c.to());
                }
            }
            wumpus_choices.push(&self.wumpus_cavern);

            let n_choices = wumpus_choices.len();
            let choice = rand::thread_rng().gen_range(0..n_choices);
            self.wumpus_cavern = String::from(wumpus_choices[choice]);
        }

        fn hit(&mut self, points: u32) {
            self.hit_points = self.hit_points.saturating_sub(points);
            if self.hit_points <= 0 {
                self.message_receiver.you_die();
            }
        }
    }

    impl HuntTheWumpus for HuntTheWumpusGame {
        fn set_player_cavern(&mut self, player_cavern: &str) {
            self.player_cavern = String::from(player_cavern);
        }
        fn get_player_cavern(&self) -> &str {
            &self.player_cavern
        }
        fn add_bat_cavern(&mut self, cavern: &str) {
            self.bat_caverns.insert(String::from(cavern));
        }
        fn add_pit_cavern(&mut self, cavern: &str) {
            self.pit_caverns.insert(String::from(cavern));
        }
        fn set_wumpus_cavern(&mut self, wumpus_cavern: &str) {
            self.wumpus_cavern = String::from(wumpus_cavern);
        }
        fn get_wumpus_cavern(&self) -> &str {
            &self.wumpus_cavern
        }
        fn set_quiver(&mut self, arrows: u32) {
            self.quiver = arrows;
        }
        fn get_quiver(&self) -> u32 {
            self.quiver
        }
        fn get_hit_points(&self) -> u32 {
            self.hit_points
        }
        fn check_wumpus_moved_to_player(&self) {
            if self.player_cavern == self.wumpus_cavern {
                self.message_receiver.wumpus_moves_to_player();
            }
        }
        fn execute_command(&mut self) {
            let (new_player_cavern, new_quiver, update_arrows_in, self_damage) =
                self.command.process_command(
                    &self.message_receiver,
                    &self.connections,
                    &self.caverns,
                    &self.player_cavern,
                    &self.wumpus_cavern,
                    &self.pit_caverns,
                    &self.bat_caverns,
                    self.quiver,
                    &self.arrows_in,
                );
            match new_player_cavern {
                Some(s) => self.player_cavern = s,
                _ => (),
            };
            match new_quiver {
                Some(s) => self.quiver = s,
                _ => (),
            };
            match update_arrows_in {
                Some(s) => self.arrows_in.extend(s),
                _ => (),
            };
            match self_damage {
                Some(u) => self.hit(u),
                _ => (),
            };
            self.move_wumpus();
            self.check_wumpus_moved_to_player();
            self.report_status();
        }
        fn make_rest_command(&mut self) {
            self.command = Box::new(RestCommand {});
        }
        fn make_shoot_command(&mut self, direction: Direction) {
            self.command = Box::new(ShootCommand::new(direction));
        }
        fn make_move_command(&mut self, direction: Direction) {
            self.command = Box::new(MoveCommand::new(direction));
        }

        fn caverns(&self) -> &Caverns {
            &self.caverns
        }

        fn connect_caverns(&mut self) {
            self.connections.connect_caverns(&self.caverns);
        }
    }

    #[cfg(test)]
    mod tests_for_hunt_the_wumpus_game {
        use super::*;
        use crate::connection::connection::Connection;
        use crate::english_message_receiver::english_htw_message_receiver::EnglishHtwMessageReceiver;

        fn type_of<T>(_: &T) -> &str {
            std::any::type_name::<T>()
        }

        fn set_up() -> HuntTheWumpusGame {
            // TODO: mock message_receiver
            let message_receiver = Box::new(EnglishHtwMessageReceiver {});
            let caverns = HashSet::from([
                String::from("cavern"),
                String::from("cavern_w"),
                String::from("cavern_e"),
                String::from("cavern_n"),
                String::from("cavern_s"),
                String::from("cavern_nn"),
            ]);
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
            let connections = Connections::new(connections);
            let player_cavern = String::from("cavern");
            let bat_caverns = HashSet::from([String::from("cavern_e")]);
            let pit_caverns = HashSet::from([String::from("cavern_s")]);
            let wumpus_cavern = String::from("cavern_w");
            let quiver = 5;
            let arrows_in = HashMap::new();
            let command = Box::new(RestCommand {});
            let hit_points = 10;
            HuntTheWumpusGame {
                message_receiver,
                caverns,
                connections,
                player_cavern,
                bat_caverns,
                pit_caverns,
                wumpus_cavern,
                quiver,
                arrows_in,
                command,
                hit_points,
            }
        }

        #[test]
        fn test_move_wumpus() {
            let mut game = set_up();
            let mut count = 0;
            while &game.wumpus_cavern == "cavern_w" || count == 100 {
                game.move_wumpus();
                count += 1;
            }
            assert_ne!(&game.wumpus_cavern, "cavern_w");
        }

        #[test]
        fn test_hit() {
            let mut game = set_up();
            assert_eq!(10, game.hit_points);
            game.hit(3);
            assert_eq!(7, game.hit_points);
        }

        #[test]
        fn test_set_player_cavern() {
            let mut game = set_up();
            assert_ne!(&game.player_cavern, "cavern_n");
            game.set_player_cavern("cavern_n");
            assert_eq!(&game.player_cavern, "cavern_n");
        }

        #[test]
        fn test_get_player_cavern() {
            let game = set_up();
            assert_eq!(game.get_player_cavern(), "cavern");
        }

        #[test]
        fn test_add_bat_cavern() {
            let mut game = set_up();
            assert_eq!(game.bat_caverns, HashSet::from([String::from("cavern_e")]));
            game.add_bat_cavern("cavern_nn");
            assert_eq!(
                game.bat_caverns,
                HashSet::from([String::from("cavern_e"), String::from("cavern_nn")])
            );
        }

        #[test]
        fn test_add_pit_cavern() {
            let mut game = set_up();
            assert_eq!(game.pit_caverns, HashSet::from([String::from("cavern_s")]));
            game.add_pit_cavern("cavern_nn");
            assert_eq!(
                game.pit_caverns,
                HashSet::from([String::from("cavern_s"), String::from("cavern_nn")])
            );
        }

        #[test]
        fn test_set_wumpus_cavern() {
            let mut game = set_up();
            assert_eq!(&game.wumpus_cavern, "cavern_w");
            game.set_wumpus_cavern("cavern_nn");
            assert_eq!(&game.wumpus_cavern, "cavern_nn");
        }

        #[test]
        fn test_get_wumpus_cavern() {
            let game = set_up();
            assert_eq!(game.get_wumpus_cavern(), &game.wumpus_cavern);
        }

        #[test]
        fn test_set_quiver() {
            let mut game = set_up();
            assert_eq!(game.quiver, 5);
            game.set_quiver(10);
            assert_eq!(game.quiver, 10);
        }

        #[test]
        fn test_get_quiver() {
            let game = set_up();
            assert_eq!(game.get_quiver(), game.quiver);
        }

        #[test]
        fn test_get_hit_points() {
            let game = set_up();
            assert_eq!(game.get_hit_points(), game.hit_points);
        }

        #[test]
        fn test_caverns_method() {
            let game = set_up();
            let caverns = game.caverns();
            assert_eq!(
                type_of(caverns),
                "std::collections::hash::set::HashSet<alloc::string::String>"
            );
            assert_eq!(caverns, &game.caverns);
        }

        #[test]
        fn test_connect_caverns() {
            let mut game = set_up();
            game.connect_caverns();
            assert_ne!(0, game.connections.connections.len());
        }

        //TODO: is it possible to test execute_command?
        //TODO: to test make_commands, Command needs to impl Debug, is it wise to do so?
    }
}
