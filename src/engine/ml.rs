pub struct Board {
    pub state: [[Option<char>; 3]; 3],
    pub moves_played: i32,
    pub w1: f64,
    pub w2: f64,
    pub w3: f64,
    pub w4: f64,
    pub w5: f64,
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: [[None; 3]; 3],
            moves_played: 0,
            w1: 0.0,
            w2: 0.0,
            w3: 0.0,
            w4: 0.0,
            w5: 0.0,
        }
    }

    pub fn board_state(&self) -> f64 {
        let mut score = 0.0;
        for row in &self.state {
            for cell in row {
                match cell {
                    Some('X') => score += 1.0,
                    Some('O') => score -= 1.0,
                    _ => (),
                }
            }
        }
        score as f64
    }

    pub fn winning_lines(&self, player: char) -> bool {
        // rows and columns
        for i in 0..3 {
            if (self.state[i][0] == Some(player)
                && self.state[i][1] == Some(player)
                && self.state[i][2] == Some(player))
                || (self.state[0][i] == Some(player)
                    && self.state[1][i] == Some(player)
                    && self.state[2][i] == Some(player))
            {
                return true;
            }
        }

        // diagonal
        if (self.state[0][0] == Some(player)
            && self.state[1][1] == Some(player)
            && self.state[2][2] == Some(player))
            || (self.state[0][2] == Some(player)
                && self.state[1][1] == Some(player)
                && self.state[2][0] == Some(player))
        {
            return true;
        }

        false
    }

    fn blocking_opponent(&self) -> f64 {
        let mut score = 0.0;

        // rows and columns
        for i in 0..3 {
            if (self.state[i][0] == Some('O')
                && self.state[i][1] == Some('O')
                && self.state[i][2] == None)
                || (self.state[i][0] == None
                    && self.state[i][1] == Some('O')
                    && self.state[i][2] == Some('O'))
                || (self.state[i][0] == Some('O')
                    && self.state[i][1] == None
                    && self.state[i][2] == Some('O'))
                || (self.state[0][i] == Some('O')
                    && self.state[1][i] == Some('O')
                    && self.state[2][i] == None)
                || (self.state[0][i] == None
                    && self.state[1][i] == Some('O')
                    && self.state[2][i] == Some('O'))
                || (self.state[0][i] == Some('O')
                    && self.state[1][i] == None
                    && self.state[2][i] == Some('O'))
            {
                score += 1.0;
            }
        }

        // diagonals
        if (self.state[0][0] == Some('O')
            && self.state[1][1] == Some('O')
            && self.state[2][2] == None)
            || (self.state[0][0] == None
                && self.state[1][1] == Some('O')
                && self.state[2][2] == Some('O'))
            || (self.state[0][0] == Some('O')
                && self.state[1][1] == None
                && self.state[2][2] == Some('O'))
            || (self.state[0][2] == Some('O')
                && self.state[1][1] == Some('O')
                && self.state[2][0] == None)
            || (self.state[0][2] == None
                && self.state[1][1] == Some('O')
                && self.state[2][0] == Some('O'))
            || (self.state[0][2] == Some('O')
                && self.state[1][1] == None
                && self.state[2][0] == Some('O'))
        {
            score += 1.0;
        }

        score as f64
    }

    fn center_control(&self) -> f64 {
        if self.state[1][1] == Some('X') {
            1 as f64
        } else {
            0 as f64
        }
    }

    fn corner_control(&self) -> f64 {
        let mut score = 0.0;
        if self.state[0][0] == Some('X') {
            score += 1.0;
        }
        if self.state[0][2] == Some('X') {
            score += 1.0;
        }
        if self.state[2][0] == Some('X') {
            score += 1.0;
        }
        if self.state[2][2] == Some('X') {
            score += 1.0;
        }
        score as f64
    }

    // `mobility` refers to the number of legal moves available to a player from a given state
    fn mobility(&self) -> f64 {
        let mut score = 0.0;
        for row in &self.state {
            for cell in row {
                if *cell == None {
                    score += 1.0;
                }
            }
        }
        score as f64
    }

    pub fn game_progress(&self) -> f64 {
        self.moves_played as f64
    }

    //TODO: change these wi's to 0
    pub fn v(&mut self) -> f64 {
        let mut score: f64 = 0.0;
        if self.winning_lines('X') {
            score = -100.0;
        } else if self.winning_lines('O') {
            score = 100.0;
        }

        let old_value: f64 = self.w1 * self.board_state()
            + self.w2 * self.blocking_opponent()
            + self.w3 * self.center_control()
            + self.w4 * self.corner_control()
            + self.w5 * self.mobility()
            - self.game_progress();

        let new_value: f64 = 0.1 * (score - old_value);

        self.w1 += new_value * self.board_state();
        self.w2 += new_value * self.blocking_opponent();
        self.w3 += new_value * self.center_control();
        self.w4 += new_value * self.corner_control();
        self.w5 += new_value * self.mobility();

        return old_value;
    }
}
