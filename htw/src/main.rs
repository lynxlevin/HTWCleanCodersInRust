// Todo: implement HtwFactory.java
// Todo: implement Main.java
// Todo: implement HuntTheWumpusGame.java

use std::process;

fn main() {
    let north = Direction::North;
    let south = Direction::South;
    let east = Direction::East;
    let west = Direction::West;
    println!("{:?}: opposite of {:?}", north, north.opposite());
    println!("{:?}: opposite of {:?}", south, south.opposite());
    println!("{:?}: opposite of {:?}", east, east.opposite());
    println!("{:?}: opposite of {:?}", west, west.opposite());

    let message_receiver = EnglishHtwMessageReceiver {};
    message_receiver.no_passage();
    message_receiver.hear_bats();
    message_receiver.hear_pit();
    message_receiver.smell_wumpus();
    message_receiver.passage(north);
    message_receiver.no_arrows();
    message_receiver.arrow_shot();
    message_receiver.player_shoots_self_in_back();
    // message_receiver.player_kills_wumpus();
    message_receiver.player_shoots_wall();
    message_receiver.arrows_found(1);
    message_receiver.arrows_found(2);
    message_receiver.fell_in_pit();
    // message_receiver.player_moves_to_wumpus();
    // message_receiver.wumpus_moves_to_player();
    message_receiver.bats_transport();
}

trait HtwMessageReceiver {
    fn no_passage(&self);
    fn hear_bats(&self);
    fn hear_pit(&self);
    fn smell_wumpus(&self);
    fn passage(&self, direction: Direction);
    fn no_arrows(&self);
    fn arrow_shot(&self);
    fn player_shoots_self_in_back(&self);
    fn player_kills_wumpus(&self);
    fn player_shoots_wall(&self);
    fn arrows_found(&self, arrows_found: u32);
    fn fell_in_pit(&self);
    fn player_moves_to_wumpus(&self);
    fn wumpus_moves_to_player(&self);
    fn bats_transport(&self);
}

struct EnglishHtwMessageReceiver {}
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

    fn passage(&self, direction: Direction) {
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
        // hit(3); // Todo: imple
    }

    fn player_kills_wumpus(&self) {
        println!("You killed the Wumpus.");
        process::exit(0);
    }

    fn player_shoots_wall(&self) {
        println!("You shot the wall and the ricochet hurt you.");
        // hit(3); // Todo: imple
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
        // hit(4); // Todo: imple
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

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn name(&self) -> &str {
        match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "West",
            Direction::West => "East",
        }
    }
}

trait HuntTheWumpus {
    fn set_player_cavern(player_cavern: &str);

    // fn getPlayerCavern() -> &str;
    fn add_bat_cavern(cavern: &str);
    fn add_pit_cavern(cavern: &str);
    fn set_wumpus_cavern(wumpus_cavern: &str);
    // fn getWumpusCavern() -> &str;
    fn set_quiver(arrows: u32);
    fn get_quiver() -> u32;
    fn get_arrows_in_cavern(cavern: &str) -> u32;
    fn connect_cavern(from: &str, to: &str, direction: Direction);
    fn find_destination(cavern: &str, direction: Direction) -> &str;
    // HuntTheWumpusGame.Command makeRestCommand(); // original code; a mistake? it should return HuntTheWumpus.Command
    fn make_rest_command() -> Box<dyn Command>;
    // HuntTheWumpusGame.Command makeShootCommand(Direction direction); // original code; a mistake? it should return HuntTheWumpus.Command
    fn make_shoot_command(direction: Direction) -> Box<dyn Command>;
    // HuntTheWumpusGame.Command makeMoveCommand(Direction direction); // original code; a mistake? it should return HuntTheWumpus.Command
    fn make_move_command(direction: Direction) -> Box<dyn Command>;
}

trait Command {
    fn execute(&self);
}

// just to kill error
struct DummyCommand {}
impl Command for DummyCommand {
    fn execute(&self) {
        println!("test");
    }
}

// // HuntTheWumpusGame.java
// struct HuntTheWumpusGame {
//     // private List<Connection> connections = new ArrayList<>();

//     // private Set<String> caverns = new HashSet<>();
//     player_cavern: String,
//     message_receiver: Box<dyn HtwMessageReceiver>,
//     // private Set<String> batCaverns = new HashSet<>();
//     // private Set<String> pitCaverns = new HashSet<>();
//     wumpus_cavern: String,
//     quiver: i32,
//     // private Map<String, Integer> arrowsIn = new HashMap<>();
// }

// impl HuntTheWumpusGame {
//     fn new(message_receiver: Box<dyn HtwMessageReceiver>) -> HuntTheWumpusGame {
//         HuntTheWumpusGame {
//             player_cavern: String::from("None"),
//             message_receiver,
//             wumpus_cavern: String::from("None"),
//             quiver: 0,
//         }
//     }
// }
