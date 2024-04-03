mod engine;
use image;
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;
use slint::VecModel;
use slint::{Image, Model};
use std::rc::Rc;

slint::include_modules!();

slint::slint! {
    import {StandardButton} from "std-widgets.slint";
    DialogBox := Dialog {
        property <string> message_text: "Default message";
        Text {
            text: root.message_text;
        }
    }
}

fn main() {
    let ui = MainWindow::new().unwrap();

    let tiles: Vec<TileData> = ui.get_memory_tiles().iter().collect();
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));

    ui.set_memory_tiles(tiles_model.clone().into());

    let ui_handle = ui.as_weak();

    let mut board = engine::ml::Board::new();

    ui.on_play(move |button_num: i32| {
        let ui = ui_handle.unwrap();
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.key == button_num);

        if let Some((t_idx, mut tile)) = flipped_tiles.next() {
            let tiles_model = tiles_model.clone();
            let mut cat_image = image::open("./icons/x.jpeg")
                .expect("Error loading cat image")
                .into_rgba8();
            image::imageops::colorops::brighten_in_place(&mut cat_image, 20);

            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                cat_image.as_raw(),
                cat_image.width(),
                cat_image.height(),
            );
            tile.image = Image::from_rgba8(buffer);
            tiles_model.set_row_data(t_idx, tile);

            // machine move
            match button_num {
                0 => {
                    let machine_move = movemnt(&mut board, &ui, 0, 0);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                1 => {
                    let machine_move = movemnt(&mut board, &ui, 0, 1);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                2 => {
                    let machine_move = movemnt(&mut board, &ui, 0, 2);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                3 => {
                    let machine_move = movemnt(&mut board, &ui, 1, 0);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                4 => {
                    let machine_move = movemnt(&mut board, &ui, 1, 1);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                5 => {
                    let machine_move = movemnt(&mut board, &ui, 1, 2);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                6 => {
                    let machine_move = movemnt(&mut board, &ui, 2, 0);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                7 => {
                    let machine_move = movemnt(&mut board, &ui, 2, 1);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                8 => {
                    let machine_move = movemnt(&mut board, &ui, 2, 2);
                    show_o(&tiles_model, &ui, machine_move, &mut board);
                }
                _ => println!("{}", "Unsupported case"),
            }
        }
    });

    // ui.on_reset_state(move || {
    //     // clear_state(&mut board);
    // });

    ui.run().unwrap();
}

fn movemnt(
    board: &mut engine::ml::Board,
    ui: &MainWindow,
    row: usize,
    col: usize,
) -> (usize, usize) {
    board.state[row][col] = Some('X');
    board.moves_played += 1;

    if board.winning_lines('X') {
        println!("u won");
        let dialog = DialogBox::new().expect("some error");
        dialog.set_message_text("you won!".into());
        let _ = dialog.run();
        ui.set_disable_tiles(true);
        return (10, 10);
    } else if board.winning_lines('O') {
        println!("computer won!");
        let dialog = DialogBox::new().expect("some error");
        dialog.set_message_text("computer won ;)".into());
        let _ = dialog.run();
        ui.set_disable_tiles(true);
        return (10, 10);
    } else if board.moves_played == 9 {
        println!("end of the game");
        let dialog = DialogBox::new().expect("some error");
        dialog.set_message_text("no one won, like life!".into());
        let _ = dialog.run();
        ui.set_disable_tiles(true);
        return (10, 10);
    }

    println!("board score is {}", board.board_state());

    let mut best_val = -std::f64::MAX;
    let mut machine_best_move_val = (0, 0);

    for (ni, nj) in neighbors(row, col) {
        // set just to test
        if board.state[ni][nj] == None && row != ni && col != nj {
            board.state[ni][nj] = Some('O');

            let v = board.v();
            if v >= best_val {
                best_val = v;
                machine_best_move_val = (ni, nj);
            } else {
                // remove is not best move
                board.state[ni][nj] = None;
            }
        }
    }

    board.moves_played += 1;

    machine_best_move_val
}

fn neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
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

fn show_o(
    tiles_model: &Rc<VecModel<TileData>>,
    ui: &MainWindow,
    machine_move: (usize, usize),
    board: &mut engine::ml::Board,
) {
    let i: usize = machine_move.0;
    let j: usize = machine_move.1;
    let mut tile_idx: usize = 512;

    if i == 0 && j == 0 {
        tile_idx = 0;
    } else if i == 0 && j == 1 {
        tile_idx = 1;
    } else if i == 0 && j == 2 {
        tile_idx = 2;
    } else if i == 1 && j == 0 {
        tile_idx = 3;
    } else if i == 1 && j == 1 {
        tile_idx = 4;
    } else if i == 1 && j == 2 {
        tile_idx = 5;
    } else if i == 2 && j == 0 {
        tile_idx = 6;
    } else if i == 2 && j == 1 {
        tile_idx = 7;
    } else if i == 2 && j == 2 {
        tile_idx = 8;
    } else {
        let mut cat_image = image::open("./icons/meme.png")
            .expect("Error loading cat image")
            .into_rgba8();
        image::imageops::colorops::brighten_in_place(&mut cat_image, 20);

        for (i, mut tile) in tiles_model.iter().enumerate() {
            tile.image_visible = false; // change after cleared the statue
            tile.solved = false;
            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                cat_image.as_raw(),
                cat_image.width(),
                cat_image.height(),
            );
            tile.image = Image::from_rgba8(buffer);
            tiles_model.set_row_data(i, tile);
        }

        ui.set_disable_tiles(false);
        board.state = [[None; 3]; 3];
        println!(
            "w1: {}, w2: {}, w3: {}, w4: {}, w5: {}, w6: {}, w7: {}",
            board.w1, board.w2, board.w3, board.w4, board.w5, board.w6, board.w7
        );
    }

    let mut flipped_tiles = tiles_model
        .iter()
        .enumerate()
        .filter(|(_, tile)| tile.key == tile_idx.try_into().unwrap());

    if let Some((t_idx, mut tile)) = flipped_tiles.next() {
        let tiles_model = tiles_model.clone();
        let mut cat_image = image::open("./icons/o.png")
            .expect("Error loading cat image")
            .into_rgba8();
        image::imageops::colorops::brighten_in_place(&mut cat_image, 20);

        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            cat_image.as_raw(),
            cat_image.width(),
            cat_image.height(),
        );
        tile.image = Image::from_rgba8(buffer);
        tile.image_visible = true; // change after cleared the statue
        tile.solved = true;
        tiles_model.set_row_data(t_idx, tile);
    }
}
