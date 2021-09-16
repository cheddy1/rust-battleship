use crate::battleship_game::data_structures::*;

pub struct Ship
{
   // This is subject to change, but I think the position of the ships
   // should be stored in a vector of two tuples, each ship will take different
   // amount of spots, which is why it's a vector and not an array
   ship_position: Vec<(u8, u8)>,

   is_sunk: bool,
}

impl Ship
{
    pub fn new_ship(ship_size: usize, start_point: (u8, u8), is_vertical: bool) -> Ship
    {
        let mut pos_vec = Vec::new();


        // TODO: check ship size

        // For loop to set ship_position coordinates based off direction
        for i in 0..ship_size
        {
            // Creates a new tuple adding i in the correct direction based off of specified ship size and direction
            let new_point = (start_point.0 + if is_vertical { 0 } else { i as u8 },
                             start_point.1 + if is_vertical { i as u8 } else { 0 });

            pos_vec.push(new_point); 
        }

        Ship
        {
            ship_position: pos_vec, 
            is_sunk: false,  
        }
    } 

    pub fn get_ships(&self) -> Vec<(u8, u8)>
    {
        self.ship_position
    }
}


impl Ship
{
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
