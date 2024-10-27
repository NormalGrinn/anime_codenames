use image::{RgbImage, Rgb};
use super::types;

pub fn create_image(cards: Vec<types::Card>) {
    let cell_size = 200;
    let grid_size = 5;
    let img_width = cell_size * grid_size;
    let img_height = cell_size * grid_size;

    let mut img: RgbImage = RgbImage::new(img_width, img_height);
    
}