// Import the sub-modules
mod data_structures;
mod debug;
mod player;
mod ship;

// Import the battleship_game module from the root of the crate
use crate::battleship_game::player::*;

const BOARD_SIZE: usize = 8;

pub struct BattleShipGame
{
    player_one: Player, 
    player_two: Player,
}
