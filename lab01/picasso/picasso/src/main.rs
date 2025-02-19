use bmp::consts;
use std::env;

fn main() {
    // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    for filepath in &args[1..] { // using the concept of borrowing here
        // let filepath = &std::env::args()
        //     .nth(1)
        //     .expect("missing required command-line argument: <filepath>");

        println!("===== {filepath} =====");
    
        let img = match bmp::open(filepath) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Error! BmpError {:?}", e);
                continue
            }
        };

        for (x, y) in img.coordinates() {
            let pix = img.get_pixel(x, y);
            match pix {
                consts::RED => print!("R "),
                consts::LIME => print!("G "),
                consts::BLUE => print!("B "),
                consts::WHITE => print!("W "),
                e => panic!("{}", e)
            }
            if x == img.get_width() - 1 {
                println!();
            }
        }
    }
}
