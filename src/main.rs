#![allow(unused_imports, dead_code)]

mod analyzer;
mod board;
mod bot;
mod control;
mod game;
mod piece;
mod queue;

use crate::control::{Command, Executable, PlacementActions};
use board::Board;
use analyzer::*;
use bot::*;
use std::collections::HashSet;
use std::time::{Duration, Instant};

fn l_spin_bot() -> Bot {
    let mut bot = Bot::new(20, 10, 43);
    let board = vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (11, 0),
        (12, 0),
        (13, 0),
        (14, 0),
        (4, 1),
        (5, 1),
        (6, 1),
        (7, 1),
        (8, 1),
        (9, 1),
        (10, 1),
        (11, 1),
        (12, 1),
        (14, 1),
        (1, 2),
        (2, 2),
        (5, 2),
        (6, 2),
        (7, 2),
        (8, 2),
        (9, 2),
        (0, 3),
        (1, 3),
        (6, 3),
        (7, 3),
        (8, 3),
        (9, 3),
        (11, 3),
        (12, 3),
        (0, 4),
        (1, 4),
        (3, 4),
        (4, 4),
        (6, 4),
        (9, 4),
        (12, 4),
        (0, 5),
        (1, 5),
        (2, 5),
        (3, 5),
        (4, 5),
        (12, 5),
        (0, 6),
        (1, 6),
        (2, 6),
        (3, 6),
        (4, 6),
        (5, 6),
        (6, 6),
        (7, 6),
        (9, 6),
        (10, 6),
        (11, 6),
        (12, 6),
        (0, 7),
        (1, 7),
        (2, 7),
        (3, 7),
        (4, 7),
        (5, 7),
        (6, 7),
        (7, 7),
        (9, 7),
        (10, 7),
        (11, 7),
        (12, 7),
        (0, 8),
        (1, 8),
        (2, 8),
        (3, 8),
        (4, 8),
        (5, 8),
        (6, 8),
        (7, 8),
        (8, 8),
        (9, 8),
        (10, 8),
        (11, 8),
        (12, 8),
        (0, 9),
        (1, 9),
        (2, 9),
        (3, 9),
        (4, 9),
        (5, 9),
        (6, 9),
        (7, 9),
        (8, 9),
        (9, 9),
        (10, 9),
        (11, 9),
        (12, 9),
    ];
    bot.game.board.bulk_add(board);
    bot
}

fn l_spin_bot_2() -> Bot {
    // requires an I piece first
    let mut bot = l_spin_bot();
    bot.game.board.bulk_add(vec![
        (13, 1),
        (13, 2),
        (13, 3),
        (13, 4),
        (13, 5),
        (15, 0),
        (15, 1),
    ]);
    bot.game.board.remove(14, 1);
    bot.game.queue.pieces.push_front(4);
    bot.game.active = bot.game.new_piece(1);
    bot
}

// fn move_gen_test() {
//     let mut bot = l_spin_bot_2();
//     println!("{}", bot);
//
//     // let now = Instant::now();
//     let actions = bot.look_ahead(2, 5, Board::new(10, 20));
//     // println!("{}", now.elapsed().as_millis());
//     for action in actions {
//         // println!("{:#?}", action.batch.commands);
//         bot.action(action.batch.into());
//         if bot.game.active.row < 5 {
//             println!("{}", bot);
//         }
//         // std::thread::sleep(Duration::from_millis(200));
//         bot.undo();
//     }
// }

fn test() {
    let width = 40;
    let height = 40;

    let goal = to_board(load_image(330)).arr[0].clone();
    let mut bot = Bot::new(height, width, 1);
    let mut board = Board::new(height, width);
    board.arr[0] = goal;

    println!("{}", board);

    // let actions = bot.look_ahead(3, 3, board);
    // println!("{}", actions.len());
    // for action in actions {
    //     bot.action(action.into());
    //     println!("{}", bot);
    //     bot.undo();
    // }

    for _ in 0..20 {
        let mut action = bot.best_action(3, 3, &board).unwrap();
        // let action = action.batch.commands.front().unwrap().clone();
        bot.action(action.into());
        println!("{}", bot);
        break;
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    test()
    // let board = to_board(load_image(210));
    // println!("{}", board);
    //
    // let mut bot = Bot::new(40, 40, 100);
    // println!("{:?}", bot.game.active);
    // bot.build_row(&board, 0);
    // bot.build_pattern(&board);
    // println!("{}", bot);

    // let mut bot = Bot::new(20, 10, 100);
    // let out = bot.dfs();
    // println!("{}", out.len());
    // for PlacementActions { placement: _, mut batch } in out {
    //     bot.action(batch.into());
    //     println!("{}", bot);
    //     bot.undo();
    // }

    // move_gen_test()
    // control_test()
    // move_benchmark()
}

fn control_test() {
    let mut bot = Bot::new(20, 10, 100);

    // movement test
    // bot.move_left();
    // println!("{}", bot);
    // bot.rotate_cw();
    // println!("{}", bot);
    // bot.soft_drop();
    // println!("{}", bot);
    // bot.rotate_cw();
    // println!("{}", bot);
    // bot.hard_drop();
    // println!("{}", bot);
    //
    // bot.undo();
    // println!("{}", bot);
    // bot.undo();
    // println!("{}", bot);
    // bot.undo();
    // println!("{}", bot);
    // bot.undo();
    // println!("{}", bot);
    // bot.undo();
    // println!("{}", bot);

    // hold test
    bot.hold();
    println!("{}", bot);
    bot.undo();
    println!("{}", bot);
    bot.hold();
    bot.hard_drop();
    println!("{}", bot);
    bot.hold();
    println!("{}", bot);
    bot.undo();
    println!("{}", bot);
}

fn move_benchmark() {
    let num_iterations = 10000000;
    let mut bot = Bot::new(20, 10, 100);

    let now = Instant::now();
    for _ in 0..num_iterations {
        bot.move_left();
        bot.move_right();
    }

    let elapsed = now.elapsed();
    let time = elapsed / (num_iterations * 2);
    println!(
        "Moving side to side takes {} nanoseconds on average.",
        time.as_nanos()
    );
}

fn some_frame_thing() {
    for frame in 0..=20 {
        let img = load_image(frame * 30);
        println!("{}", to_board(img));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
