use fltk::frame::Frame;
use std::rc::Rc;
use std::cell::RefCell;

// Now we need to keep track of whether our boats are being moved on the screen
// or not.
pub enum BoatState
{
    Placed,
    Moving(Rc<RefCell<Frame>>, i32, i32),
}

pub struct Model
{
    pub game_boat_state: BoatState,
}

impl Model
{
    pub fn new() -> Self
    {
        Self
        {
            game_boat_state: BoatState::Placed,
        }
    }

}
