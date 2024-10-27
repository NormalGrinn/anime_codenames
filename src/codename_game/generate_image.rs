use image::{RgbImage, Rgb};
use ab_glyph::{FontArc, PxScale};
use super::types;

pub fn create_image(cards: Vec<types::Card>) {
    let cell_size: i32 = 400;
    let grid_size: i32 = 5;
    let img_width: u32 = (cell_size * grid_size).try_into().unwrap();
    let img_height: u32 = (cell_size * grid_size).try_into().unwrap();

    let mut img: RgbImage = RgbImage::new(img_width, img_height);
    let white = Rgb([255, 255, 255]);

    let font = FontArc::try_from_slice(include_bytes!("../../fonts/pokemon-emerald.ttf")).expect("Error loading font");

    let scale = PxScale::from(20.0);

    let mut i = 0;
    for row in 0..grid_size {
        for col in 0..grid_size {
            let text = format!("{}", cards[i].names);
            i += 1;

            let x: i32 = col * cell_size + 10;
            let y: i32 = row * cell_size + 30;

            imageproc::drawing::draw_text_mut(&mut img, white, x, y, scale, &font, &text);
        }
    }
    img.save("output.png").expect("Failed to save image")
}