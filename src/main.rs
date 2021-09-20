mod battleship_game;

use crate::battleship_game::{model::Model, view::init_game};
use std::rc::Rc;
use std::cell::RefCell;

fn main()
{
    let model = Rc::new(RefCell::new(Model::new()));
    init_game(&model);
}
