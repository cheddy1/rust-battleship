// uses
use std::io;

// Import the sub-modules
mod data_structures;
mod player;
mod ship;

// Import the battleship_game module from the root of the crate
use crate::battleship_game::player::*;
use crate::battleship_game::data_structures::*;

// // Import FLTK modules for GUI
// #[allow(unused_imports)]
// use fltk::
// {
//     app, 
//     button::Button, 
//     frame::Frame, 
//     text::TextDisplay, 
//     enums::FrameType, 
//     enums::Color, 
//     enums::Align, 
//     prelude::*, 
//     window::Window, 
//     group::Group, 
//     widget_extends
// };

// // Standard Library stuff
//use std::str;

// // Size constants
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 9;
// const SQUARE_SIZE: i32 = 50;
// const NUM_OFFSET: i32 = 50;

// const P1_BOARD_LEFT_OFFSET: i32 = 0;
// const P2_BOARD_LEFT_OFFSET: i32 = 850;
// const BOARD_TOP_OFFSET: i32 = 50;

// We need to draw two boards on the screen. In order to avoid
// repeating the same code to draw two boards, we group up all of the
// widgets that make up a board into a Board widget. That way, we just
// have to place two boards on the screen.

//Creates a struct with a property of type Group
// struct Board
// {
//     grp: Group,
// }

// //First implementation of our board
// impl Board
// {
//     pub fn new(x: i32, y: i32) -> Self
//     {
//         // The constructor of Board is where we arrange all of the
//         // coordinate labels and squares.
//         let mut grp = Group::new(
//             x, 
//             y, 
//             BOARD_WIDTH as i32 * SQUARE_SIZE + NUM_OFFSET, 
//             BOARD_HEIGHT as i32 * SQUARE_SIZE + NUM_OFFSET, 
//             None,
//         );
//         //Sets squares' frame to a FlatBox type
//         grp.set_frame(FrameType::FlatBox);

//         //For loop to set up the dimensions of the board using integers to correspond to its incrementing 
//         //letter in the alphabet
//         for (i, c) in ('A'..='J').enumerate() // (0, 'A'), (1, 'B'),...(9, 'J')
//         {
//             // Column Labels
//             let mut board_col_label = Frame::new(
//                 i as i32 * SQUARE_SIZE + grp.x() + NUM_OFFSET, 
//                 grp.y(), 
//                 SQUARE_SIZE, 
//                 SQUARE_SIZE, 
//                 "",
//             );
//             //Displaying column labels
//             board_col_label.set_label(str::from_utf8(&[c as u8]).unwrap());
//             //Sets board's column label's frame to FlatBox type
//             board_col_label.set_frame(FrameType::FlatBox);
//         }

//         //for loop from 0 to the height of the board
//         for j in 0..BOARD_HEIGHT
//         {
//             //Row labels
//             let mut board_num_label = Frame::new(
//                 grp.x(), 
//                 j as i32 * SQUARE_SIZE + grp.y() + NUM_OFFSET, 
//                 SQUARE_SIZE, 
//                 SQUARE_SIZE, 
//                 "",
//             );
//             //Risplaying row labels
//             board_num_label.set_label(&(j + 1).to_string());
//             //Sets board's row label's frame to FlatBox type
//             board_num_label.set_frame(FrameType::FlatBox);

//             //for loop from 0 to the width of the board
//             for i in 0..BOARD_WIDTH
//             {
//                 // Build board.
//                 let mut temp_btn = Button::new(
//                     i as i32 * SQUARE_SIZE + grp.x() + NUM_OFFSET, 
//                     j as i32 * SQUARE_SIZE + grp.y() + NUM_OFFSET, 
//                     SQUARE_SIZE, 
//                     SQUARE_SIZE, 
//                     "~",
//                 );
//                 //Sets boards frame to type BorderBox
//                 temp_btn.set_frame(FrameType::BorderBox);
//                 //Sets the board's color
//                 temp_btn.set_color(Color::from_u32(0x48769C));
//                 //Sets the selection color
//                 temp_btn.set_selection_color(Color::from_u32(0x1a2b38));

//                 //Prints player 1's move
//                 temp_btn.set_callback(move |btn| {
//                     println!("Player 1 {},{}", i, j);
//                     btn.set_color(Color::from_u32(0x00000A));
//                 });
//             }
//         }
//         // Create new frame for a container that holds player 1's ships
//         let end_of_squares = grp.x() + NUM_OFFSET + (SQUARE_SIZE * BOARD_WIDTH as i32);
//         let top_of_squares = grp.y() + NUM_OFFSET;
//         let mut ship_container = Frame::new(end_of_squares, top_of_squares, 300, 450, "");
//         ship_container.set_frame(FrameType::BorderBox);
//         ship_container.set_color(Color::from_u32(0x455766));

//         grp.end();

//         Self { grp } // In Rust, you can return an expression by simply writing
//         // it as the last line of your function without a semicolon. It's just
//         // a concise way of returning things without having to type the word
//         // "return". Here, our constructor returns the object we just initialized.
//     }
// }

// Normally we would subclass FL_Group (https://www.fltk.org/doc-1.1/Fl_Group.html)
// in order to define our own widget as a collection of widgets.
// However, Rust doesn't really have subclasses; it has Traits, which are analogous to 
// Classes in Haskell or Interfaces in Java/C++. Anyway, the following line can be 
// viewed as an alternative to subclassing.
// widget_extends!(Board, Group, grp);

/// This is the main game logic handler, it holds both play objects, and therefore both boards and
/// all the ships on the board
pub struct BattleShipGame
{
    /// The object that holds the first player
    player_one: Player, 

    /// The object that holds the second player
    player_two: Player,

    /// This variable keeps track of whose turn it is
    is_player_one_turn: bool,
    ship_count: usize,
    is_p2_ai: bool,
}

#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_assignments, unused_must_use)]
impl BattleShipGame
{
    /// This is the method you should use to make a new game object, all it needs is:
    /// - `ship_count`: the amount of ships each player should have on their board
    // pub fn init_game(ship_count: usize) -> BattleShipGame
    // {
    //     // I dont know where to put this window creation logic, so it can live in init_game for now.
    //     let app = app::App::default();
    //     let mut wind = Window::new(100, 100, 1750, 600, "💥 Rust Battleship 💥");

    //     // Track the buttons with 2d array (broken atm)
    //     //let mut p1_button_ary = [[Button::default(); 9]; 10];
    //     //let mut p2_button_ary = [[Button::default(); 9]; 10];

    //     // TODO: Move to separate module

    //     let mut p1_board = Board::new(P1_BOARD_LEFT_OFFSET, BOARD_TOP_OFFSET);
    //     let mut p2_board = Board::new(P2_BOARD_LEFT_OFFSET, BOARD_TOP_OFFSET);
    //     // Test ship graphics for Player 1
    //     for n in (0..6).rev()
    //     {
    //         // 5px was added to the y pos, and 10px taken off the hight of the ship for vertical padding reasons. 
    //         let mut dummy_ship = Frame::new(350, 55 + ((n + 1) * SQUARE_SIZE), (n + 1) * (SQUARE_SIZE ), (SQUARE_SIZE - 10), "");
    //         dummy_ship.set_frame(FrameType::OvalBox);
    //         dummy_ship.set_color(Color::from_u32(0xE9ECF0));
    //     }


    //     // This is a concept of drawing a box to the screen. We can keep it for now as a backup.
    //     // let mut screen_blocker1 = fltk::draw::draw_box(FrameType::FlatBox, 0, 0, 1750, 600, Color::from_u32(0x00000A));

    //     // Button that covers the screen between turns.
    //     let mut screen_blocker = Button::new(0, 0, 1750, 600, "Click here to start your turn.");

    //     screen_blocker.set_frame(FrameType::FlatBox);
    //     screen_blocker.set_color(Color::from_u32(0xC0C0C0));
    //     screen_blocker.hide();

    //     // Button that indicates a user is finished looking a the board after a turn.
    //     let mut cont_btn = Button::new( 1600, 555, 100, 40, "Continue");

    //     cont_btn.set_frame(FrameType::ThinUpBox);
    //     cont_btn.set_color(Color::from_u32(0x4C9C48));
    //     cont_btn.set_selection_color(Color::from_u32(0x397536));

    //     // Hide the screen blocker when clicked.
    //     screen_blocker.set_callback(move |btn|
    //     {
    //         println!("Player 1");
    //         btn.hide();
    //     });

    //     // Call back for the continue button that blocks the screen with screen_blocker.
    //     cont_btn.set_callback(move |btn|
    //     {
    //         println!("Player 1");
    //         screen_blocker.show();
    //         // btn.hide() // possibly hide or disable when the screen_blocker is shown?
    //     });

        
    //     // Cool alert... we can delete once we know we dont need it.
    //     // fltk::dialog::alert(
    //     //     875, 
    //     //     300, 
    //     //     "Dissmiss to start your turn."
    //     // );

    //     // Dont know how to set a callback for this yet, or require valid input.
    //     /*let mut ship_input = fltk::dialog::input(
    //         875, 
    //         300, 
    //         "How many ships do you want to play with?",
    //         ""
    //     );
    //     //Basic callback functionality
    //     wind.end();
    //     wind.show();
    //     //app.run().unwrap();
    //     */
        
    //     // BattleShipGame
    //     // {
    //     //     player_one: Player::new_player(ship_count, Players::PlayerOne),
    //     //     player_two: Player::new_player(ship_count, Players::PlayerTwo),
    //     //     is_player_one_turn: true,
    //     //     ship_count: ship_count,
    //     // }
        
    //}

    // RELEVANT CLI CODE BEGINS BELOW

    /// This function will execute the turn for the current player, and will return a bool of
    /// whether or not the game has finished
    pub fn take_turn(&mut self, pos: (u8, u8)) -> (FireState, Option<&Players>)
    {
        if self.is_player_one_turn 
        {
            // First we fire on the other player's board
            let hit_or_miss = self.player_two.fire(pos);

            // Invert the member variable dictating whose turn it is
            self.is_player_one_turn = !self.is_player_one_turn;

            // Then we return either a Some is victory is achieved, or a None if it hasn't
            let vic = if self.check_victory() { Some(self.player_one.get_sig()) } else { None };

            (hit_or_miss, vic)
        }
        else
        {
            // Same idea here
            let hit_or_miss = self.player_one.fire(pos);
            self.is_player_one_turn = !self.is_player_one_turn;
            let vic = if self.check_victory() { Some(self.player_two.get_sig()) } else { None };
            (hit_or_miss, vic)
        }
    }    

    /// This is a private function that will check to see if there's been a victory from either
    /// player
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

    // Now we begin a CLI implementation

    /// This function obtains a ship count from the players and returns a BattleShipGame object
    pub fn begin() -> BattleShipGame
    {
        // Obtain ship count
        let mut ship_count = 0;
        let mut local = 1;
        let mut difficulty = 1;
        let mut ai_bool = false;
        let mut correct_input = false;
        while  !correct_input
        {
            let mut input = String::new();
            println!("Are you playing (1) locally or with an (2) AI? Type 1 or 2!");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            local = input.trim().parse::<usize>().unwrap_or(0);
            
            if local == 2
            {
                println!("Do you want to play on (1) easy, (2) medium, or (3) hard mode? Type 1, 2, 3!");
                input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                difficulty = input.trim().parse::<usize>().unwrap_or(0);
                ai_bool = true;
            }

            println!("How many ships will you play with?");
            input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            ship_count = input.trim().parse::<usize>().unwrap_or(0);

            if ship_count < 1 || ship_count > 6
            {
                println!("Ship count must be between 1 and 6.");
            }
            else if local < 1 || local > 2
            {
                println!("Must chose (1) local or (2) AI. Type 1 or 2!");
            }
            else if difficulty < 1 || difficulty > 3
            {
                println!("Must chose (1) easy, (2) medium, or (3) hard mode? Type 1, 2, 3!");
            }
            else
            {
                println!("{}", local);
                correct_input = true;
            }
        }

        BattleShipGame
        {
            player_one: Player::new_player(ship_count, Players::PlayerOne),
            player_two: Player::new_player(ship_count, Players::PlayerTwo),
            is_player_one_turn: true,
            ship_count: ship_count,
            is_p2_ai: ai_bool,
        }
    }

    /// This function takes char inputs and returns usize ints.
    /// It is to be used when the players are providing input for 
    /// where to place ships or attack, since the chars need to
    /// be turned into numbers to update game values
    pub fn char_convert(&self, x: char) -> usize
    {
        match x
        {
            'A' => return 1,
            'B' => return 2,
            'C' => return 3,
            'D' => return 4,
            'E' => return 5,
            'F' => return 6,
            'G' => return 7,
            'H' => return 8,
            'I' => return 9,
            'J' => return 10,
            'a' => return 1,
            'b' => return 2,
            'c' => return 3,
            'd' => return 4,
            'e' => return 5,
            'f' => return 6,
            'g' => return 7,
            'h' => return 8,
            'i' => return 9,
            'j' => return 10,
            _ => return 0,
        }
    }

    /// This function will allow both players to place ships
    pub fn place_ships(&mut self)
    {
        let mut row: usize;
        let mut col_char: char;
        let mut col: usize;
        let mut vertical = true;
        let mut local_players = 2;

        // This line clears the terminal
        print!("{esc}c", esc = 27 as char);
        
        if self.is_p2_ai == true
        {
            local_players = 1;  
        }
        
        for n in 1..=local_players
        {
            if n == 1
            {
                println!("Player 2, please look away");
                println!("Player 1, time to place your ships");
            }
            else 
            {
                println!("Player 1, please look away");
                println!("Player 2, time to place your ships");
            }

            // Will run this loop until all ships are placed
            for i in 0..self.ship_count
            {
                let mut ship_conflict = true;
                while ship_conflict
                {
                    row = 0;
                    col = 0;
                    println!("Your board so far:");
                    if n == 1
                    {
                        self.player_one.print_board(true);
                    }
                    else 
                    {
                        self.player_two.print_board(true);
                    }

                    // Get vertical or horizontal
                    let mut correct_input = false;
                    while !correct_input
                    {
                        println!("Vertical or horizontal?(v/h)");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let choice = input.trim().parse::<char>().unwrap_or('x');
                        if choice == 'v' || choice == 'V'
                        {
                            vertical = true;
                            correct_input = true;
                        }
                        else if choice == 'h' || choice == 'H'
                        {
                            vertical = false;
                            correct_input = true;
                        }
                        else 
                        {
                            println!("Invalid choice");
                        }
                    }

                    // Get row
                    correct_input = false;
                    while !correct_input
                    {
                        let mut input = String::new();
                        println!("Choose a row to start ship {} at", i+1);
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        row = input.trim().parse::<usize>().unwrap_or(0);
                        if row < 1 || row > 9
                        {
                            println!("Invalid input");
                        }
                        else if row + i > 9 && vertical
                        {
                            println!("Ship cannot go there");
                        }
                        else
                        {
                            correct_input = true;
                        }
                    }

                    // Get column
                    correct_input = false;
                    while !correct_input
                    {
                        let mut input = String::new();
                        println!("Choose a column to start ship {} at", i+1);
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        col_char = input.trim().parse::<char>().unwrap_or('x');
                        col = self.char_convert(col_char);
                        if col < 1 || row > 10
                        {
                            println!("Invalid input");
                        }
                        else if col + i > 10 && !vertical
                        {
                            println!("Ship cannot go there");
                        }
                        else
                        {
                            correct_input = true;
                        }
                    }

                    // This will check if there will be any overlap with given input and existing ships
                    if n == 1
                    {
                        for k in 0..=i
                        {
                            if vertical
                            {
                                if self.player_one.ship_index_at((row as u8 - 1 + k as u8, col as u8 - 1)) == None
                                {
                                    ship_conflict = false;
                                }
                                else
                                {
                                    print!("{esc}c", esc = 27 as char);
                                    println!("There is already a ship there");
                                    ship_conflict = true;
                                    break;
                                }
                            }
                            else
                            {
                                if self.player_one.ship_index_at((row as u8 - 1, col as u8 - 1 + k as u8)) == None
                                {
                                    ship_conflict = false;
                                }
                                else
                                {
                                    print!("{esc}c", esc = 27 as char);
                                    println!("There is already a ship there");
                                    ship_conflict = true;
                                    break;
                                }
                            }
                        }
                    }
                    else 
                    {
                        for k in 0..=i
                        {
                            if vertical
                            {
                                if self.player_two.ship_index_at((row as u8 - 1 + k as u8, col as u8 - 1)) == None
                                {
                                    ship_conflict = false;
                                }
                                else
                                {
                                    print!("{esc}c", esc = 27 as char);
                                    println!("There is already a ship there");
                                    ship_conflict = true;
                                    break;
                                }
                            }
                            else
                            {
                                if self.player_two.ship_index_at((row as u8 - 1, col as u8 - 1 + k as u8)) == None
                                {
                                    ship_conflict = false;
                                }
                                else
                                {
                                    print!("{esc}c", esc = 27 as char);
                                    println!("There is already a ship there");
                                    ship_conflict = true;
                                    break;
                                }
                            }
                        }
                    }

                    // This ensures that ships are only placed when they don't overlap
                    if !ship_conflict
                    {
                        // Place a ship
                        if n == 1
                        {
                            self.player_one.place_ship((col as u8 - 1, row as u8 - 1), !vertical);
                        }
                        else 
                        {
                            self.player_two.place_ship((col as u8 - 1, row as u8 - 1), !vertical);
                        }
                        print!("{esc}c", esc = 27 as char);
                    }
                }
            }
        }
    }

    /// This will loop until the game is over
    /// It will cycle between each player's turn, allowing
    /// them to make attacks until it determines that one player
    /// is out of ships
    pub fn play_game(&mut self)
    {
        let mut game_over = false;
        while !game_over
        {
            let mut correct_input = false;
            // print!("{esc}c", esc = 27 as char);
            if self.is_player_one_turn
            {
                println!("Please swap to player one");
            }
            else
            {
                println!("Please swap to player two");
            }
            println!("Press enter when ready");
            let mut garbage = String::new();
            io::stdin().read_line(&mut garbage);

            println!("Your ships:");

            if self.is_player_one_turn
            {
                self.player_one.print_board(true);
                if !self.player_one.printed_s_check()
                {
                   println!("player 2 wins!");
                   break;
                }
            }
            else
            {
                self.player_two.print_board(true);
                if !self.player_two.printed_s_check()
                {
                    println!("player 1 wins!");
                    break;
                }
            }
            println!("Your opponent's board so far:");
            if self.is_player_one_turn
            {
                self.player_two.print_board(false);
            }
            else
            {
                self.player_one.print_board(false);
            }

            // While loop to ensure valid input coordinates
            while !correct_input
            {
                let mut row = 0;
                let mut col = 0;
                let mut col_char: char;
                let mut input = String::new();
                println!("Time to make an attack!");

                println!("Choose a row:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                row = input.trim().parse::<usize>().unwrap_or(0);

                input = String::new();
                println!("Choose a column:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                col_char = input.trim().parse::<char>().unwrap_or('x');
                col = self.char_convert(col_char);

                if row < 1 || row > 9 || col < 1 || col > 10
                {
                    println!("Invalid coordinates");
                }
                else if self.is_player_one_turn && self.player_two.is_attacked(row-1, col-1)
                {
                    println!("Already attacked there");
                }
                else if !self.is_player_one_turn && self.player_one.is_attacked(row-1, col-1)
                {
                    println!("Already attacked there");
                }
                else
                {
                    correct_input = true;
                    self.take_turn((col as u8 - 1, row as u8 - 1));
                }
            }
            
        }
    }
}

// Debug block
#[allow(dead_code)]
impl BattleShipGame
{
    //Prints player 1 board
    pub fn print_p1_board(&mut self)
    {
        self.player_one.print_board(false);
    }

    //Prints player 1's ships
    pub fn print_p1_ships(&self)
    {
        self.player_one.print_ships();
    }

    pub fn test(&mut self)
    {
        // Alternate player turns, starting at p1
        self.turn_debug((0, 0));
        self.turn_debug((0, 0));
        self.turn_debug((5, 5));
        self.turn_debug((2, 2));
        self.turn_debug((3, 3));
        self.turn_debug((0, 1));
    }

    //Turn debugger
    pub fn turn_debug(&mut self, pos: (u8, u8))
    {
        //Takes a turn 
        let (hit, vic) = self.take_turn(pos);

        println!("{}, turn for {} at position ({}, {}) resulted in a {}",
        //Prints whether the game is still going or not
        match vic
        {
            Some(_) => "over",
            None => "playing",
        },
        //Prints whether its player 1's turn or not
        match &mut self.is_player_one_turn
        {
            true => "P1",
            false => "P2",
        }, pos.0, pos.1, //Prints the position targeted
        //Prints whether it was a hit or not
        match hit
        {
            FireState::Miss => "miss",
            FireState::Hit => "hit",
        });
    }
}
