pub struct Board {
    pub state: [[Option<char>; 3]; 3],
    pub moves_played: i32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: [[None; 3]; 3],
            moves_played: 0,
        }
    }

    pub fn board_state(&self) -> i32 {
        let mut score = 0;
        for row in &self.state {
            for cell in row {
                match cell {
                    Some('X') => score += 1,
                    Some('O') => score -= 1,
                    _ => (),
                }
            }
        }
        score
    }

    fn winning_lines(&self, player: char) -> i32 {
        let mut score = 0;

        // rows and columns
        for i in 0..3 {
            if (self.state[i][0] == Some(player)
                && self.state[i][1] == Some(player)
                && self.state[i][2] == Some(player))
                || (self.state[0][i] == Some(player)
                    && self.state[1][i] == Some(player)
                    && self.state[2][i] == Some(player))
            {
                score += 1;
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
            score += 1;
        }

        score
    }

    fn blocking_opponent(&self) -> i32 {
        let mut score = 0;

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
                score += 1;
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
            score += 1;
        }

        score
    }

    fn center_control(&self) -> i32 {
        if self.state[1][1] == Some('X') {
            1
        } else {
            0
        }
    }

    fn corner_control(&self) -> i32 {
        let mut score = 0;
        if self.state[0][0] == Some('X') {
            score += 1;
        }
        if self.state[0][2] == Some('X') {
            score += 1;
        }
        if self.state[2][0] == Some('X') {
            score += 1;
        }
        if self.state[2][2] == Some('X') {
            score += 1;
        }
        score
    }

    // `mobility` refers to the number of legal moves available to a player from a given state
    fn mobility(&self) -> i32 {
        let mut score = 0;
        for row in &self.state {
            for cell in row {
                if *cell == None {
                    score += 1;
                }
            }
        }
        score
    }

    pub fn game_progress(&self) -> i32 {
        self.moves_played
    }

    //TODO: change these wi's to 0
    pub fn v(&self) -> i32 {
        let w1 = 1;
        let w2 = 1;
        let w3 = 1;
        let w4 = 1;
        let w5 = 1;
        let w6 = 1;
        let w7 = 1;

        w1 * self.board_state() + w2 * self.winning_lines('X') - w3 * self.winning_lines('O')
            + w4 * self.blocking_opponent()
            + w5 * self.center_control()
            + w6 * self.corner_control()
            + w7 * self.mobility()
            - self.game_progress()
    }
}
