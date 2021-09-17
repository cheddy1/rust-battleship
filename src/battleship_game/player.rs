use crate::battleship_game::BOARD_WIDTH;
use crate::battleship_game::BOARD_HEIGHT;
use crate::battleship_game::data_structures::*;
use crate::battleship_game::ship::*;

pub struct Player
{
    board_matrix: [[ WaterSquare; BOARD_WIDTH ]; BOARD_HEIGHT ],
    ships_vec: Vec<Ship>,
    player_signature: Players, 
}

impl Player
{
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
        }
    }

    pub fn place_ship(&mut self, starting_point: (u8, u8), is_vertical: bool)
    {
        self.ships_vec.push(Ship::new_ship(self.ships_vec.len() + 1, starting_point, is_vertical));
    }

    // Function for being hit on a board
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
                //ship.update_ship(&self.board_matrix);

                // Then return a hit
                FireState::Hit
            }
            None => FireState::Miss
        }
    }

    // Function for returning if there's a ship at a particular location
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
}

// Debug implementation for printing a player's board
impl Player
{
    pub fn print_ships(&self)
    {
        for ship in self.ships_vec.iter()
        {
            ship.print_ship();
        }
    }

    pub fn print_board(&self)
    {
        print!("["); // Start the array in the print

        // Iterate through the matrix to print it
        for i in 0..BOARD_HEIGHT
        {
            for j in 0..BOARD_WIDTH
            {
                // Match statement to show if the square is either empty or has been hit
                let to_print = match self.board_matrix[i][j]
                {
                    WaterSquare::Empty => 'e',
                    WaterSquare::Hit => 'x',
                };

                // Add an extra space if it isn't the very first character
                if i != 0 && j == 0
                {
                    print!(" ");
                }

                // Print the value
                print!(" {}", to_print);

                if i != (BOARD_HEIGHT - 1) || j != (BOARD_WIDTH - 1)
                {
                    // Add a comma if it's not the very last value
                    print!(", ");
                }
            }

            // Print a new line as long as it's not the last line
            if i != (BOARD_HEIGHT - 1) 
            {
                print!("\n");
            }
        }

        // Print the closing bracket for the array
        print!(" ]\n");
    }
}
