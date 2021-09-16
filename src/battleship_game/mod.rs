// Import the sub-modules
mod data_structures;
mod player;
mod ship;

// Import the battleship_game module from the root of the crate
use crate::battleship_game::player::*;
use crate::battleship_game::data_structures::*;

// Import FLTK modules for GUI
use fltk::{app, button::Button, frame::Frame, enums::FrameType, enums::Color, prelude::*, window::Window};


const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 9;
const SQUARE_SIZE: i32 = 50;

const P1_BOARD_LEFT_OFFSET: i32 = 50;
const P2_BOARD_LEFT_OFFSET: i32 = 750;
const BOARD_TOP_OFFSET: i32 = 100;

pub struct BattleShipGame
{
    player_one: Player, 
    player_two: Player,
}

impl BattleShipGame
{
    pub fn init_game(ship_count: usize) -> BattleShipGame
    {
        // I dont know where to put this window creation logic, so it can live in init_game for now.
        let app = app::App::default();
        let mut wind = Window::new(100, 100, 1450, 600, "💥 Rust Battleship 💥");

        // Track the buttons with 2d array (broken atm)
        //let mut p1_button_ary = [[Button::default(); 9]; 10];
        //let mut p2_button_ary = [[Button::default(); 9]; 10];


        for j in 0..BOARD_HEIGHT
        {
            for i in 0..BOARD_WIDTH
            {
                // Build player 1's board.
                let mut temp_btn = Button::new(P1_BOARD_LEFT_OFFSET + (i as i32 * SQUARE_SIZE),
                                               BOARD_TOP_OFFSET + (j as i32 * SQUARE_SIZE), SQUARE_SIZE, SQUARE_SIZE, "B");

                temp_btn.set_frame(FrameType::BorderBox);
                temp_btn.set_color(Color::from_u32(0x48769C));
                temp_btn.set_selection_color(Color::from_u32(0x1a2b38));

                // Custom function can replace println in future!
                temp_btn.set_callback(move |_| println!("Player 1 {},{}", i, j));

                // 2d array currently broken.
                // p1_buttons[i][j] = temp_btn; // broken


                // Build player 2's board.
                let mut temp_btn2 = Button::new(P2_BOARD_LEFT_OFFSET + (i as i32 * SQUARE_SIZE),
                                                BOARD_TOP_OFFSET + (j as i32 * SQUARE_SIZE), SQUARE_SIZE, SQUARE_SIZE, "B");

                temp_btn2.set_frame(FrameType::BorderBox);
                temp_btn2.set_color(Color::from_u32(0x48769C));
                temp_btn2.set_selection_color(Color::from_u32(0x1a2b38));

                // Custom function can replace println in future!
                temp_btn2.set_callback(move |_| println!("Player 2 {},{}", i, j));

                // 2d array currently broken.
                // p2_button_ary[i][j] = temp_btn2; // broken
            }
        }

        wind.end();
        wind.show();
        app.run().unwrap();

        BattleShipGame
        {
            player_one: Player::new_player(ship_count, Players::PlayerOne),
            player_two: Player::new_player(ship_count, Players::PlayerTwo),
        }
    }

    pub fn print_p1_board(&self)
    {
        self.player_one.print_board();
    }

    pub fn print_p1_ships(&self)
    {
        self.player_one.print_ships();
    }
}
