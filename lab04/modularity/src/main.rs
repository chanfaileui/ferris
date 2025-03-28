use modularity::compute_tribonacci;
use modularity::*;

fn main() {
    let shift_size = utils::first_argument();

    if let Err(e) = compute_tribonacci(shift_size) {
        println!("Error: {}", e.0)
    }
}
