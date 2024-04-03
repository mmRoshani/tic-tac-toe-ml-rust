use std::char;

pub struct Board {
    pub state: [[Option<char>; 3]; 3],
    pub moves_played: i32,
    pub w: Vec<f64>,
    pub x_o_winning_state: (i32, i32),
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: [[None; 3]; 3],
            moves_played: 0,
            w: vec![0.0; 16],
            x_o_winning_state: (0, 0),
        }
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

    pub fn count_letter_in_row(&self, letter: char, row_index: usize) -> usize {
        let mut counter: usize = 0;
        for j in 0..3 {
            if self.state[row_index][j] == Some(letter) {
                counter += 1;
            }
        }

        counter
    }

    pub fn count_letter_in_column(&self, letter: char, column_index: usize) -> usize {
        let mut counter: usize = 0;
        for i in 0..3 {
            if self.state[i][column_index] == Some(letter) {
                counter += 1;
            }
        }

        counter
    }

    pub fn extract_features(&self) -> Vec<usize> {
        let mut features = Vec::new();
        // diagonal search
        let mut main_diagnol_counter: usize = 0;
        let mut main_diagnol: Vec<Option<char>> = vec![];
        main_diagnol.push(self.state[0][0]);
        main_diagnol.push(self.state[1][1]);
        main_diagnol.push(self.state[2][2]);

        let mut sub_diagnol_counter: usize = 0;
        let mut sub_diagnol: Vec<Option<char>> = vec![];
        sub_diagnol.push(self.state[0][2]);
        sub_diagnol.push(self.state[1][1]);
        sub_diagnol.push(self.state[2][0]);
        for &letter in ['X', 'O'].iter() {
            for i in 0..3 {
                let row_counter = self.count_letter_in_row(letter, i);
                features.push(row_counter);
                let col_conter = self.count_letter_in_column(letter, i);
                features.push(col_conter);

                // diagonal
                if main_diagnol[i] == Some(letter) {
                    main_diagnol_counter += 1;
                }
                if sub_diagnol[i] == Some(letter) {
                    sub_diagnol_counter += 1;
                }
            }
            features.push(main_diagnol_counter);
            features.push(sub_diagnol_counter);
            main_diagnol_counter = 0;
            sub_diagnol_counter = 0;
        }

        features
    }

    pub fn calculate_value(&self, features: &[usize]) -> f64 {
        let mut value: f64 = 0.0;
        for i in 0..self.w.len() {
            value += self.w[i] * (features[i] as f64);
        }
        return value;
    }

    pub fn update_function(&mut self, winner: Option<char>) {
        let value = match winner {
            Some('X') => -100.0,
            Some('O') => 100.0,
            _ => 0.0,
        };

        let features = self.extract_features();

        for i in 0..self.w.len() {
            self.w[i] += 0.1 * (value - self.calculate_value(&features)) * features[i] as f64;
        }
        // println!("Weights: {:?}", self.w);
    }

    pub fn computer_move(&mut self) -> (usize, usize) {
        let mut max_value = f64::NEG_INFINITY;
        let mut best_move = None;

        for i in 0..3 {
            for j in 0..3 {
                if self.state[i][j].is_none() {
                    // Temporarily place the 'O'
                    self.state[i][j] = Some('O');
                    let features = self.extract_features();
                    let value = self.calculate_value(&features);
                    // Remove the 'O'
                    self.state[i][j] = None;

                    if value > max_value {
                        max_value = value;
                        best_move = Some((i, j));
                    }
                }
            }
        }

        if let Some((i, j)) = best_move {
            self.state[i][j] = Some('O');
        }
        best_move.unwrap_or((30, 30))
    }
}
