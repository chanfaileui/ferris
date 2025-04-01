pub mod basic;
pub mod complex;
pub mod medium;

use ortalib::{Chips, Edition, Joker, JokerCard, Mult};

use crate::{errors::GameResult, game::GameState};

// pub use self::basic::BasicJoker;
// pub use self::complex::ComplexJoker;
// pub use self::medium::MediumJoker;

// Define any shared traits or types
/// Core trait for all joker effects

/// Represents when a joker's effect activates
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivationType {
    Independent, // Activates after all cards are scored
    OnScored,    // Activates when a specific card is scored
    OnHeld,      // Activates based on cards held in hand
}
pub trait JokerEffect {
    /// The type of activation for this joker
    fn activation_type(&self) -> ActivationType;
    /// Apply the joker's effect to the game state
    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()>;

    /// Optional method for checking if a joker can be applied
    fn can_apply(&self, game_state: &GameState) -> bool {
        true // Default implementation always allows application
    }
}

/// Creates the appropriate joker effect based on joker type
pub fn create_joker_effect(joker: Joker) -> Box<dyn JokerEffect> {
    match joker {
        // Stage 3 - Basic jokers
        Joker::Joker => Box::new(basic::Joker),
        Joker::JollyJoker => Box::new(basic::JollyJoker),
        Joker::ZanyJoker => Box::new(basic::ZanyJoker),
        Joker::MadJoker => Box::new(basic::MadJoker),
        Joker::CrazyJoker => Box::new(basic::CrazyJoker),
        Joker::DrollJoker => Box::new(basic::DrollJoker),
        Joker::SlyJoker => Box::new(basic::SlyJoker),
        Joker::WilyJoker => Box::new(basic::WilyJoker),
        Joker::CleverJoker => Box::new(basic::CleverJoker),
        Joker::DeviousJoker => Box::new(basic::DeviousJoker),
        Joker::CraftyJoker => Box::new(basic::CraftyJoker),
        Joker::AbstractJoker => Box::new(basic::AbstractJoker),

        // Stage 4 - Medium jokers
        Joker::RaisedFist => Box::new(medium::RaisedFist),
        Joker::Blackboard => Box::new(medium::Blackboard),
        Joker::Baron => Box::new(medium::Baron),
        Joker::GreedyJoker => Box::new(medium::GreedyJoker),
        Joker::LustyJoker => Box::new(medium::LustyJoker),
        Joker::WrathfulJoker => Box::new(medium::WrathfulJoker),
        Joker::GluttonousJoker => Box::new(medium::GluttonousJoker),
        Joker::Fibonacci => Box::new(medium::Fibonacci),
        Joker::ScaryFace => Box::new(medium::ScaryFace),
        Joker::EvenSteven => Box::new(medium::EvenSteven),
        Joker::OddTodd => Box::new(medium::OddTodd),
        Joker::Photograph => Box::new(medium::Photograph),
        Joker::SmileyFace => Box::new(medium::SmileyFace),
        Joker::FlowerPot => Box::new(medium::FlowerPot),

        // Stage 5 - Complex jokers
        Joker::FourFingers => Box::new(complex::FourFingers),
        Joker::Shortcut => Box::new(complex::Shortcut),
        Joker::Mime => Box::new(complex::Mime),
        Joker::Pareidolia => Box::new(complex::Pareidolia),
        Joker::Splash => Box::new(complex::Splash),
        Joker::SockAndBuskin => Box::new(complex::SockAndBuskin),
        Joker::SmearedJoker => Box::new(complex::SmearedJoker),
        Joker::Blueprint => Box::new(complex::Blueprint),
    }
}

/// Processes joker editions
pub fn apply_joker_edition(
    joker_card: &JokerCard,
    chips: &mut Chips,
    mult: &mut Mult,
) -> GameResult<Vec<String>> {
    let mut explanations = Vec::new();

    match joker_card.edition {
        Some(Edition::Foil) => {
            *chips += 50.0;
            explanations.push(format!(
                "{} Foil +50 Chips ({} x {})",
                joker_card.joker, chips, mult
            ));
        }
        Some(Edition::Holographic) => {
            *mult += 10.0;
            explanations.push(format!(
                "{} Holographic +10 Mult ({} x {})",
                joker_card.joker, chips, mult
            ));
        }
        Some(Edition::Polychrome) => {
            *mult *= 1.5;
            explanations.push(format!(
                "{} Polychrome x1.5 Mult ({} x {})",
                joker_card.joker, chips, mult
            ));
        }
        None => (),
    }
    Ok(explanations)
}

/// Helper function to apply joker effects in the proper order
pub fn process_jokers(game_state: &mut GameState) -> GameResult<Vec<String>> {
    let mut explanations = Vec::new();

    // Stage 1: Process joker editions (Foil, Holographic) before independent activation
    for joker_card in &game_state.round.jokers {
        if let Some(Edition::Foil) | Some(Edition::Holographic) = joker_card.edition {
            let joker_edition_explanations =
                apply_joker_edition(joker_card, &mut game_state.chips, &mut game_state.mult)?;
            explanations.extend(joker_edition_explanations);
        }
    }

    // Stage 2: Process independent jokers
    let joker_cards = game_state.round.jokers.clone(); // Clone the entire joker collection once
    for joker_card in &joker_cards {
        let joker_effect = create_joker_effect(joker_card.joker);

        let can_apply = joker_effect.activation_type() == ActivationType::Independent
            && joker_effect.can_apply(&game_state);
        if can_apply {
            let joker_effect_explanations = joker_effect.apply(game_state, joker_card)?;
            explanations.extend(joker_effect_explanations);
        }
    }

    // Stage 3: Process Polychrome editions after all jokers have been applied
    for joker_card in &game_state.round.jokers {
        if let Some(Edition::Polychrome) = joker_card.edition {
            let joker_edition_explanations =
                apply_joker_edition(joker_card, &mut game_state.chips, &mut game_state.mult)?;
            explanations.extend(joker_edition_explanations);
        }
    }

    Ok(explanations)
}
