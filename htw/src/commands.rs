pub mod commands {
    use crate::connection::connection::Connection;
    use crate::connections::connections::Connections;
    use crate::direction::direction::Direction;
    use crate::htw_game::htw_game::{ArrowsIn, BatCaverns, Caverns, Command, PitCaverns};
    use crate::HtwMessageReceiver;
    use rand::Rng;
    use std::collections::{HashMap, HashSet};

    pub struct RestCommand {}
    impl Command for RestCommand {
        fn process_command(
            &self,
            _message_receiver: &Box<dyn HtwMessageReceiver>,
            _connections: &Connections,
            _caverns: &Caverns,
            _player_cavern: &String,
            _wumpus_cavern: &String,
            _pit_caverns: &PitCaverns,
            _bat_caverns: &BatCaverns,
            _quiver: u32,
            _arrows_in: &ArrowsIn,
        ) -> (Option<String>, Option<u32>, Option<ArrowsIn>, Option<u32>) {
            (None, None, None, None)
        }
    }

    pub struct MoveCommand {
        direction: Direction,
    }
    impl MoveCommand {
        pub fn new(direction: Direction) -> MoveCommand {
            MoveCommand { direction }
        }

        fn check_for_wumpus(
            &self,
            message_receiver: &Box<dyn HtwMessageReceiver>,
            player_cavern: &String,
            wumpus_cavern: &String,
        ) {
            if wumpus_cavern == player_cavern {
                message_receiver.player_moves_to_wumpus();
            }
        }

        fn check_for_pit(
            &self,
            message_receiver: &Box<dyn HtwMessageReceiver>,
            player_cavern: &String,
            pit_caverns: &PitCaverns,
        ) -> Option<u32> {
            let mut self_damage = None;
            if pit_caverns.contains(player_cavern) {
                message_receiver.fell_in_pit();
                self_damage = Some(4);
            }
            self_damage
        }

        fn randomly_transport_player(&self, caverns: &Caverns, player_cavern: &String) -> String {
            let mut transport_choices = HashSet::new();
            transport_choices.extend(caverns);
            transport_choices.remove(&player_cavern);
            let n_choices = transport_choices.len();
            let choice = rand::thread_rng().gen_range(0..n_choices);
            Vec::from_iter(transport_choices)[choice].to_string()
        }

        fn check_for_bats(
            &self,
            message_receiver: &Box<dyn HtwMessageReceiver>,
            caverns: &Caverns,
            player_cavern: &String,
            bat_caverns: &BatCaverns,
        ) -> Option<String> {
            if bat_caverns.contains(player_cavern) {
                message_receiver.bats_transport();
                let new_player_cavern = self.randomly_transport_player(caverns, player_cavern);
                Some(new_player_cavern)
            } else {
                None
            }
        }

        fn get_arrows_in_cavern(&self, arrows_in: &ArrowsIn, cavern: &String) -> u32 {
            match arrows_in.get(cavern) {
                Some(&number) => number,
                None => 0,
            }
        }

        fn check_for_arrows(
            &self,
            message_receiver: &Box<dyn HtwMessageReceiver>,
            player_cavern: &String,
            quiver: u32,
            arrows_in: &ArrowsIn,
        ) -> (Option<u32>, Option<ArrowsIn>) {
            let arrows_found = self.get_arrows_in_cavern(arrows_in, player_cavern);
            let mut new_quiver = None;
            if arrows_found > 0 {
                message_receiver.arrows_found(arrows_found);
                new_quiver = Some(quiver + arrows_found);
            }
            let update_arrows_in = Some(HashMap::from([(player_cavern.to_string(), 0)]));
            (new_quiver, update_arrows_in)
        }
    }
    impl Command for MoveCommand {
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
        ) -> (Option<String>, Option<u32>, Option<ArrowsIn>, Option<u32>) {
            match connections.find_destination(player_cavern, &self.direction) {
                Some(s) => {
                    let new_player_cavern = s;
                    self.check_for_wumpus(message_receiver, &new_player_cavern, wumpus_cavern);
                    let self_damage =
                        self.check_for_pit(message_receiver, &new_player_cavern, pit_caverns);
                    let new_player_cavern = match self.check_for_bats(
                        message_receiver,
                        caverns,
                        &new_player_cavern,
                        bat_caverns,
                    ) {
                        Some(s) => s,
                        None => new_player_cavern,
                    };
                    let (new_quiver, update_arrows_in) = self.check_for_arrows(
                        message_receiver,
                        &new_player_cavern,
                        quiver,
                        arrows_in,
                    );
                    let new_player_cavern = Some(new_player_cavern);
                    return (new_player_cavern, new_quiver, update_arrows_in, self_damage);
                }
                None => {
                    message_receiver.no_passage();
                    return (None, None, None, None);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests_for_move_command {
        use super::*;
        use crate::english_message_receiver::english_htw_message_receiver::EnglishHtwMessageReceiver;

        fn set_up_command() -> MoveCommand {
            let direction = Direction::North;
            MoveCommand::new(direction)
        }

        fn set_up() -> (
            Box<dyn HtwMessageReceiver>,
            Caverns,
            BatCaverns,
            PitCaverns,
            ArrowsIn,
            u32,
            MoveCommand,
        ) {
            let message_receiver = Box::new(EnglishHtwMessageReceiver {});
            let caverns = HashSet::from([
                String::from("cavern"),
                String::from("cavern_w"),
                String::from("cavern_e"),
                String::from("cavern_n"),
                String::from("cavern_s"),
                String::from("cavern_nn"),
            ]);
            let bat_caverns = HashSet::from([String::from("cavern_n")]);
            let pit_caverns = HashSet::from([String::from("cavern_n"), String::from("cavern_nn")]);
            let arrows_in = HashMap::from([(String::from("cavern_n"), 5)]);
            let quiver = 5;
            let command = set_up_command();
            (
                message_receiver,
                caverns,
                bat_caverns,
                pit_caverns,
                arrows_in,
                quiver,
                command,
            )
        }

        #[test]
        fn test_check_for_pit_no_pit() {
            let (message_receiver, _, _, pit_caverns, _, _, command) = set_up();
            let player_cavern = String::from("cavern");
            assert_eq!(
                None,
                command.check_for_pit(&message_receiver, &player_cavern, &pit_caverns)
            );
        }

        #[test]
        fn test_check_for_pit_pit_exists() {
            let (message_receiver, _, _, pit_caverns, _, _, command) = set_up();
            let player_cavern = String::from("cavern_n");
            assert_eq!(
                Some(4),
                command.check_for_pit(&message_receiver, &player_cavern, &pit_caverns)
            );
        }

        #[test]
        fn test_randomly_transport_player() {
            let player_cavern = String::from("cavern");
            let (_, caverns, _, _, _, _, command) = set_up();
            let result = command.randomly_transport_player(&caverns, &player_cavern);
            assert_ne!(String::from("cavern"), result);
        }

        #[test]
        fn test_check_for_bats_no_bats() {
            let (message_receiver, caverns, bat_caverns, _, _, _, command) = set_up();
            let player_cavern = String::from("cavern");
            let result =
                command.check_for_bats(&message_receiver, &caverns, &player_cavern, &bat_caverns);
            assert_eq!(None, result);
        }

        #[test]
        fn test_check_for_bats_bat_exists() {
            let (message_receiver, caverns, bat_caverns, _, _, _, command) = set_up();
            let player_cavern = String::from("cavern_n");
            let result =
                command.check_for_bats(&message_receiver, &caverns, &player_cavern, &bat_caverns);
            assert_ne!(None, result);
            assert_ne!(Some(String::from("cavern_n")), result);
        }

        #[test]
        fn test_get_arrows_in_cavern_no_arrows() {
            let (_, _, _, _, arrows_in, _, command) = set_up();
            let cavern = String::from("cavern");
            let result = command.get_arrows_in_cavern(&arrows_in, &cavern);
            assert_eq!(0, result);
        }

        #[test]
        fn test_get_arrows_in_cavern_5_arrows() {
            let (_, _, _, _, arrows_in, _, command) = set_up();
            let cavern = String::from("cavern_n");
            let result = command.get_arrows_in_cavern(&arrows_in, &cavern);
            assert_eq!(5, result);
        }

        #[test]
        fn test_check_for_arrows_no_arrows() {
            let player_cavern = String::from("cavern");
            let (message_receiver, _, _, _, arrows_in, quiver, command) = set_up();
            let result =
                command.check_for_arrows(&message_receiver, &player_cavern, quiver, &arrows_in);
            assert_eq!(
                (None, Some(HashMap::from([(String::from("cavern"), 0)]))),
                result
            );
        }

        #[test]
        fn test_check_for_arrows_5_arrows() {
            let player_cavern = String::from("cavern_n");
            let (message_receiver, _, _, _, arrows_in, quiver, command) = set_up();
            let result =
                command.check_for_arrows(&message_receiver, &player_cavern, quiver, &arrows_in);
            assert_eq!(
                (
                    Some(10),
                    Some(HashMap::from([(String::from("cavern_n"), 0)]))
                ),
                result
            );
        }
    }

    pub struct ShootCommand {
        direction: Direction,
    }
    impl ShootCommand {
        pub fn new(direction: Direction) -> ShootCommand {
            ShootCommand { direction }
        }

        fn get_arrows_in_cavern(&self, arrows_in: &ArrowsIn, cavern: &String) -> u32 {
            match arrows_in.get(cavern) {
                Some(&number) => number,
                None => 0,
            }
        }

        fn increment_arrows_in_cavern(
            &self,
            arrows_in: &ArrowsIn,
            arrow_cavern: &String,
        ) -> Option<ArrowsIn> {
            let arrows = self.get_arrows_in_cavern(arrows_in, arrow_cavern);
            let update_arrows_in = Some(HashMap::from([(arrow_cavern.to_string(), arrows + 1)]));
            update_arrows_in
        }
    }
    impl Command for ShootCommand {
        fn process_command(
            &self,
            message_receiver: &Box<dyn HtwMessageReceiver>,
            connections: &Connections,
            _caverns: &Caverns,
            player_cavern: &String,
            wumpus_cavern: &String,
            _pit_caverns: &PitCaverns,
            _bat_caverns: &BatCaverns,
            quiver: u32,
            arrows_in: &ArrowsIn,
        ) -> (Option<String>, Option<u32>, Option<ArrowsIn>, Option<u32>) {
            if quiver == 0 {
                message_receiver.no_arrows();
                return (None, None, None, None);
            } else {
                message_receiver.arrow_shot();
                let new_quiver = Some(quiver - 1);
                let mut arrow_tracker = ArrowTracker::new(player_cavern.to_string());
                let self_damage = arrow_tracker.track_arrow(
                    &self.direction,
                    message_receiver,
                    &connections.connections,
                    player_cavern,
                    wumpus_cavern,
                );
                if arrow_tracker.arrow_hit_something() {
                    return (None, None, None, self_damage);
                } else {
                    let update_arrows_in = self
                        .increment_arrows_in_cavern(arrows_in, &arrow_tracker.get_arrow_cavern());
                    return (None, new_quiver, update_arrows_in, None);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests_for_shoot_command {
        use super::*;
        fn set_up() -> (ShootCommand, ArrowsIn) {
            let direction = Direction::North;
            let command = ShootCommand::new(direction);
            let arrows_in = HashMap::from([(String::from("cavern_n"), 5)]);
            (command, arrows_in)
        }

        #[test]
        fn test_get_arrows_in_cavern_none() {
            let (command, arrows_in) = set_up();
            let cavern = String::from("cavern_s");
            let result = command.get_arrows_in_cavern(&arrows_in, &cavern);
            assert_eq!(0, result);
        }

        #[test]
        fn test_get_arrows_in_cavern_some() {
            let (command, arrows_in) = set_up();
            let cavern = String::from("cavern_n");
            let result = command.get_arrows_in_cavern(&arrows_in, &cavern);
            assert_eq!(5, result);
        }

        #[test]
        fn test_increment_arrows_in_cavern() {
            let (command, arrows_in) = set_up();
            let arrow_cavern = String::from("cavern_n");
            let result = command.increment_arrows_in_cavern(&arrows_in, &arrow_cavern);
            assert_eq!(Some(HashMap::from([(String::from("cavern_n"), 6)])), result);
        }
    }

    struct ArrowTracker {
        hit_something: bool,
        arrow_cavern: String,
    }
    impl ArrowTracker {
        fn new(starting_cavern: String) -> ArrowTracker {
            ArrowTracker {
                hit_something: false,
                arrow_cavern: starting_cavern,
            }
        }

        fn arrow_hit_something(&self) -> bool {
            self.hit_something
        }

        fn get_arrow_cavern(&self) -> String {
            self.arrow_cavern.to_string()
        }

        fn next_cavern(
            &self,
            cavern: String,
            direction: &Direction,
            connections: &Vec<Connection>,
        ) -> Option<String> {
            for c in connections {
                if cavern == c.from() && direction == c.direction() {
                    return Some(c.to().to_string());
                }
            }
            return None;
        }

        fn shoot_self_in_back(
            &mut self,
            message_receiver: &Box<dyn HtwMessageReceiver>,
        ) -> Option<u32> {
            message_receiver.player_shoots_self_in_back();
            self.hit_something = true;
            let self_damage = Some(3);
            self_damage
        }

        fn shoot_wumpus(&mut self, message_receiver: &Box<dyn HtwMessageReceiver>) {
            message_receiver.player_kills_wumpus();
            self.hit_something = true;
        }

        fn shoot_wall(&mut self, message_receiver: &Box<dyn HtwMessageReceiver>) -> Option<u32> {
            message_receiver.player_shoots_wall();
            self.hit_something = true;
            let self_damage = Some(3);
            self_damage
        }

        fn track_arrow(
            &mut self,
            direction: &Direction,
            message_receiver: &Box<dyn HtwMessageReceiver>,
            connections: &Vec<Connection>,
            player_cavern: &String,
            wumpus_cavern: &String,
        ) -> Option<u32> {
            let mut count = 0;
            while let Some(c) =
                self.next_cavern(self.arrow_cavern.to_string(), direction, connections)
            {
                count += 1;
                self.arrow_cavern = c;
                if &self.arrow_cavern == player_cavern {
                    let self_damage = self.shoot_self_in_back(message_receiver);
                    return self_damage;
                };
                if &self.arrow_cavern == wumpus_cavern {
                    self.shoot_wumpus(message_receiver);
                    return None;
                };
                if count > 100 {
                    return None;
                };
            }
            // when there is no connecting cavern in the shooting direction, the arrow hits wall.
            if &self.arrow_cavern == player_cavern {
                let self_damage = self.shoot_wall(message_receiver);
                return self_damage;
            }
            None
        }
    }

    #[cfg(test)]
    mod tests_for_arrow_tracker {
        use super::*;
        use crate::connection::connection::Connection;
        use crate::english_message_receiver::english_htw_message_receiver::EnglishHtwMessageReceiver;

        fn set_up_tracker() -> ArrowTracker {
            ArrowTracker::new(String::from("cavern"))
        }

        fn set_up() -> (
            ArrowTracker,
            Box<dyn HtwMessageReceiver>,
            Direction,
            Vec<Connection>,
        ) {
            let tracker = set_up_tracker();
            let message_receiver = Box::new(EnglishHtwMessageReceiver {});
            let direction = Direction::North;
            let connections = vec![
                Connection::new("cavern", "cavern_n", &Direction::North),
                Connection::new("cavern_n", "cavern", &Direction::South),
                Connection::new("cavern_n", "cavern_nn", &Direction::North),
                Connection::new("cavern_nn", "cavern_n", &Direction::South),
                Connection::new("cavern_nn", "cavern", &Direction::North),
                Connection::new("cavern", "cavern_nn", &Direction::South),
                Connection::new("cavern", "cavern_w", &Direction::West),
                Connection::new("cavern_w", "cavern", &Direction::East),
            ];
            (tracker, message_receiver, direction, connections)
        }

        #[test]
        fn test_new() {
            let result = ArrowTracker::new(String::from("cavern"));
            assert_eq!(result.hit_something, false);
            assert_eq!(result.arrow_cavern, "cavern");
        }

        #[test]
        fn test_arrow_hit_something() {
            let tracker = set_up_tracker();
            assert_eq!(tracker.hit_something, tracker.arrow_hit_something());
        }

        #[test]
        fn test_get_arrow_cavern() {
            let tracker = set_up_tracker();
            assert_eq!(tracker.arrow_cavern, tracker.get_arrow_cavern());
        }

        #[test]
        fn test_next_cavern_exists_not() {
            let (tracker, _, _, connections) = set_up();
            let direction = Direction::East;
            let cavern = String::from("cavern");
            let result = tracker.next_cavern(cavern, &direction, &connections);
            assert_eq!(None, result);
        }

        #[test]
        fn test_next_cavern_exists() {
            let (tracker, _, direction, connections) = set_up();
            let cavern = String::from("cavern");
            let result = tracker.next_cavern(cavern, &direction, &connections);
            assert_eq!(Some(String::from("cavern_n")), result);
        }

        #[test]
        fn test_shoot_self_in_back() {
            let (mut tracker, message_receiver, _, _) = set_up();
            let result = tracker.shoot_self_in_back(&message_receiver);
            assert_eq!(result, Some(3));
        }

        #[test]
        fn test_shoot_wall() {
            let (mut tracker, message_receiver, _, _) = set_up();
            let self_damage = tracker.shoot_wall(&message_receiver);
            assert!(tracker.arrow_hit_something());
            assert_eq!(Some(3), self_damage);
        }

        #[test]
        fn test_track_arrow_travels_over_100_caverns() {
            let (mut tracker, message_receiver, direction, connections) = set_up();
            let player_cavern = String::from("none");
            let wumpus_cavern = String::from("none");
            let result = tracker.track_arrow(
                &direction,
                &message_receiver,
                &connections,
                &player_cavern,
                &wumpus_cavern,
            );
            assert_eq!(None, result);
        }

        #[test]
        fn test_track_arrow_shoots_self() {
            let (mut tracker, message_receiver, direction, connections) = set_up();
            let player_cavern = String::from("cavern");
            let wumpus_cavern = String::from("none");
            let result = tracker.track_arrow(
                &direction,
                &message_receiver,
                &connections,
                &player_cavern,
                &wumpus_cavern,
            );
            assert_eq!(Some(3), result);
            assert!(tracker.arrow_hit_something());
        }

        #[test]
        fn test_track_arrow_shoots_wall() {
            let (mut tracker, message_receiver, _, connections) = set_up();
            let direction = Direction::East;
            let player_cavern = String::from("cavern");
            let wumpus_cavern = String::from("none");
            let result = tracker.track_arrow(
                &direction,
                &message_receiver,
                &connections,
                &player_cavern,
                &wumpus_cavern,
            );
            assert_eq!(Some(3), result);
            assert!(tracker.arrow_hit_something());
        }

        #[test]
        fn test_track_arrow_hits_nothing() {
            let (mut tracker, message_receiver, _, connections) = set_up();
            let direction = Direction::West;
            let player_cavern = String::from("cavern");
            let wumpus_cavern = String::from("none");
            let result = tracker.track_arrow(
                &direction,
                &message_receiver,
                &connections,
                &player_cavern,
                &wumpus_cavern,
            );
            assert_eq!(None, result);
            assert!(!tracker.arrow_hit_something());
        }
    }
}
