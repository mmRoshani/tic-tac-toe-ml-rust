mod engine;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let mut board = engine::ml::Board::new();

    
    ui.on_play(move |button_num: i32| {
        let ui = ui_handle.unwrap();
        

        match button_num {
            0 => {
                let machine_move = movemnt(&mut board, 0, 0);
                btn_lock(ui, machine_move);
            },
            1 => {
                let machine_move = movemnt(&mut board, 0, 1);
                btn_lock(ui, machine_move);
            },
            2 => {        
                let machine_move = movemnt(&mut board, 0, 2);                
                btn_lock(ui, machine_move);
            },  
            3 => {              
                let machine_move = movemnt(&mut board,1 , 0);              
                btn_lock(ui, machine_move);
            },
            4 => { 
                let machine_move = movemnt(&mut board,1 , 1);        
                btn_lock(ui, machine_move);
            },
            5 => {
                let machine_move = movemnt(&mut board,1 , 2);   
                btn_lock(ui, machine_move);
            },
            6 => { 
                let machine_move = movemnt(&mut board,2 , 0);       
                btn_lock(ui, machine_move);
            },
            7 => { 
                let machine_move = movemnt(&mut board,2 , 1);          
                btn_lock(ui, machine_move);
            },
            8 => {
                let machine_move = movemnt(&mut board,2 , 2);   
                btn_lock(ui, machine_move);
            },
            _ => println!("{}", "Unsupported case"),
        }
    });

    ui.run()
}

fn movemnt(board: &mut engine::ml::Board, row: usize, col: usize) -> (usize, usize) {
    board.state[row][col] = Some('X');

    println!("board score is {}", board.board_state());


    let mut best_val = -std::i32::MAX;
    let mut machine_best_move_val = (0, 0);

    for (ni, nj) in neighbors(row, col) {
      // set just to test
        if board.state[ni][nj] == None && row != ni && col != nj {

        board.state[ni][nj] = Some('O'); 

        let v = board.v();
        if v >= best_val {
            best_val = v;
            machine_best_move_val = (ni, nj);
        }
        else{
            // remove is not best move
            board.state[ni][nj] = None;
        }
        }
    }

    board.moves_played = board.moves_played + 2;

        for i in 0..3 {
            for j in 0..3 {
                print!("| {:?}| ", board.state[i][j]);
            }
        print!("\n");
        }
    
    machine_best_move_val
}

fn neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let mut neighbors = Vec::new();
    for (di, dj) in &directions {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0 && nj >= 0 && ni < 3 && nj < 3 {
            neighbors.push((ni as usize, nj as usize));
        }
    }

    neighbors
}

fn btn_lock(ui: AppWindow, machine_move: (usize, usize)) {
    let i: usize = machine_move.0;
    let j: usize = machine_move.1;
    
    if i == 0 && j == 0 {
        ui.set_btn_0("O".into());
    } 
    else if i == 0 && j == 1 {
        ui.set_btn_1("O".into());
    }else if i == 0 && j == 2 {
        ui.set_btn_2("O".into());
    }else if i == 1 && j == 0 {
        ui.set_btn_3("O".into());
    }
    else if i == 1 && j == 1 {
         ui.set_btn_4("O".into());
    }
    else if i == 1 && j == 2 {
        ui.set_btn_5("O".into());
    }else if i == 2 && j == 0 {
        ui.set_btn_6("O".into());
    }else if i == 2 && j == 1 {
        ui.set_btn_7("O".into());
    }else if i == 2 && j == 2 {
        ui.set_btn_8("O".into());
    }else{
        println!("Unhandled case!")
    }
}
