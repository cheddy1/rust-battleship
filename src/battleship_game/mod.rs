// Import the sub-modules
mod data_structures;
mod player;
mod ship;
pub mod model;
pub mod view;

// Import the battleship_game module from the root of the crate
use crate::battleship_game::player::*;
use crate::battleship_game::data_structures::*;

// Size constants
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 9;

/// This is the main game logic handler, it holds both play objects, and therefore both boards and
/// all the ships on the board
pub struct BattleShipGame
{
    /// The object that holds the first player
    player_one: Player, 

    /// The object that holds the second player
    player_two: Player,

    /// This variable keeps track of whose turn it is
    is_player_one_turn: bool,

    /// This stores how many ship each player should have on their board
    ship_count_pp: usize, // Ship count per player
}

impl BattleShipGame
{
    /// This is the method you should use to make a new game object, all it needs is:
    /// - `ship_count`: the amount of ships each player should have on their board
    pub fn new(ship_count: usize) -> Self
    {
        BattleShipGame
        {
            player_one: Player::new_player(ship_count, Players::PlayerOne),
            player_two: Player::new_player(ship_count, Players::PlayerTwo),
            is_player_one_turn: true,
            ship_count_pp: ship_count,
        }
    }

    /// This function will execute the turn for the current player, and will return both whether a
    /// ship was hit or not, as well as if there's a winner after this turn
    /// - `pos`: a two tuple holding the position we're taking the turn at
    pub fn take_turn(&mut self, pos: (u8, u8)) -> (FireState, Option<&Players>)
    {
        // This is gross, I understand that I've copy pasted code and it's not something you're
        // supposed to do, but I tried using a pointer to point to the current player but rust went
        // WAAAAAAAAAAAAAAAAA MUTABLE VARIABLES WAAAAAAAAAAAAA I'M A GAY BABY WAAAAAAAAAAAAA
        if self.is_player_one_turn 
        {
            // First we fire on the other player's board
            let hit_or_miss = self.player_two.fire(pos);

            // Invert the member variable dictating whose turn it is
            self.is_player_one_turn = !self.is_player_one_turn;

            // Then we return either a Some is victory is achieved, or a None if it hasn't
            let vic = if self.check_victory() { Some(self.player_one.get_sig()) } else { None };

            (hit_or_miss, vic)
        }
        else
        {
            // Same idea here
            let hit_or_miss = self.player_one.fire(pos);
            self.is_player_one_turn = !self.is_player_one_turn;
            let vic = if self.check_victory() { Some(self.player_two.get_sig()) } else { None };
            (hit_or_miss, vic)
        }
    }    

    /// This is a private function that will check to see if there's been a victory from either
    /// player
    fn check_victory(&self) -> bool
    {
       // We invert the self.is_player_one_turn var right before this, so this will look a little
       // backwards
       if self.is_player_one_turn
       {
           // If this is true, that means it's really player two's turn when this runs, so to check
           // victory we need to check player one's board
           self.player_one.all_ships_sunk()
       }
       else
       {
           self.player_two.all_ships_sunk()
       }
    }

    /// This is the top level function to place a ship, this places a ship on the players board, if
    /// the game has X boards per player, then after X board this function switches back to the
    /// next player. If for some reason X ships are placed for player one, then X for player two,
    /// anymore will be placed on the board for player one
    pub fn place_a_ship(&mut self, pos: (u8, u8), is_vertical: bool)
    {
        // First thing we want to do is place a ship on the current player
        if self.is_player_one_turn
        {
            self.player_one.place_ship(pos, is_vertical);
            
            // After we place the ship, we want to check and make sure we've placed the right amount of
            // ships on that players board, so we know when to swap to the next player
            if self.player_one.get_ship_count() >= self.ship_count_pp
            {
                self.is_player_one_turn = !self.is_player_one_turn;
            }

        }
        else
        {
            self.player_two.place_ship(pos, is_vertical);

            if self.player_two.get_ship_count() >= self.ship_count_pp
            {
                self.is_player_one_turn = !self.is_player_one_turn;
            }
        }
    }
}

// Debug block
impl BattleShipGame
{
    pub fn print_p1_board(&self)
    {
        self.player_one.print_board();
    }

    pub fn print_p1_ships(&self)
    {
        self.player_one.print_ships();
    }

    pub fn test(&mut self)
    {
        // P1 ships
        self.place_a_ship((2, 2), false);
        self.place_a_ship((0, 0), true);

        // P2 ships
        self.place_a_ship((1, 1), false);
        self.place_a_ship((3, 3), true);

        // Alternate player turns, starting at p1
        self.turn_debug((0, 0));
        self.turn_debug((0, 0));
        self.turn_debug((5, 5));
        self.turn_debug((2, 2));
        self.turn_debug((3, 3));
        self.turn_debug((0, 1));
    }

    pub fn turn_debug(&mut self, pos: (u8, u8))
    {
        let (hit, vic) = self.take_turn(pos);

        println!("{}, turn for {} at position ({}, {}) resulted in a {}",
        match vic
        {
            Some(_) => "over",
            None => "playing",
        },
        match &mut self.is_player_one_turn
        {
            true => "P1",
            false => "P2",
        }, pos.0, pos.1,
        match hit
        {
            FireState::Miss => "miss",
            FireState::Hit => "hit",
        });
    }
}
