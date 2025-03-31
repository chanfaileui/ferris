pub mod basic;
pub mod complex;
pub mod medium;

use ortalib::{Chips, Mult};

use crate::errors::GameResult;

// pub use self::basic::BasicJoker;
// pub use self::complex::ComplexJoker;
// pub use self::medium::MediumJoker;

// Define any shared traits or types
pub trait JokerEffect {
    // Applies the joker effect to the chips and mult
    fn apply(&self, chips: &mut Chips, mult: &mut Mult) -> GameResult<Vec<String>>;

    // Returns the explanation for the joker
    fn get_explanation(&self) -> Vec<String>;

    // Returns the type of activation for the joker
    fn activation_type(&self) -> ActivationType;
}

pub enum ActivationType {
    Independent, // Activates after all cards are scored
    OnScored,    // Activates when a specific card is scored
    OnHeld,      // Activates based on cards held in hand
}
