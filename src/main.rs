#![allow(unused_imports, dead_code)]

mod analyzer;
mod board;
mod bot;
mod control;
mod game;
mod piece;
mod queue;

use analyzer::*;
use bot::*;
use std::collections::HashSet;
use std::time::Instant;
use crate::control::{Executable, PlacementActions};

fn main() {
    // let board = to_board(load_image(210));
    // println!("{}", board);
    //
    // let mut bot = Bot::new(40, 40, 100);
    // println!("{:?}", bot.game.active);
    // bot.build_row(&board, 0);
    // bot.build_pattern(&board);
    // println!("{}", bot);

    let mut bot = Bot::new(20, 10, 100);
    let out = bot.dfs();
    println!("{}", out.len());
    for PlacementActions {placement: _, mut batch} in out {
        bot.action(batch.into());
        println!("{}", bot);
        bot.undo();
    }
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
