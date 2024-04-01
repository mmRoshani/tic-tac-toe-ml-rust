use std::collections::HashMap;

/// Also known as `TTT`
pub struct TicTacToe {
    board: [i32; 9],
    state_value: HashMap<[i32; 9], f32>,
    alpha: f32,
    counter: i32,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [0; 9],
            state_value: HashMap::new(),
            alpha: 0.5,
            counter: 0,
        }
    }

    fn turn(&mut self) -> i32 {
        let mut turn_sign = -1;

        if self.counter % 2 == 0 {
            turn_sign = 1;
        }else{
            turn_sign = 2;
        }

        self.counter = self.counter + 1;
        return turn_sign
    }

    /// Just giving the `X` and `O`
    /// out put.
    /// 
    /// #### notes:
    /// - Deferent object as the main TicTacToe object.  
    pub fn turn_str(&mut self) -> String {
        let mut turn_sign = String::new();

        if self.counter % 2 == 0 {
            turn_sign = String::from("X");
        }else{
            turn_sign=String::from("O");
        }

        return turn_sign
    }

    fn reset(&mut self) {
        self.board = [0; 9];
        self.counter =0;
    }

    fn is_terminal(&self) -> bool {
        // Check rows, columns and diagonals for a win
        // If there's a win, return true
        // If there's no win and no empty spaces, it's a draw, return true
        // Otherwise, return false
        // This is a placeholder, you'll need to implement this
        false
    }

    fn get_state_value(&self, state: &[i32; 9]) -> f32 {
        *self.state_value.get(state).unwrap_or(&0.0)
    }

    fn update_state_value(&mut self, state: &[i32; 9], new_val: f32) {
        let old_val = self.get_state_value(state);
        let updated_val = old_val + self.alpha * (new_val - old_val);
        self.state_value.insert(*state, updated_val);
    }

    pub fn make_move(&mut self, btn_num: usize) {
        // This is a placeholder, yvou'll need to implement this
        // Update the board with the move
        let turn_sign: i32 = self.turn();
        self.board[btn_num] = turn_sign;
        // If the game is in a terminal state, update the state value
        if self.is_terminal() {
            // print!("{}", String::from("The game ended!"))
        }else {
            // print!("{}", String::from("The game ended!"))
        }
    }
}

