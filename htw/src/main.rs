use htw::Caverns;
use htw::Direction;
use htw::EnglishHtwMessageReceiver;
use htw::HuntTheWumpus;
use htw::HuntTheWumpusGame;
use rand::Rng;
use std::collections::HashSet;
use std::io;
use std::process;

fn main() {
  let message_receiver = EnglishHtwMessageReceiver {};
  let caverns = create_caverns();
  let mut game = HuntTheWumpusGame::new(Box::new(message_receiver), caverns);
  game.connect_caverns();
  set_special_caverns(&mut game);
  game.set_quiver(5);
  game.make_rest_command();
  game.execute_command();
  loop {
    println!("{}", game.get_player_cavern());
    println!(
      "Health: {} arrows: {}",
      game.get_hit_points(),
      game.get_quiver()
    );
    println!("game.make_rest_command");
    game.make_rest_command();
    println!(">");
    let mut command = String::new();
    match io::stdin().read_line(&mut command) {
      Ok(_) => match &*command.trim().to_lowercase() {
        "e" => game.make_move_command(Direction::East),
        "w" => game.make_move_command(Direction::West),
        "n" => game.make_move_command(Direction::North),
        "s" => game.make_move_command(Direction::South),
        "se" => game.make_shoot_command(Direction::East),
        "sw" => game.make_shoot_command(Direction::West),
        "sn" => game.make_shoot_command(Direction::North),
        "ss" => game.make_shoot_command(Direction::South),
        "q" => process::exit(0),
        _ => game.make_rest_command(),
      },
      Err(error) => println!("error: {}", error),
    }
    game.execute_command();
  }
}

fn create_caverns() -> Caverns {
  let mut caverns = HashSet::new();
  let mut n_caverns = rand::thread_rng().gen_range(0..=30) + 10;
  while n_caverns > 0 {
    caverns.insert(make_name());
    n_caverns -= 1;
  }
  caverns
}

fn make_name() -> String {
  let environments = vec![
    "bright", "humid", "dry", "creepy", "ugly", "foggy", "hot", "cold", "drafty", "dreadful",
  ];

  let shapes = vec![
    "round",
    "square",
    "oval",
    "irregular",
    "long",
    "craggy",
    "rough",
    "tall",
    "narrow",
  ];

  let cavern_types = vec![
    "cavern",
    "room",
    "chamber",
    "catacomb",
    "crevasse",
    "cell",
    "tunnel",
    "passageway",
    "hall",
    "expanse",
  ];

  let adornments = vec![
    "smelling of sulphur",
    "with engravings on the walls",
    "with a bumpy floor",
    "",
    "littered with garbage",
    "spattered with guano",
    "with piles of Wumpus droppings",
    "with bones scattered around",
    "with a corpse on the floor",
    "that seems to vibrate",
    "that feels stuffy",
    "that fills you with dread",
  ];

  String::from("A ")
    + choose_name(environments)
    + " "
    + choose_name(shapes)
    + " "
    + choose_name(cavern_types)
    + " "
    + choose_name(adornments)
}

fn choose_name(names: Vec<&str>) -> &str {
  let n = names.len();
  let choice = rand::thread_rng().gen_range(0..n);
  names[choice]
}

fn set_special_caverns(game: &mut Box<dyn HuntTheWumpus>) {
  let player_cavern = game.any_cavern();
  game.set_player_cavern(&player_cavern);
  game.set_wumpus_cavern(&game.any_other(&player_cavern));
  game.add_bat_cavern(&game.any_other(&player_cavern));
  game.add_bat_cavern(&game.any_other(&player_cavern));
  game.add_bat_cavern(&game.any_other(&player_cavern));

  game.add_pit_cavern(&game.any_other(&player_cavern));
  game.add_pit_cavern(&game.any_other(&player_cavern));
  game.add_pit_cavern(&game.any_other(&player_cavern));
}
