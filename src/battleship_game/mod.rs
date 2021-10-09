// uses
use std::io;

// Import the sub-modules
mod data_structures;
mod player;
mod ship;
mod medium;

// Import the battleship_game module from the root of the crate
use crate::battleship_game::player::*;
use crate::battleship_game::data_structures::*;
use crate::battleship_game::medium::*;
use rand::Rng;

// // Size constants
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 9;

/// This is the main game logic handler, it holds both play objects, and therefore both boards and
/// all the ships on the board
pub struct BattleShipGame
{
    /// The object that holds the first player
    player_one: Player, 

    /// The object that holds the second player
    player_two: Player,

    player_medium_ai: MediumAI,

    /// This variable keeps track of whose turn it is
    is_player_one_turn: bool,
    ship_count: usize,
    is_p2_ai: bool,
    ai_difficulty: usize,
    game_over: bool,
}

#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_assignments, unused_must_use)]
impl BattleShipGame
{

    // RELEVANT CLI CODE BEGINS BELOW

    /// This function will execute the turn for the current player, and will return a bool of
    /// whether or not the game has finished
    
    pub fn take_turn(&mut self, pos: (u8, u8))
    {
        if self.is_player_one_turn 
        {
            // First we fire on the other player's board
            let hit_or_miss = self.player_two.fire(pos);
            self.player_one.count_hits_misses(self.player_two.ship_index_at(pos));

            // Invert the member variable dictating whose turn it is
            self.is_player_one_turn = !self.is_player_one_turn;

            // Then we return either a Some is victory is achieved, or a None if it hasn't
            let vic = if self.check_victory() { Some(self.player_one.get_sig()) } else { None };

        }
        else
        {
            // Same idea here
            let hit_or_miss = self.player_one.fire(pos);
            self.player_two.count_hits_misses(self.player_one.ship_index_at(pos));
            self.is_player_one_turn = !self.is_player_one_turn;
            let vic = if self.check_victory() { Some(self.player_two.get_sig()) } else { None };
            if (self.ai_difficulty == 2 && matches!(hit_or_miss , FireState::Hit)){
                if (self.player_medium_ai.get_main_hit() == (0,0)){
                    self.player_medium_ai.assign_main_hit(pos);
                    self.player_medium_ai.assign_current_hit(pos);
                    self.player_medium_ai.change_direction("on");
                } else {
                    self.player_medium_ai.assign_current_hit(pos);
                    self.player_medium_ai.change_direction("off");
                    self.player_medium_ai.set_run_direction(true);
                }
            } else {
                let main_hit = self.player_medium_ai.get_main_hit();
                self.player_medium_ai.assign_current_hit(main_hit);
                self.player_medium_ai.set_run_direction(false);
                self.player_medium_ai.change_direction("on");
            }
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
                correct_input = true;
            }
        }

        BattleShipGame
        {
            player_one: Player::new_player(ship_count, Players::PlayerOne),
            player_two: Player::new_player(ship_count, Players::PlayerTwo),
            player_medium_ai: MediumAI::create_ai(),
            is_player_one_turn: true,
            ship_count: ship_count,
            is_p2_ai: ai_bool,
            ai_difficulty: difficulty,
            game_over: false,
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

    pub fn ai_place_ships(&mut self)
    {
        for ship in 0..self.ship_count
        {
            let mut ship_conflict = true;
            while ship_conflict
            {
                let mut vertical = true;
                if ship == 0
                {
                    let col = rand::thread_rng().gen_range(1..11);
                    let row = rand::thread_rng().gen_range(1..10);
                    self.player_two.place_ship((col as u8 - 1, row as u8 - 1), !vertical);
                    ship_conflict = false;
                }
                else
                {
                    let mut valid_loc = false;
                    let hv = rand::thread_rng().gen_range(1..3); 
                    while !valid_loc
                    {
                        let col = rand::thread_rng().gen_range(1..11);
                        let row = rand::thread_rng().gen_range(1..10);
                        // Vertical
                        if hv == 1
                        {
                           println!("Hello");
                            if row + ship > 9 && hv == 1
                            {
                                valid_loc = false;
                            }
                            else
                            {
                                for k in 0..= ship
                                {
                                    if self.player_two.ship_index_at((col as u8 - 1 + k as u8, row as u8 - 1)) == None
                                    {
                                        valid_loc = true;
                                        ship_conflict = false;
                                        
                                    }
                                    else
                                    {
                                        valid_loc = false;
                                        break;
                                    }
                                }
                                if !ship_conflict && valid_loc
                                {
                                    self.player_two.place_ship((col as u8 - 1, row as u8 - 1), !vertical);
                                }
                            }
                        }
                        else
                        {
                            vertical = false;
                            if col + ship > 10 && !vertical
                            {
                                valid_loc = false;
                            }
                            else
                            {
                                for k in 0..= ship
                                {
                                    if self.player_two.ship_index_at((row as u8 - 1, col as u8 - 1 + k as u8)) == None
                                    {
                                        valid_loc = true;
                                        ship_conflict = false;
                                    }
                                    else
                                    {
                                        valid_loc = false;
                                        break;
                                    }
                                }
                                if !ship_conflict && valid_loc
                                {
                                    self.player_two.place_ship((col as u8 - 1, row as u8 - 1), !vertical);
                                }
                            
                            }
                        }
                    }
                    
                }
                
            }
            
            
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
            self.ai_place_ships();
            //self.player_two.print_board(true);
        }
        
        for n in 1..=local_players
        {
		if(self.is_p2_ai==true)
		{
			if(n==1)
			{
				println!("Player 1, time to place your ships")
				
			}
		}
		else
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
                        	println!("Hello");
                            if vertical
                            {
                                if self.player_one.ship_index_at((col as u8 - 1 + k as u8, row as u8 - 1)) == None
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
                                if self.player_one.ship_index_at((col as u8 - 1, row as u8 - 1 + k as u8)) == None
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
                                if self.player_two.ship_index_at((col as u8 - 1 + k as u8, row as u8 - 1)) == None
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
                                if self.player_two.ship_index_at((col as u8 - 1, row as u8 - 1 + k as u8)) == None
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
    pub fn ai_easy_turn(&mut self)
    {
        loop {
            let col = rand::thread_rng().gen_range(1..11);
            let row = rand::thread_rng().gen_range(1..10);
            if self.player_one.is_attacked(row-1,col-1) == false {
                self.take_turn((col as u8 - 1, row as u8 - 1));
                break;
            }
        } 
        //println!("AI fired at: ({}, {})", col, row);
    }
    
    pub fn ai_med_turn(&mut self){

        if self.player_medium_ai.get_run_direction() == true {
            if (self.player_medium_ai.get_current_hit().1 == 0) && self.player_medium_ai.get_direction("up") == true {
                self.player_medium_ai.change_direction("on");
            } else if self.player_medium_ai.get_current_hit().0 == 0 && self.player_medium_ai.get_direction("left") == true {
                self.player_medium_ai.change_direction("on");
            } else if self.player_medium_ai.get_current_hit().0 == 9 && self.player_medium_ai.get_direction("right") == true {
                self.player_medium_ai.change_direction("on");
            } else if self.player_medium_ai.get_current_hit().1 == 8 && self.player_medium_ai.get_direction("down") == true {
                self.player_medium_ai.change_direction("on");
            }
        }

        if (self.player_medium_ai.get_main_hit() == (0,0)){
            loop {
                let col = rand::thread_rng().gen_range(1..11);
                let row = rand::thread_rng().gen_range(1..10);
                if self.player_one.is_attacked(row-1,col-1) == false {
                    self.take_turn((col as u8 - 1, row as u8 - 1));
                    break;
                }
            } 
        } else {
            if (self.player_medium_ai.get_direction("up") == true && (self.player_medium_ai.get_current_hit().1) != 0 && self.ai_med_turn_valid("up") == true) {
                self.player_medium_ai.change_direction("up");  
                let col = self.player_medium_ai.get_current_hit().0;
                let row = (self.player_medium_ai.get_current_hit().1) - 1;
                self.take_turn((col as u8, row as u8));
                if (self.player_medium_ai.get_run_direction() == true) {
                    self.player_medium_ai.change_direction("up");
                }
            } else if (self.player_medium_ai.get_direction("right") == true && (self.player_medium_ai.get_current_hit().0) + 1 < 10 && self.ai_med_turn_valid("right") == true){
                self.player_medium_ai.change_direction("right");
                let col = (self.player_medium_ai.get_current_hit().0) + 1;
                let row = self.player_medium_ai.get_current_hit().1;
                self.take_turn((col as u8, row as u8 ));
                if (self.player_medium_ai.get_run_direction() == true) {
                    self.player_medium_ai.change_direction("right");
                }
            } else if (self.player_medium_ai.get_direction("down") == true && (self.player_medium_ai.get_current_hit().1) + 1 < 9 && self.ai_med_turn_valid("down") == true){
                self.player_medium_ai.change_direction("down");
                let col = self.player_medium_ai.get_current_hit().0;
                let row = (self.player_medium_ai.get_current_hit().1) + 1;
                self.take_turn((col as u8 , row as u8 ));
                if (self.player_medium_ai.get_run_direction() == true) {
                    self.player_medium_ai.change_direction("down");
                }
            } else if (self.player_medium_ai.get_direction("left") == true && (self.player_medium_ai.get_current_hit().0) != 0 && self.ai_med_turn_valid("left") == true){
                self.player_medium_ai.change_direction("left");
                let col = (self.player_medium_ai.get_current_hit().0) - 1;
                let row = self.player_medium_ai.get_current_hit().1;
                self.take_turn((col as u8 , row as u8 ));
                if (self.player_medium_ai.get_run_direction() == true) {
                    self.player_medium_ai.change_direction("left");
                }
            } else {
                self.player_medium_ai.assign_current_hit((0,0));
                self.player_medium_ai.assign_main_hit((0,0));
                loop {
                    let col = rand::thread_rng().gen_range(1..11);
                    let row = rand::thread_rng().gen_range(1..10);
                    if self.player_one.is_attacked(row-1,col-1) == false {
                        self.take_turn((col as u8 - 1, row as u8 - 1));
                        break;
                    }
                } 
            }
        }
    }

    pub fn ai_med_turn_valid(&mut self, direction: &str) -> bool {
        if direction == "up" {
            let col = self.player_medium_ai.get_current_hit().0;
            let row = (self.player_medium_ai.get_current_hit().1) - 1;
            !self.player_one.is_attacked(row.into(), col.into())
        } else if direction == "down"{
            let col = self.player_medium_ai.get_current_hit().0;
            let row = (self.player_medium_ai.get_current_hit().1) + 1;
            !self.player_one.is_attacked(row.into(), col.into())
        } else if direction == "left" {
            let col = (self.player_medium_ai.get_current_hit().0) - 1;
            let row = self.player_medium_ai.get_current_hit().1;
            !self.player_one.is_attacked(row.into(), col.into())
        } else if direction == "right" {
            let col = (self.player_medium_ai.get_current_hit().0) + 1;
            let row = self.player_medium_ai.get_current_hit().1;
            !self.player_one.is_attacked(row.into(), col.into())
        } else {
            false
        }
    }

    pub fn ai_hard_turn(&mut self)
    {

        while self.is_player_one_turn == false
        {
            for row in 1..=9{
                for col in 1..=10{
                    if self.player_one.is_ship(col-1,row-1) == true && self.player_one.board_matrix[row-1][col-1] == WaterSquare::Empty
                    {
                        //println!("AI fired at: ({}, {})", row, col);
                        self.take_turn((col as u8 - 1,row as u8 - 1));
                        return
                    }
                }
            }
        }
    }


    pub fn ai_play_game(&mut self)
    {
    	let mut correct_input = false;
        if self.is_player_one_turn
        {
            print!("{esc}c", esc = 27 as char);
            self.player_one.print_scoreboard();
            println!("{:?} is the main hit, {:?} is the current hit", self.player_medium_ai.get_main_hit(), self.player_medium_ai.get_current_hit() );
            println!("lookLeft: {}, lookRight: {}, lookUp: {}, lookDown: {}", self.player_medium_ai.get_direction("left"), self.player_medium_ai.get_direction("right"), self.player_medium_ai.get_direction("up"), self.player_medium_ai.get_direction("down"));
            println!("Directions: {}", self.player_medium_ai.get_direction("all"));
            println!("Your ships:");
            self.player_one.print_board(true);
            println!("AI board (our pespective):");
            self.player_two.print_board(false);
            println!("AI board (debug)");
            self.player_two.print_board(true);
            if self.player_one.all_ships_sunk()
            {
                println!("AI wins!");
                self.game_over = true;
                return;
            }	
            else if self.player_two.all_ships_sunk()
            {
                println!("Player 1 wins!");
                self.game_over = true;
                return;
            }
        }
        while !correct_input
        {
            if self.is_player_one_turn
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
            else
            {
                correct_input=true;
                if self.ai_difficulty == 1
                {
                    self.ai_easy_turn();
                } else if self.ai_difficulty == 2
                {
                    self.ai_med_turn();
                } else if self.ai_difficulty == 3
                {
                    self.ai_hard_turn();
                }


                if self.player_two.all_ships_sunk()
		        {
			        println!("Player 1 wins!");
			        self.game_over = true;
			        return;
		        } else if self.player_one.all_ships_sunk()
		        {
			        println!("AI wins!");
			        self.game_over = true;
			        return;
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
        while !self.game_over
        {
            if self.is_p2_ai == true
            {
                self.ai_play_game();
            }
            
            else
            {
                let mut correct_input = false;
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
                    self.player_one.print_scoreboard();
                    self.player_one.print_board(true);
                    if !self.player_one.printed_s_check()
                    {
                        println!("player 2 wins!");
                        break;
                    }
                }
                else
                {
                    self.player_two.print_scoreboard();
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
}
