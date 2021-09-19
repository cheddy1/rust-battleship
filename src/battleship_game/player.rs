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

#[allow(dead_code, unused_variables, unused_mut)]
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

    pub fn all_ships_sunk(&self) -> bool
    {
        self.ships_vec.iter().all(|ship| ship.get_sunk())
    }

    pub fn get_sig(&self) -> &Players
    {
        return &self.player_signature;
    }

    pub fn get_ship_count(&self) -> usize
    {
        return self.ships_vec.len();
    }

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
    pub fn print_ships(&self)
    {
        for ship in self.ships_vec.iter()
        {
            ship.print_ship();
        }
    }

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

    pub fn print_board(&self, is_turn: bool)
    {
        print!("["); // Start the array in the print

        // Iterate through the matrix to print it
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
                if i != 0 && j != 0
                {
                    let to_print = match self.board_matrix[i-1][j-1]
                    {
                        WaterSquare::Empty => if is_turn && self.is_ship(i-1,j-1) {'S'} else {' '},
                        WaterSquare::Hit => 'x',
                    };
                    // Print the value
                    print!(" {}", to_print);
                }

                

                if i != (BOARD_HEIGHT) || j != (BOARD_WIDTH)
                {
                    // Add a comma if it's not the very last value
                    print!(", ");
                }
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
