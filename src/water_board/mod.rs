// This imports the module from the file data_structues.rs in the water_board directory
mod data_structures;
mod debug;

// This tells rust that we want to use everything from the data_structues module in this scope
use data_structures::*;
use debug::*;

const BOARD_SIZE: usize = 8;

pub struct WaterBoard
{
    // Battleship uses an 8x8 grid, so we're going to make a 2D array of WaterSquares
    board_matrix: [[ WaterSquare; BOARD_SIZE ]; BOARD_SIZE ]
}

impl WaterBoard
{
    pub fn new_board() -> WaterBoard
    {
        // Create a new WaterBoard object where the matrix is iniaialized to all empty squares
        WaterBoard { board_matrix: [[ WaterSquare::Empty; 8 ]; 8 ] }
    }
}
