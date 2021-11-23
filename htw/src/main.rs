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
    // fn passage(direction: HuntTheWumpus::Direction);
    fn no_arrows();
    fn arrow_shot();
    fn player_shoots_self_in_back();
    fn player_kills_wumpus();
    fn player_shoots_wall();
    fn arrows_found(arrows_found: i32);
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
