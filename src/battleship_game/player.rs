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
            board_matrix: [[ WaterSquare::Empty; BOARD_WIDTH ]; BOARD_HEIGHT ],
            ships_vec: Vec::new(),
            player_signature: player_sig, 
        }
    }

    pub fn place_ship(&mut self, starting_point: (u8, u8), is_vertical: bool)
    {
        self.ships_vec.push(Ship::new_ship(self.ships_vec.len() + 1, starting_point, is_vertical));
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
