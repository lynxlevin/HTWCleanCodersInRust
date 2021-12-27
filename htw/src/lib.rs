mod commands;
mod connection;
pub mod connections;
pub mod direction;
pub mod english_message_receiver;
pub mod htw_game;
use crate::direction::direction::Direction;
use crate::htw_game::htw_game::Caverns;

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
    fn you_die(&self);
}

pub trait HuntTheWumpus {
    fn set_player_cavern(&mut self, player_cavern: &str);
    fn get_player_cavern(&self) -> &str;
    fn add_bat_cavern(&mut self, cavern: &str);
    fn add_pit_cavern(&mut self, cavern: &str);
    fn set_wumpus_cavern(&mut self, wumpus_cavern: &str);
    fn get_wumpus_cavern(&self) -> &str;
    fn set_quiver(&mut self, arrows: u32);
    fn get_quiver(&self) -> u32;
    fn get_hit_points(&self) -> u32;
    fn check_wumpus_moved_to_player(&self);
    fn execute_command(&mut self);
    fn make_rest_command(&mut self);
    fn make_shoot_command(&mut self, direction: Direction);
    fn make_move_command(&mut self, direction: Direction);
    fn caverns(&self) -> &Caverns;
    fn connect_caverns(&mut self);
}
