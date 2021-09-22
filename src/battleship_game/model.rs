use fltk::frame::Frame;

// Now we need to keep track of whether our boats are being moved on the screen
// or not.
pub enum BoatState<'a>
{
    Placed,
    Moving(&'a mut Frame),
}

pub struct Model<'a>
{
    pub game_boat_state: BoatState<'a>,
}

impl<'a> Model<'a>
{
    pub fn new() -> Self
    {
        Self
        {
            game_boat_state: BoatState::Placed,
        }
    }

}
