use crate::battleship_game::BOARD_WIDTH;
use crate::battleship_game::BOARD_HEIGHT;
use crate::battleship_game::data_structures::*;

/// This is the struct that contains the data for a Ship on the board
pub struct Ship
{
    // This is subject to change, but I think the position of the ships
    // should be stored in a vector of two tuples, each ship will take different
    // amount of spots, which is why it's a vector and not an array
    /// A vector that stores the coordinate of each piece of the ship, since the ships can vary in
    /// size, we decided to use a vector instead of an array
    ship_position: Vec<(u8, u8)>,

    /// Holds the sunk status of the ship, if the enemy has hit each position where this particular
    /// ship, then this ship counts as sunk
    is_sunk: bool,
}

impl Ship
{
    /// This method places a new ship at a particular location on the board with the given
    /// parameters:
    /// - `ship_size`: the size of the ship that we're placing
    /// - `start_point`: the start point of the ship on the board
    /// - `is_vertical`: whether or not the ship should be placed vertically on the board, if this
    /// is true, the ship is placed up from `start_point`, if it's false, it's placed to the right
    pub fn new_ship(ship_size: usize, start_point: (u8, u8), is_vertical: bool) -> Ship
    {
        let mut pos_vec = Vec::new();

        // For loop to set ship_position coordinates based off direction
        for i in 0..ship_size
        {
            // Creates a new tuple adding i in the correct direction based off of specified ship size and direction
            let new_point = (start_point.0 + if is_vertical { i as u8 } else { 0 },
                             start_point.1 + if is_vertical { 0 } else { i as u8 });

            pos_vec.push(new_point); 
        }

        Ship
        {
            ship_position: pos_vec, 
            is_sunk: false,  
        }
    } 

    /// Returns a reference to the [`Vec`](vector) of the positions that this ship is at
    pub fn get_ships(&self) -> &Vec<(u8, u8)>
    {
        // Return a reference to the ship position Vector
        &self.ship_position
    }

    /// Updates the sunk status of the ship, but to do this we need:
    /// - `board`: a reference to the board matrix, size same as the constants defined in the
    /// [`battleship_game`] module
    pub fn update_ship(&mut self, board: &[[ WaterSquare; BOARD_WIDTH ]; BOARD_HEIGHT ])
    {
        // This one liner makes me happy
        // If all board positions that this ship takes up are hit, then we set is_sunk to true
        self.is_sunk = self.ship_position.iter().all(|pos| board[pos.1 as usize][pos.0 as usize] == WaterSquare::Hit);
        
        if self.is_sunk
        {
            // TODO: Remove this debug
            println!("YOU SUNK ME WAAAAAA");
        }
    }

    /// Returns the sunk status of this ship
    pub fn get_sunk(&self) -> bool
    {
        self.is_sunk
    }

    /// Returns the length of the ship
    pub fn get_len(&self) -> usize
    {
        return self.ship_position.len();
    }
}


impl Ship
{
    /// Prints the vector of ships out in an array style
    pub fn print_ship(&self)
    {
        print!("[ ");

        for (i, pos) in self.ship_position.iter().enumerate()
        {
            print!("({}, {})", pos.0, pos.1);

            if i != self.ship_position.len() - 1
            {
                print!(", ");
            }
        }

        print!(" ]\n");
    }
}
