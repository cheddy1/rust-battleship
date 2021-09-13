// This imports the module from the file data_structues.rs in the water_board directory
mod data_structures;

// This tells rust that we want to use everything from the data_structues module in this scope
use data_structures::*;

pub struct WaterBoard
{
    // Battleship uses an 8x8 grid, so we're going to make a 2D array of WaterSquares
    board_matrix: [[ WaterSquare; 8 ]; 8 ]
}

impl WaterBoard
{
    pub fn new_board() -> WaterBoard
    {
        // Create a new WaterBoard object where the matrix is iniaialized to all empty squares
        WaterBoard { board_matrix: [[ WaterSquare::Empty; 8 ]; 8 ] }
    }

    pub fn print_board(&self)
    {
        print!("[");

        for i in 0..8
        {
            for j in 0..8
            {
                let to_print = match self.board_matrix[i][j]
                {
                    WaterSquare::Empty => 'e',
                    WaterSquare::Hit => 'x',
                };

                if i != 0 && j == 0
                {
                    print!(" ");
                }

                print!(" {}", to_print);

                if i != 7 || j != 7
                {
                    print!(", ");
                }
            }

            if i != 7
            {
                print!("\n");
            }
        }
        print!(" ]\n");
    }
}
