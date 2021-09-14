use crate::battleship_game::BOARD_SIZE;
use crate::battleship_game::data_structures::*;

pub struct Player
{
    board_matrix: [[ WaterSquare; BOARD_SIZE ]; BOARD_SIZE ],
}

impl Player
{
    pub fn new_player() -> Player
    {
        // Create a new WaterBoard object where the matrix is iniaialized to all empty squares
        Player { board_matrix: [[ WaterSquare::Empty; 8 ]; 8 ] }
    }
    
    pub fn print_board(&self)
    {
        print!("["); // Start the array in the print

        // Iterate through the matrix to print it
        for i in 0..BOARD_SIZE
        {
            for j in 0..BOARD_SIZE
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

                if i != (BOARD_SIZE - 1) || j != (BOARD_SIZE - 1)
                {
                    // Add a comma if it's not the very last value
                    print!(", ");
                }
            }

            // Print a new line as long as it's not the last line
            if i != (BOARD_SIZE - 1) 
            {
                print!("\n");
            }
        }

        // Print the closing bracket for the array
        print!(" ]\n");
    }

}
