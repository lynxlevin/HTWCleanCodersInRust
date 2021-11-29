use htw::Direction;
use htw::EnglishHtwMessageReceiver;
use htw::HtwMessageReceiver;

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
