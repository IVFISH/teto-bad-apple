#![allow(unused_imports, dead_code)]

mod analyzer;
mod board;
mod game;
mod piece;
mod bot;
mod queue;
mod control;

use analyzer::*;
use bot::*;
use std::time::Instant;

fn main() {
    // move_benchmark()

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