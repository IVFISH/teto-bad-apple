#![allow(dead_code)]
use crate::board::Board;
use image::*;
use std::path::Path;

pub fn load_image(frame: usize) -> GrayImage {
    let filename = format!("video/frames/frame{}.jpg", frame);
    let path = Path::new(&filename);
    open(&path).unwrap().to_luma8()
}

pub fn to_board(img: GrayImage) -> Board {
    let (width, height, data) = (img.width() as usize, img.height() as usize, img.as_bytes());
    Board::from_vec(
        (0..height)
            .rev()
            .into_iter()
            .map(|h| {
                data[(h * width)..(h * width + width)]
                    .iter()
                    .map(|&x| x > 125)
                    .collect()
            })
            .collect(),
    )
}
