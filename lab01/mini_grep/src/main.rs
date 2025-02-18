fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string; // Immutable borrow - you can only read
                                   // this is different to C's address operator

    loop  {
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);
        if line.is_empty() {
            return;
        }
        if line.contains(pattern) {
            print!("{line}");
        } 
    } 
}
