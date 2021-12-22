pub mod english_htw_message_receiver {
    use crate::{Direction, HtwMessageReceiver};
    use std::process;

    pub struct EnglishHtwMessageReceiver {}

    impl EnglishHtwMessageReceiver {}

    impl HtwMessageReceiver for EnglishHtwMessageReceiver {
        fn no_passage(&self) {
            println!("No Passage.");
        }

        fn hear_bats(&self) {
            println!("You hear chirping.");
        }

        fn hear_pit(&self) {
            println!("You hear wind.");
        }

        fn smell_wumpus(&self) {
            println!("There is a terrible smell.");
        }

        fn passage(&self, direction: &Direction) {
            println!("You can go {}", direction.name());
        }

        fn no_arrows(&self) {
            println!("You have no arrows.");
        }

        fn arrow_shot(&self) {
            println!("Thwang!");
        }

        fn player_shoots_self_in_back(&self) {
            println!("Ow!  You shot yourself in the back.");
        }

        fn player_kills_wumpus(&self) {
            println!("You killed the Wumpus.");
            process::exit(0);
        }

        fn player_shoots_wall(&self) {
            println!("You shot the wall and the ricochet hurt you.");
        }

        fn arrows_found(&self, arrows_found: u32) {
            let mut plural = "";
            if arrows_found != 1 {
                plural = "s";
            }
            println!("You found {} arrow{}.", arrows_found, plural);
        }

        fn fell_in_pit(&self) {
            println!("You fell in a pit and hurt yourself.");
        }

        fn player_moves_to_wumpus(&self) {
            println!("You walked into the waiting arms of the Wumpus.");
            process::exit(0);
        }

        fn wumpus_moves_to_player(&self) {
            println!("The Wumpus has found you.");
            process::exit(0);
        }

        fn bats_transport(&self) {
            println!("Some bats carried you away.");
        }
    }
}
