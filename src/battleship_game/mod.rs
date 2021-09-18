// Import the sub-modules
mod data_structures;
mod player;
mod ship;

// Import the battleship_game module from the root of the crate
use crate::battleship_game::player::*;
use crate::battleship_game::data_structures::*;

// Import FLTK modules for GUI
use fltk::
{
    app, 
    button::Button, 
    frame::Frame, 
    text::TextDisplay, 
    enums::FrameType, 
    enums::Color, 
    enums::Align, 
    prelude::*, 
    window::Window, 
    group::Group, 
    widget_extends
};

// Standard Library stuff
use std::str;

// Size constants
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 9;
const SQUARE_SIZE: i32 = 50;
const NUM_OFFSET: i32 = 50;

const P1_BOARD_LEFT_OFFSET: i32 = 0;
const P2_BOARD_LEFT_OFFSET: i32 = 850;
const BOARD_TOP_OFFSET: i32 = 50;

// We need to draw two boards on the screen. In order to avoid
// repeating the same code to draw two boards, we group up all of the
// widgets that make up a board into a Board widget. That way, we just
// have to place two boards on the screen.
struct Board
{
    grp: Group,
}

impl Board
{
    pub fn new(x: i32, y: i32) -> Self
    {
        // The constructor of Board is where we arrange all of the
        // coordinate labels and squares.
        let mut grp = Group::new(
            x, 
            y, 
            BOARD_WIDTH as i32 * SQUARE_SIZE + NUM_OFFSET, 
            BOARD_HEIGHT as i32 * SQUARE_SIZE + NUM_OFFSET, 
            None,
        );
        grp.set_frame(FrameType::FlatBox);

        for (i, c) in ('A'..='J').enumerate() // (0, 'A'), (1, 'B'),...(9, 'J')
        {
            // Column Labels
            let mut board_col_label = Frame::new(
                i as i32 * SQUARE_SIZE + grp.x() + NUM_OFFSET, 
                grp.y(), 
                SQUARE_SIZE, 
                SQUARE_SIZE, 
                "",
            );
            board_col_label.set_label(str::from_utf8(&[c as u8]).unwrap());
            board_col_label.set_frame(FrameType::FlatBox);
        }

        for j in 0..BOARD_HEIGHT
        {
            let mut board_num_label = Frame::new(
                grp.x(), 
                j as i32 * SQUARE_SIZE + grp.y() + NUM_OFFSET, 
                SQUARE_SIZE, 
                SQUARE_SIZE, 
                "",
            );
            board_num_label.set_label(&(j + 1).to_string());
            board_num_label.set_frame(FrameType::FlatBox);

            for i in 0..BOARD_WIDTH
            {
                // Build board.
                let mut temp_btn = Button::new(
                    i as i32 * SQUARE_SIZE + grp.x() + NUM_OFFSET, 
                    j as i32 * SQUARE_SIZE + grp.y() + NUM_OFFSET, 
                    SQUARE_SIZE, 
                    SQUARE_SIZE, 
                    "~",
                );

                temp_btn.set_frame(FrameType::BorderBox);
                temp_btn.set_color(Color::from_u32(0x48769C));
                temp_btn.set_selection_color(Color::from_u32(0x1a2b38));

                // Custom function can replace println in future!
                temp_btn.set_callback(move |btn| {
                    println!("Player 1 {},{}", i, j);
                    btn.set_color(Color::from_u32(0x00000A));
                });
            }
        }
        // Create new frame for a container that holds player 1's ships
        let end_of_squares = grp.x() + NUM_OFFSET + (SQUARE_SIZE * BOARD_WIDTH as i32);
        let top_of_squares = grp.y() + NUM_OFFSET;
        let mut ship_container = Frame::new(end_of_squares, top_of_squares, 300, 450, "");
        ship_container.set_frame(FrameType::BorderBox);
        ship_container.set_color(Color::from_u32(0x455766));

        grp.end();

        Self { grp } // In Rust, you can return an expression by simply writing
        // it as the last line of your function without a semicolon. It's just
        // a concise way of returning things without having to type the word
        // "return". Here, our constructor returns the object we just initialized.
    }
}

// Normally we would subclass FL_Group (https://www.fltk.org/doc-1.1/Fl_Group.html)
// in order to define our own widget as a collection of widgets.
// However, Rust doesn't really have subclasses; it has Traits, which are analogous to 
// Classes in Haskell or Interfaces in Java/C++. Anyway, the following line can be 
// viewed as an alternative to subclassing.
widget_extends!(Board, Group, grp);

// Now we need to keep track of whether our boats are being moved on the screen
// or not.
enum BoatState
{
    Placed,
    Moving(Frame),
}

pub struct BattleShipGame
{
    player_one: Player, 
    player_two: Player,
    game_boat_state: BoatState,
}

impl BattleShipGame
{
    pub fn new(ship_count: usize) -> Self
    {
        BattleShipGame
        {
            player_one: Player::new_player(ship_count, Players::PlayerOne),
            player_two: Player::new_player(ship_count, Players::PlayerTwo),
            game_boat_state: BoatState::Placed,
        }
    }
    pub fn init_game(&self)
    {
        // I dont know where to put this window creation logic, so it can live in init_game for now.
        let app = app::App::default();
        let mut wind = Window::new(100, 100, 1750, 600, "💥 Rust Battleship 💥");

        // Track the buttons with 2d array (broken atm)
        //let mut p1_button_ary = [[Button::default(); 9]; 10];
        //let mut p2_button_ary = [[Button::default(); 9]; 10];

        // TODO: Move to separate module

        let mut p1_board = Board::new(P1_BOARD_LEFT_OFFSET, BOARD_TOP_OFFSET);
        let mut p2_board = Board::new(P2_BOARD_LEFT_OFFSET, BOARD_TOP_OFFSET);
        // Test ship graphics for Player 1
        for n in (0..6).rev()
        {
            // 5px was added to the y pos, and 10px taken off the hight of the ship for vertical padding reasons. 
            let mut dummy_ship = Frame::new(350, 55 + ((n + 1) * SQUARE_SIZE), (n + 1) * (SQUARE_SIZE ), (SQUARE_SIZE - 10), "");
            dummy_ship.set_frame(FrameType::OvalBox);
            dummy_ship.set_color(Color::from_u32(0xE9ECF0));
            dummy_ship.set_callback(|s| {
                match self.game_boat_state
                {

                }
            });
        }


        // This is a concept of drawing a box to the screen. We can keep it for now as a backup.
        // let mut screen_blocker1 = fltk::draw::draw_box(FrameType::FlatBox, 0, 0, 1750, 600, Color::from_u32(0x00000A));

        // Button that covers the screen between turns.
        let mut screen_blocker = Button::new(0, 0, 1750, 600, "Click here to start your turn.");

        screen_blocker.set_frame(FrameType::FlatBox);
        screen_blocker.set_color(Color::from_u32(0xC0C0C0));
        screen_blocker.hide();

        // Button that indicates a user is finished looking a the board after a turn.
        let mut cont_btn = Button::new( 1600, 555, 100, 40, "Continue");

        cont_btn.set_frame(FrameType::ThinUpBox);
        cont_btn.set_color(Color::from_u32(0x4C9C48));
        cont_btn.set_selection_color(Color::from_u32(0x397536));

        // Hide the screen blocker when clicked.
        screen_blocker.set_callback(move |btn| {
            println!("Player 1");
            btn.hide();
        });

        // Call back for the continue button that blocks the screen with screen_blocker.
        cont_btn.set_callback(move |btn| {
            println!("Player 1");
            screen_blocker.show();
            // btn.hide() // possibly hide or disable when the screen_blocker is shown?
        });

        
        // Cool alert... we can delete once we know we dont need it.
        // fltk::dialog::alert(
        //     875, 
        //     300, 
        //     "Dissmiss to start your turn."
        // );

        // Dont know how to set a callback for this yet, or require valid input.
        let mut ship_input = fltk::dialog::input(
            875, 
            300, 
            "How many ships do you want to play with?",
            ""
        );


        wind.end();
        wind.show();
        app.run().unwrap();
        
    }

    pub fn print_p1_board(&self)
    {
        self.player_one.print_board();
    }

    pub fn print_p1_ships(&self)
    {
        self.player_one.print_ships();
    }

    pub fn test(&mut self)
    {
        self.player_one.place_ship((2, 2), false);
        self.player_one.place_ship((0, 0), true);

        // TODO: Test a move
        self.player_one.fire((0, 3));
        self.player_one.fire((2, 3));
        self.player_one.fire((0, 0));
        self.player_one.fire((0, 1));
        self.player_one.fire((2, 2));
    }
}
