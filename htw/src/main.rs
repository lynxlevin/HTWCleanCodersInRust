fn main() {
    let north = Direction::North;
    let south = Direction::South;
    let east = Direction::East;
    let west = Direction::West;
    println!("{:?}: opposite of {:?}", north, north.opposite());
    println!("{:?}: opposite of {:?}", south, south.opposite());
    println!("{:?}: opposite of {:?}", east, east.opposite());
    println!("{:?}: opposite of {:?}", west, west.opposite());
}

trait HtwMessageReceiver {
    fn no_passage();
    fn hear_bats();
    fn hear_pit();
    fn smell_wumpus();
    fn passage(direction: Direction);
    fn no_arrows();
    fn arrow_shot();
    fn player_shoots_self_in_back();
    fn player_kills_wumpus();
    fn player_shoots_wall();
    fn arrows_found(arrows_found: u32);
    fn fell_in_pit();
    fn player_moves_to_wumpus();
    fn wumpus_moves_to_player();
    fn bats_transport();
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
