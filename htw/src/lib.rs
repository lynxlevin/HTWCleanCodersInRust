use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::process;

pub trait HtwMessageReceiver {
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

pub struct EnglishHtwMessageReceiver {}
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
    fn connect_cavern(&mut self, from: &str, to: &str, direction: Direction);
    fn check_wumpus_moved_to_player(&self);
    fn execute_command(&mut self);
    fn make_rest_command(&mut self);
    fn make_shoot_command(&mut self, direction: Direction);
    fn make_move_command(&mut self, direction: Direction);
}

trait Command {
    fn process_command(
        &self,
        message_receiver: &Box<dyn HtwMessageReceiver>,
        connections: &Vec<Connection>,
        caverns: &HashSet<String>,
        player_cavern: &String,
        wumpus_cavern: &String,
        pit_caverns: &HashSet<String>,
        bat_caverns: &HashSet<String>,
        quiver: u32,
        arrows_in: &HashMap<String, u32>,
    ) -> (Option<String>, Option<u32>, Option<HashMap<String, u32>>);
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
    command: Box<dyn Command>,
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
            command: Box::new(RestCommand {}),
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

    // TODO: see if report_neaby could be implemented using predicate
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

    fn connect_cavern(&mut self, from: &str, to: &str, direction: Direction) {
        self.connections.push(Connection::new(from, to, direction));
        self.caverns.insert(String::from(from));
        self.caverns.insert(String::from(to));
    }
    fn check_wumpus_moved_to_player(&self) {
        if self.player_cavern == self.wumpus_cavern {
            self.message_receiver.wumpus_moves_to_player();
        }
    }
    fn execute_command(&mut self) {
        let (new_player_cavern, new_quiver, update_arrows_in) = self.command.process_command(
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
        self.move_wumpus();
        self.check_wumpus_moved_to_player();
        self.report_status();
    }
    fn make_rest_command(&mut self) {
        self.command = Box::new(RestCommand {});
    }
    fn make_shoot_command(&mut self, direction: Direction) {
        self.command = Box::new(ShootCommand { direction });
    }
    fn make_move_command(&mut self, direction: Direction) {
        self.command = Box::new(MoveCommand { direction });
    }
}

struct RestCommand {}
impl Command for RestCommand {
    fn process_command(
        &self,
        _message_receiver: &Box<dyn HtwMessageReceiver>,
        _connections: &Vec<Connection>,
        _caverns: &HashSet<String>,
        _player_cavern: &String,
        _wumpus_cavern: &String,
        _pit_caverns: &HashSet<String>,
        _bat_caverns: &HashSet<String>,
        _quiver: u32,
        _arrows_in: &HashMap<String, u32>,
    ) -> (Option<String>, Option<u32>, Option<HashMap<String, u32>>) {
        (None, None, None)
    }
}

struct MoveCommand {
    direction: Direction,
}
impl MoveCommand {
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
        pit_caverns: &HashSet<String>,
    ) {
        if pit_caverns.contains(player_cavern) {
            message_receiver.fell_in_pit();
        }
    }

    fn randomly_transport_player(
        &self,
        caverns: &HashSet<String>,
        player_cavern: &String,
    ) -> String {
        let mut transport_choices = HashSet::new();
        transport_choices.extend(caverns);
        transport_choices.remove(&player_cavern);
        let n_choices = transport_choices.len();
        let choice = rand::thread_rng().gen_range(0..=n_choices);
        Vec::from_iter(transport_choices)[choice].to_string()
    }

    fn check_for_bats(
        &self,
        message_receiver: &Box<dyn HtwMessageReceiver>,
        caverns: &HashSet<String>,
        player_cavern: &String,
        bat_caverns: &HashSet<String>,
    ) -> Option<String> {
        if bat_caverns.contains(player_cavern) {
            message_receiver.bats_transport();
            let new_player_cavern = self.randomly_transport_player(caverns, player_cavern);
            Some(new_player_cavern)
        } else {
            None
        }
    }

    fn get_arrows_in_cavern(&self, arrows_in: &HashMap<String, u32>, cavern: &String) -> u32 {
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
        arrows_in: &HashMap<String, u32>,
    ) -> (Option<u32>, Option<HashMap<String, u32>>) {
        let arrows_found = self.get_arrows_in_cavern(arrows_in, player_cavern);
        let mut new_quiver = None;
        if arrows_found > 0 {
            message_receiver.arrows_found(arrows_found);
            new_quiver = Some(quiver + arrows_found);
        }
        let update_arrows_in = Some(HashMap::from([(player_cavern.to_string(), 0)]));
        (new_quiver, update_arrows_in)
    }

    fn find_destination(
        &self,
        player_cavern: &String,
        direction: &Direction,
        connections: &Vec<Connection>,
    ) -> Option<String> {
        for c in connections {
            if &c.from == player_cavern && c.direction.name() == direction.name() {
                return Some(c.to.to_string());
            }
        }
        None
    }
}
impl Command for MoveCommand {
    fn process_command(
        &self,
        message_receiver: &Box<dyn HtwMessageReceiver>,
        connections: &Vec<Connection>,
        caverns: &HashSet<String>,
        player_cavern: &String,
        wumpus_cavern: &String,
        pit_caverns: &HashSet<String>,
        bat_caverns: &HashSet<String>,
        quiver: u32,
        arrows_in: &HashMap<String, u32>,
    ) -> (Option<String>, Option<u32>, Option<HashMap<String, u32>>) {
        match self.find_destination(player_cavern, &self.direction, connections) {
            Some(s) => {
                let new_player_cavern = s;
                self.check_for_wumpus(message_receiver, &new_player_cavern, wumpus_cavern);
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
                let (new_quiver, update_arrows_in) =
                    self.check_for_arrows(message_receiver, &new_player_cavern, quiver, arrows_in);
                let new_player_cavern = Some(new_player_cavern);
                return (new_player_cavern, new_quiver, update_arrows_in);
            }
            None => {
                message_receiver.no_passage();
                return (None, None, None);
            }
        }
    }
}

struct ShootCommand {
    direction: Direction,
}
impl ShootCommand {
    fn get_arrows_in_cavern(&self, arrows_in: &HashMap<String, u32>, cavern: &String) -> u32 {
        match arrows_in.get(cavern) {
            Some(&number) => number,
            None => 0,
        }
    }

    fn increment_arrows_in_cavern(
        &self,
        arrows_in: &HashMap<String, u32>,
        arrow_cavern: &String,
    ) -> Option<HashMap<String, u32>> {
        let arrows = self.get_arrows_in_cavern(arrows_in, arrow_cavern);
        let update_arrows_in = Some(HashMap::from([(arrow_cavern.to_string(), arrows + 1)]));
        update_arrows_in
    }
}
impl Command for ShootCommand {
    fn process_command(
        &self,
        message_receiver: &Box<dyn HtwMessageReceiver>,
        connections: &Vec<Connection>,
        _caverns: &HashSet<String>,
        player_cavern: &String,
        wumpus_cavern: &String,
        _pit_caverns: &HashSet<String>,
        _bat_caverns: &HashSet<String>,
        quiver: u32,
        arrows_in: &HashMap<String, u32>,
    ) -> (Option<String>, Option<u32>, Option<HashMap<String, u32>>) {
        if quiver == 0 {
            message_receiver.no_arrows();
            return (None, None, None);
        } else {
            message_receiver.arrow_shot();
            let new_quiver = Some(quiver - 1);
            let mut arrow_tracker = ArrowTracker {
                hit_something: false,
                arrow_cavern: player_cavern.to_string(),
            };
            arrow_tracker.track_arrow(
                &self.direction,
                message_receiver,
                connections,
                player_cavern,
                wumpus_cavern,
            );
            if arrow_tracker.arrow_hit_something() {
                return (None, None, None);
            } else {
                let update_arrows_in =
                    self.increment_arrows_in_cavern(arrows_in, &arrow_tracker.get_arrow_cavern());
                return (None, new_quiver, update_arrows_in);
            }
        }
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
            if cavern == c.from && direction.name() == c.direction.name() {
                return Some(c.to.to_string());
            }
        }
        return None;
    }

    fn shot_self_in_back(
        &mut self,
        message_receiver: &Box<dyn HtwMessageReceiver>,
        player_cavern: &String,
    ) -> bool {
        if &self.arrow_cavern == player_cavern {
            message_receiver.player_shoots_self_in_back();
            self.hit_something = true;
            true
        } else {
            false
        }
    }

    fn shot_wumpus(
        &mut self,
        message_receiver: &Box<dyn HtwMessageReceiver>,
        wumpus_cavern: &String,
    ) -> bool {
        if &self.arrow_cavern == wumpus_cavern {
            message_receiver.player_kills_wumpus();
            self.hit_something = true;
            true
        } else {
            false
        }
    }

    fn track_arrow(
        &mut self,
        direction: &Direction,
        message_receiver: &Box<dyn HtwMessageReceiver>,
        connections: &Vec<Connection>,
        player_cavern: &String,
        wumpus_cavern: &String,
    ) {
        let mut next_cavern = None;
        let mut count = 0;
        while {
            next_cavern = self.next_cavern(self.arrow_cavern.to_string(), direction, connections);
            &next_cavern
        } != &None
        {
            count += 1;
            self.arrow_cavern = next_cavern.unwrap();
            if self.shot_self_in_back(message_receiver, player_cavern) {
                return ();
            };
            if self.shot_wumpus(message_receiver, wumpus_cavern) {
                return ();
            };
            if count > 100 {
                return ();
            };
            if &self.arrow_cavern == player_cavern {
                message_receiver.player_shoots_wall();
                return ();
            }
        }
    }
}
