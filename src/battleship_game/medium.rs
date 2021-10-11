pub struct MediumAI {
    current_hit: (u8,u8),
    main_hit: (u8,u8),
    look_right: bool,
    look_left: bool,
    look_down: bool,
    look_up: bool,
    run_direction: bool,
}

impl MediumAI {

    pub fn create_ai() -> MediumAI {
        MediumAI 
        {
            current_hit: (0,0),
            main_hit: (0,0),
            look_right: false,
            look_left: false,
            look_up: false,
            look_down: false,
            run_direction: false,
        }
    }

    pub fn assign_current_hit(&mut self, pos: (u8,u8)){
        self.current_hit = pos;
    }

    pub fn assign_main_hit(&mut self, pos: (u8,u8)){
        self.main_hit = pos;
    }

    pub fn get_current_hit(&mut self) -> (u8,u8){
        self.current_hit
    }

    pub fn get_main_hit(&mut self) -> (u8,u8){
        self.main_hit
    }

    pub fn get_run_direction(&mut self) -> bool {
        self.run_direction
    }

    pub fn set_run_direction(&mut self, var: bool){
        if var == true {
            self.run_direction = true;
        } else {
            self.run_direction = false;
        }
    }

    pub fn change_direction(&mut self, direction: &str) {
        if direction == "up" {
            self.look_up = !self.look_up;
        } else if direction == "down" {
            self.look_down =!self.look_down;
        } else if direction == "left" {
            self.look_left =!self.look_left;
        } else if direction == "right" {
            self.look_right =!self.look_right;
        } else if direction == "all" {
            self.look_up = !self.look_up;
            self.look_down =!self.look_down;
            self.look_left =!self.look_left;
            self.look_right =!self.look_right;
        } else if direction == "on" {
            self.look_up = true;
            self.look_down = true;
            self.look_left = true;
            self.look_right = true;
        } else if direction == "off" {
            self.look_up = false;
            self.look_down = false;
            self.look_left = false;
            self.look_right = false;
        }
    }

    pub fn get_direction(&mut self, direction: &str) -> bool {
        if direction == "up" {
            self.look_up
        } else if direction == "down" {
            self.look_down
        } else if direction == "left" {
            self.look_left
        } else if direction == "right" {
            self.look_right
        } else if direction == "all" {
            if (self.look_up && self.look_down && self.look_left && self.look_right) == false{
                false
            } else {
                true
            }
        } else {
            false
        }
    }
    
}
