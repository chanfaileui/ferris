#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

fn main() {
    let mut img = Image::new(256, 256);
    
    let square_size = 32;
    for (x, y) in img.coordinates() {        
        let is_black_square = ((x / square_size) + (y / square_size)) % 2 == 0;

        if is_black_square {
            img.set_pixel(x, y, px!(x, y, 200));
        } else {
            img.set_pixel(x, y, px!(255, 255, 255));
        }
    }
    
    let _ = img.save("img.bmp");
}