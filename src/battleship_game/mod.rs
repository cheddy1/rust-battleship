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

pub struct BattleShipGame
{
    player_one: Player, 
    player_two: Player,
    is_player_one_turn: bool,
}

impl BattleShipGame
{
    pub fn init_game(ship_count: usize) -> BattleShipGame
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
        screen_blocker.set_callback(move |btn|
        {
            println!("Player 1");
            btn.hide();
        });

        // Call back for the continue button that blocks the screen with screen_blocker.
        cont_btn.set_callback(move |btn|
        {
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

        BattleShipGame
        {
            player_one: Player::new_player(ship_count, Players::PlayerOne),
            player_two: Player::new_player(ship_count, Players::PlayerTwo),
            is_player_one_turn: true,
        }
        
    }

    // This function will execute the turn for the current player, and will return a bool of
    // whether or not the game has finished
    pub fn take_turn(&mut self, pos: (u8, u8)) -> Option<&Players>
    {
        // This is gross, I understand that I've copy pasted code and it's not something you're
        // supposed to do, but I tried using a pointer to point to the current player but rust went
        // WAAAAAAAAAAAAAAAAA MUTABLE VARIABLES WAAAAAAAAAAAAA I'M A GAY BABY WAAAAAAAAAAAAA
        if self.is_player_one_turn 
        {
            // First we fire on the other player's board
            self.player_two.fire(pos);

            // Invert the member variable dictating whose turn it is
            self.is_player_one_turn = !self.is_player_one_turn;

            // Then we return either a Some is victory is achieved, or a None if it hasn't
            if self.check_victory() { Some(self.player_one.get_sig()) } else { None }
        }
        else
        {
            // Same idea here
            self.player_one.fire(pos);
            self.is_player_one_turn = !self.is_player_one_turn;
            if self.check_victory() { Some(self.player_two.get_sig()) } else { None }  
        }
    }    

    fn check_victory(&self) -> bool
    {
       // We invert the self.is_player_one_turn var right before this, so this will look a little
       // backwards
       if self.is_player_one_turn
       {
           // If this is true, that means it's really player two's turn when this runs, so to check
           // victory we need to check player one's board
           self.player_one.all_ships_sunk()
       }
       else
       {
           self.player_two.all_ships_sunk()
       }
    }
}

// Debug block
impl BattleShipGame
{
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
