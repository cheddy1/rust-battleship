use crate::battleship_game::BOARD_WIDTH;
use crate::battleship_game::BOARD_HEIGHT;
use crate::battleship_game::data_structures::*;
use crate::battleship_game::ship::*;

/// This is the struct that contains the data for each indiviual player
pub struct Player
{
    /// A matrix of size [`BOARD_WIDTH`] x [`BOARD_HEIGHT`] to store which squares were hit and which
    /// ones weren't
    board_matrix: [[ WaterSquare; BOARD_WIDTH ]; BOARD_HEIGHT ],

    /// A vector of the ship objects the player has on their board
    ships_vec: Vec<Ship>,

    /// The signature of the player, mostly used to let the front end know which player achieved a
    /// victory
    player_signature: Players, 

    /// This will be used to detect if an S is printed
    printed_s: bool,
}

#[allow(dead_code, unused_variables, unused_mut)]
impl Player
{
    /// Static function that creates a new player object with the given parameters:
    /// - `ship_count`: the amount of ships that this player should have (currently unused)
    /// - `player_sig`: the signature of the player in the game, so the code can differentiate which
    /// player is which 
    pub fn new_player(ship_count: usize, player_sig: Players) -> Player
    {
        // Create a new WaterBoard object where the matrix is iniaialized to all empty squares
        Player
        {
            // Matrix storing board positions
            board_matrix: [[ WaterSquare::Empty; BOARD_WIDTH ]; BOARD_HEIGHT ],
            
            // New empty vector to be filled out later by using place_ship func
            ships_vec: Vec::new(),
            
            // The signature of the player, mostly for checking the player's turn
            player_signature: player_sig, 

            printed_s: true,
       }
    }

    /// Function to place a ship on this player, the size of the ship is automatically determined
    /// by how many ships are already placed on the board
    /// - `starting_point`: a tuple contiaining the point where the ship should begin
    /// - `is_vertical`: determines whether or not the ship should be vertical, if this is false, the
    /// ship will go to the right from `starting_point`, if it's true, then it will go down
    pub fn place_ship(&mut self, starting_point: (u8, u8), is_vertical: bool)
    {
        self.ships_vec.push(Ship::new_ship(self.ships_vec.len() + 1, starting_point, is_vertical));
    }

    /// Function for being hit on a board, this will be called when it's the other player's turn.
    /// This returns a [`FireState`] so the game knows if it needs to check for sinked ships
    /// - `pos`: a tuple containing the position on the board that we're firing on 
    pub fn fire(&mut self, pos: (u8, u8)) -> FireState
    {
        // Changes the board at the given position to Hit, we shouldn't ever be changing a square
        // from hit to empty so this shouldn't be an issue not having an if
        self.board_matrix[pos.1 as usize][pos.0 as usize] = WaterSquare::Hit;

        // We should then see if a ship has been hit
        return match self.ship_index_at(pos)
        {
            Some(i) =>
            {
                // If we hit some ship, we want to see if it's sunk
                self.ships_vec.get_mut(i).unwrap().update_ship(&self.board_matrix);

                // Then return a hit
                FireState::Hit
            }
            None => FireState::Miss
        }

    }

    /// Function for returning if there's a ship at a particular location, if there's a ship, this
    /// returns `Some(i)` with i being the index of the ship at that location in the vector, if
    /// there's no ship, None is returned instead
    /// - `pos`: the position to check for a ship at
    pub fn ship_index_at(&self, pos: (u8, u8)) -> Option<usize>
    {
        // Enumerate gives us an interator with a two tuple, the first value is the index of
        // iteration, the second value is the item in the vector at the current index
        for (i, ship) in self.ships_vec.iter().enumerate()
        {
           if ship.get_ships().contains(&pos)
           {
               // If this spot has a ship in it, return the ship
               return Some(i);
           }
        }

        // If there is no ship in this spot, return none
        None
    }

    /// This checks if all the ships that this player has on their board have sunk, returns
    /// respective bool value
    pub fn all_ships_sunk(&self) -> bool
    {
        for i in 0..self.ships_vec.len()
        {
            if !self.ships_vec[i].get_sunk()
            {
                return false;
            }
        }
        return true;
    }

    /// Gets the player signature, mostly to know at the end which player has won the game
    pub fn get_sig(&self) -> &Players
    {
        return &self.player_signature;
    }

    /// Returns the number of ships the player has
    pub fn get_ship_count(&self) -> usize
    {
        return self.ships_vec.len();
    }

    /// Returns true if the input coorinate is attacked,
    /// false if it is not
    pub fn is_attacked(&self, row: usize, col: usize) -> bool
    {
        if self.board_matrix[row][col] == WaterSquare::Hit
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

// THIS IS BEING USED FOR CLI
// Debug implementation for printing a player's board
impl Player
{
    /// Prints the vectors of the ships the player has into the command line, mostly for debugging
    pub fn print_ships(&self)
    {
        for ship in self.ships_vec.iter()
        {
            ship.print_ship();
        }
    }

    /// This function checks if there is a ship at the location, and returns true if there is.
    pub fn is_ship(&self, x: usize, y: usize) -> bool
    {
        for i in 0..self.ships_vec.len()
        {
            for j in 0..self.ships_vec[i].get_len()
            {
                if self.ships_vec[i].get_ships()[j] == (x as u8, y as u8)
                {
                    return true;
                }
            }
        }
        return false;
    }

    /// This function to convert ints to chars, to be called when printing the board.
    pub fn char_prints(&self, x: usize) -> char
    {
        match x
        {
            1 => return 'A',
            2 => return 'B',
            3 => return 'C',
            4 => return 'D',
            5 => return 'E',
            6 => return 'F',
            7 => return 'G',
            8 => return 'H',
            9 => return 'I',
            10 => return 'J',
            _ => return 'X',
        }
    }

    /// returns variable
    pub fn printed_s_check(&self) -> bool
    {
        return self.printed_s;
    }

    /// This function prints the player's board.
    /// If the bool passed in is true, it will print where the player's ships are.
    /// If false, it will only print hit locations.
    pub fn print_board(&mut self, is_turn: bool)
    {
        print!("["); // Start the array in the print

        // Iterate through the matrix to print it
        let mut ship = false;
        for i in 0..=BOARD_HEIGHT
        {
            for j in 0..=BOARD_WIDTH
            {
                if j == 0 
                {
                    print!("{} ", i)
                }
                if i == 0 && j != 0
                {
                    print!("{} ", self.char_prints(j))
                }
                // Add an extra space if it isn't the very first character
                if i != 0 && j == 0
                {
                    print!(" ");
                }
                // Match statement to show if the square is either empty or has been hit
                // im so sorry
                
                if i != 0 && j != 0
                {
                    let to_print = match self.board_matrix[i-1][j-1]
                    {
                        WaterSquare::Empty => if is_turn && self.is_ship(j-1,i-1) {'S'} else {' '},
                        WaterSquare::Hit => 'x',
                    };
                    if self.board_matrix[i-1][j-1] == WaterSquare::Empty && is_turn && self.is_ship(i-1, j-1)
                    {
                        ship = true;
                    }
                    // Print the value
                    print!(" {}", to_print);
                }

                if i != (BOARD_HEIGHT) || j != (BOARD_WIDTH)
                {
                    // Add a comma if it's not the very last value
                    print!(", ");
                }

                self.printed_s = ship;
            }

            // Print a new line as long as it's not the last line
            if i != (BOARD_HEIGHT) 
            {
                print!("\n");
            }
        }

        // Print the closing bracket for the array
        print!(" ]\n");
    }
}
