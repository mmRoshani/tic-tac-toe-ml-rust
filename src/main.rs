mod engine;
use image;
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;
use slint::VecModel;
use slint::{Image, Model};
use std::rc::Rc;

slint::include_modules!();

slint::slint! {
    import {StandardButton, ProgressIndicator} from "std-widgets.slint";
    DialogBox := Dialog {
        property <string> message_text: "Default message";
        Text {
            text: root.message_text;
        }
    }
}
slint::slint! {
    import { ProgressIndicator} from "std-widgets.slint";

    DialogScrollable := Dialog {
        in property <float> on_going: 0/100;
        width: 200px;
        height: 25px;
        ProgressIndicator {
            progress: on_going;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut threshold: i32 = 0;
    if args.len() > 1 {
        threshold = args[1].parse::<i32>().unwrap();
    }

    println!(
        "Secondry agent starting to run for threshold of: {}",
        threshold
    );

    // run ui
    let ui = MainWindow::new().unwrap();

    let tiles: Vec<TileData> = ui.get_memory_tiles().iter().collect();
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));

    ui.set_memory_tiles(tiles_model.clone().into());

    let ui_handle = ui.as_weak();

    // game initializtion
    let mut board = engine::ml::Board::new(); // this agent is playing `O`
    let mut secondary_board = engine::ml::Board::new(); // that agent is playing `X`
                                                        // Open dialog
    let progress_dialog = DialogScrollable::new().expect("some error");
    let _ = progress_dialog.show();

    while ((board.x_o_winning_state.1 + 1) + (secondary_board.x_o_winning_state.0 + 1)) <= threshold
    {
        // set the progerss
        let progress: f32 =
            ((board.x_o_winning_state.1 + 1 + secondary_board.x_o_winning_state.0 + 1) / threshold)
                as f32
                * 100.0;
        progress_dialog.set_on_going(progress);
        println!(">>>>>>>{:?}>>>>>>>", progress);
        // playing `O`
        let first_agent_move = board.computer_move();
        // let first_agent_tile_idx = calculate_tile_index(first_agent_move);
        board.moves_played += 1;
        if board.moves_played == 9 {
            board.x_o_winning_state =
                (board.x_o_winning_state.0 + 1, board.x_o_winning_state.1 + 1);
            board.moves_played = 0;
            continue;
        } else if board.winning_lines('O') {
            board.x_o_winning_state = (board.x_o_winning_state.0, board.x_o_winning_state.1 + 1);
            board.moves_played = 0;
            continue;
        }
        secondary_board.state[first_agent_move.0][first_agent_move.1];
        // let seconf_agent_move = calculate_tile_index(secondary_board.computer_move());
        if secondary_board.moves_played == 9 {
            secondary_board.x_o_winning_state = (
                secondary_board.x_o_winning_state.0 + 1,
                secondary_board.x_o_winning_state.1 + 1,
            );
            secondary_board.moves_played = 0;
            continue;
        } else if secondary_board.winning_lines('X') {
            secondary_board.x_o_winning_state = (
                secondary_board.x_o_winning_state.0 + 1,
                secondary_board.x_o_winning_state.1,
            );
            secondary_board.moves_played = 0;

            continue;
        }
    }

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
        board.x_o_winning_state = (board.x_o_winning_state.0 + 1, board.x_o_winning_state.1);
        board.update_function(Some('X'));
        let dialog = DialogBox::new().expect("some error");
        dialog.set_message_text(
            format!(
                "You won, your score is: {} and computer socre is: {}",
                board.x_o_winning_state.0, board.x_o_winning_state.1
            )
            .into(),
        );
        let _ = dialog.run();
        ui.set_disable_tiles(true);
        return (10, 10);
    } else if board.moves_played == 9 {
        board.x_o_winning_state = (board.x_o_winning_state.0 + 1, board.x_o_winning_state.1 + 1);
        board.update_function(Some('-'));
        let dialog = DialogBox::new().expect("some error");
        dialog.set_message_text(
            format!(
                "Equal game, good job; your score is: {} and computer socre is: {}",
                board.x_o_winning_state.0, board.x_o_winning_state.1
            )
            .into(),
        );
        let _ = dialog.run();
        ui.set_disable_tiles(true);
        return (10, 10);
    }

    board.moves_played += 1;

    // do the movemnt
    let computer_move = board.computer_move();

    if board.winning_lines('O') {
        board.x_o_winning_state = (board.x_o_winning_state.0, board.x_o_winning_state.1 + 1);
        board.update_function(Some('O'));
        let dialog = DialogBox::new().expect("some error");
        dialog.set_message_text(
            format!(
                "Computer won ;), your score is: {} and computer socre is: {}",
                board.x_o_winning_state.0, board.x_o_winning_state.1
            )
            .into(),
        );
        let _ = dialog.run();
        ui.set_disable_tiles(true);
        return (10, 10);
    } else if board.moves_played == 9 {
        board.x_o_winning_state = (board.x_o_winning_state.0 + 1, board.x_o_winning_state.1 + 1);
        board.update_function(Some('-'));
        let dialog = DialogBox::new().expect("some error");
        dialog.set_message_text(
            format!(
                "Equal game, good job; your score is: {} and computer socre is: {}",
                board.x_o_winning_state.0, board.x_o_winning_state.1
            )
            .into(),
        );
        let _ = dialog.run();
        ui.set_disable_tiles(true);
        return (10, 10);
    }

    return computer_move;
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
        board.moves_played = 0;
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

fn calculate_tile_index(machine_move: (usize, usize)) -> usize {
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
        unimplemented!();
    }

    tile_idx
}
