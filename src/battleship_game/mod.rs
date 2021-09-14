// Import the sub-modules
mod data_structures;
mod player;
mod ship;

// Import the battleship_game module from the root of the crate
use crate::battleship_game::player::*;
use crate::battleship_game::data_structures::*;

const BOARD_SIZE: usize = 8;

pub struct BattleShipGame
{
    player_one: Player, 
    player_two: Player,
}

impl BattleShipGame
{
    pub fn init_game() -> BattleShipGame
    {
        BattleShipGame
        {
            player_one: Player::new_player(Players::PlayerOne),
            player_two: Player::new_player(Players::PlayerTwo),
        }
    }
}
