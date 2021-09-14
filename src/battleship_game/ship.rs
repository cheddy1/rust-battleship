use crate::battleship_game::data_structures::*;

pub struct Ship
{
   // This is subject to change, but I think the position of the ships
   // should be stored in a vector of two tuples, each ship will take different
   // amount of spots, which is why it's a vector and not an array
   ship_position: Vec<(u8, u8)>,

   is_sunk: bool,
}
