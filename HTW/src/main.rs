fn main() {
    println!("Hello, world!");
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
