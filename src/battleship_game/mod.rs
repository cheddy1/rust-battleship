// Import the sub-modules
mod data_structures;
mod player;
mod ship;

// Import the battleship_game module from the root of the crate
use crate::battleship_game::player::*;
use crate::battleship_game::data_structures::*;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 9;

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

    pub fn print_p1_board(&self)
    {
        self.player_one.print_board();
    }
}
