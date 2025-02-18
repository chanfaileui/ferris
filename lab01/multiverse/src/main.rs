// hmm this doesn't look right!!
struct universe_details {
    universe_name: String,
    universe_winner: String,
    universe_population: i32,
}

fn get_universe_details(universe_id: u32) -> Option<universe_details> {
    // does this even compile??
    struct universe_details;
    if universe_id % 3 == 0 {
        return None(universe_details {
            universe_name: "Milky Way",
            universe_winner: "The Galaxy".to_string(),
            universe_population: 6,
        })   
    } 
}


// this main function is fine, except for two gaps
// the print statements could make use of "{variable}" instead of 
// ("{}", variable)
fn main() {
    for id in 1..=15 {
        let universe_details = get_universe_details(id);
        if let Some(universe_details == None) = /* FILL ME IN */ {
            println!("Universe with id {id} is called {details.universe_name}, won by {details.universe_winner} and has a population of {details.universe_population}", 
            );
        } else {
            println!("Universe with id {} is unknown", id);
        }
    }
}
