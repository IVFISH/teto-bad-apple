use std::path::Path;
use image::*;

fn load_image(frame: usize) -> ImageResult<> {
    let string_path = format!("video/frames/frame{}.jpg", frame);
    let path = Path::new(&string_path);
    open_gray_image(path)
}