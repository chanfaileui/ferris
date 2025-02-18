use bmp::{Image, Pixel};

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");

    // // if doesn't exist
    // if fs::File::open(operation.as_str()).is_ok() {
    //     // if path exists
        
    // } else {
        
    // }

    bmp::open(path.as_str()).unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } 
    else if operation.as_str() == "single" {
        single_pixel(path.as_str())
    } 
    else if operation.as_str() == "diagonal" {
        diagonal_line(path.as_str())
    } else if operation.as_str() == "house" {
        draw_house(path.as_str())
    }
    else {
        eprintln!("The operation {operation} was not recognised!");
    }
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly.");
}

fn single_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(5, 5, bmp::consts::GOLDENROD);

    match image.save(path) {
        Ok(_) => return,
        Err(_) => println!("Encountered some error :(")
    }
}

fn diagonal_line(path: &str) {
    let mut image = Image::new(100, 100);

    // if (width != height) {
    //     Err("Not square!")
    // }

    for (x, y) in image.coordinates() {
        if x == y {
            image.set_pixel(x, y, bmp::consts::GOLDENROD);
        }
    }
    
    match image.save(path) {
        Ok(_) => return,
        Err(_) => println!("Encountered some error :(")
    }
}

fn draw_house(path: &str) {
    let mut image = Image::new(100, 100);

    for (x, y) in image.coordinates() {
        if x == y && x < 50 && y < 50 {
            image.set_pixel(x, y, bmp::consts::GOLDENROD);
        }
        if x == y && x >= 50 && y < 50 {
            image.set_pixel(x, y, bmp::consts::GOLDENROD);
        }
    }
    
    match image.save(path) {
        Ok(_) => return,
        Err(_) => println!("Encountered some error :(")
    }
}
