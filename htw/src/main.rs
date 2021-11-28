// TODO: implement HtwFactory.java
// TODO: implement Main.java
// TODO: implement HuntTheWumpusGame.java

use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
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
    message_receiver.passage(&north);
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
    fn passage(&self, direction: &Direction);
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
        // hit(3); // TODO: imple
    }

    fn player_kills_wumpus(&self) {
        println!("You killed the Wumpus.");
        process::exit(0);
    }

    fn player_shoots_wall(&self) {
        println!("You shot the wall and the ricochet hurt you.");
        // hit(3); // TODO: imple
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
        // hit(4); // TODO: imple
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
    fn set_player_cavern(&mut self, player_cavern: &str);
    fn get_player_cavern(&self) -> &str;
    fn add_bat_cavern(&mut self, cavern: &str);
    fn add_pit_cavern(&mut self, cavern: &str);
    fn set_wumpus_cavern(&mut self, wumpus_cavern: &str);
    fn get_wumpus_cavern(&self) -> &str;
    fn set_quiver(&mut self, arrows: u32);
    fn get_quiver(&self) -> u32;
    fn get_arrows_in_cavern(&self, cavern: &str) -> u32;
    fn connect_cavern(&mut self, from: &str, to: &str, direction: Direction);
    fn find_destination(&self, cavern: &str, direction: Direction) -> Option<&str>;
    // fn make_rest_command(&self) -> Box<dyn Command>;
    // fn make_shoot_command(&self, direction: Direction) -> Box<dyn Command>;
    // fn make_move_command(&self, direction: Direction) -> Box<dyn Command>;
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

// HuntTheWumpusGame.java
struct Connection {
    from: String,
    to: String,
    direction: Direction,
}

impl Connection {
    fn new(from: &str, to: &str, direction: Direction) -> Connection {
        Connection {
            from: String::from(from),
            to: String::from(to),
            direction,
        }
    }
}

struct HuntTheWumpusGame {
    connections: Vec<Connection>,
    caverns: HashSet<String>,
    player_cavern: String,
    message_receiver: Box<dyn HtwMessageReceiver>,
    bat_caverns: HashSet<String>,
    pit_caverns: HashSet<String>,
    wumpus_cavern: String,
    quiver: u32,
    arrows_in: HashMap<String, u32>,
}

impl HuntTheWumpusGame {
    fn new(message_receiver: Box<dyn HtwMessageReceiver>) -> HuntTheWumpusGame {
        HuntTheWumpusGame {
            connections: vec![],
            caverns: HashSet::new(),
            player_cavern: String::from("None"),
            message_receiver,
            bat_caverns: HashSet::new(),
            pit_caverns: HashSet::new(),
            wumpus_cavern: String::from("None"),
            quiver: 0,
            arrows_in: HashMap::new(),
        }
    }

    fn report_status(&self) {
        self.report_available_directions();
        if self.report_nearby(&self.bat_caverns) {
            self.message_receiver.hear_bats();
        }
        if self.report_nearby(&self.pit_caverns) {
            self.message_receiver.hear_pit();
        }
        if self.report_nearby(&HashSet::from([String::from(&self.wumpus_cavern)])) {
            self.message_receiver.smell_wumpus();
        }
    }

    // TODO: see if report_neaby could be implemented using precicate
    // this is a work-around without predicate
    fn report_nearby(&self, test_caverns: &HashSet<String>) -> bool {
        for c in &self.connections {
            if c.from == self.player_cavern && test_caverns.contains(&String::from(&c.to)) {
                return true;
            }
        }
        false
    }

    fn report_available_directions(&self) {
        for c in &self.connections {
            if c.from == self.player_cavern {
                self.message_receiver.passage(&c.direction);
            }
        }
    }

    fn move_wumpus(&mut self) {
        let mut wumpus_choices = vec![];
        for c in &self.connections {
            if self.wumpus_cavern == c.from {
                wumpus_choices.push(&c.to);
            }
        }
        wumpus_choices.push(&self.wumpus_cavern);

        let n_choices = wumpus_choices.len();
        let choice = rand::thread_rng().gen_range(0..=n_choices);
        self.wumpus_cavern = String::from(wumpus_choices[choice]);
    }

    fn randomly_transport_player(&mut self) {
        let mut transport_choices = HashSet::new();
        transport_choices.extend(&self.caverns);
        transport_choices.remove(&self.player_cavern);
        let n_choices = transport_choices.len();
        let choice = rand::thread_rng().gen_range(0..=n_choices);
        self.player_cavern = Vec::from_iter(transport_choices)[choice].to_string();
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
    fn get_arrows_in_cavern(&self, cavern: &str) -> u32 {
        match self.arrows_in.get(cavern) {
            Some(&number) => number,
            None => 0,
        }
    }

    fn connect_cavern(&mut self, from: &str, to: &str, direction: Direction) {
        self.connections.push(Connection::new(from, to, direction));
        self.caverns.insert(String::from(from));
        self.caverns.insert(String::from(to));
    }
    fn find_destination(&self, cavern: &str, direction: Direction) -> Option<&str> {
        for c in &self.connections {
            if c.from == cavern && c.direction.name() == direction.name() {
                return Some(&c.to);
            }
        }
        None
    }
    // TODO: below 3 functions need GameCommand implement
    // fn make_rest_command(&self) -> Box<dyn Command>
    // fn make_shoot_command(&self, direction: Direction) -> Box<dyn Command>
    // fn make_move_command(&self, direction: Direction) -> Box<dyn Command>
}
