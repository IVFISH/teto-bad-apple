#![allow(unused_imports, dead_code)]

mod analyzer;
mod board;
mod game;
mod piece;
mod bot;
mod queue;
mod control;

use std::collections::HashSet;
use analyzer::*;
use bot::*;
use std::time::Instant;

fn main() {
    let board = to_board(load_image(210));
    println!("{}", board);

    let mut bot = Bot::new(90, 160, 100);
    println!("{}", bot);
    bot.build_pattern(&board);
    println!("{}", bot);

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
    println!("Moving side to side takes {} nanoseconds on average.", time.as_nanos());

}


fn some_frame_thing() {
    for frame in 0..=20 {
        let img = load_image(frame * 30);
        println!("{}", to_board(img));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}