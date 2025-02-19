use bmp::consts;
use std::env;

fn main() {
    // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    for filepath in &args[1..] { // using the concept of borrowing here
        println!("===== {filepath} =====");
    
        let img = match bmp::open(filepath) {
            Ok(img) => img,
            Err(e) => {
                println!("Error! {:?}", e);
                continue
            }
        };

        for (x, y) in img.coordinates() {
            let pix = img.get_pixel(x, y);
            if pix == consts::RED {
                print!("R ");
            } else if pix == consts::LIME {
                print!("G ");
            } else if pix == consts::BLUE {
                print!("B ");
            } else if pix == consts::WHITE {
                print!("W ");
            } else {
                print!("? ");
            }
            if x == img.get_width() - 1 {
                println!();
            }
        }
    }
}
