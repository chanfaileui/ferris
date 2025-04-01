pub mod basic;
pub mod complex;
pub mod medium;

use ortalib::{Chips, Edition, Joker, JokerCard, Mult};

use crate::{errors::GameResult, game::GameState};

use crate::explain_dbg_bool;

/// Represents when a joker's effect activates
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivationType {
    Independent, // Activates after all cards are scored
    OnScored,    // Activates when a specific card is scored
    OnHeld,      // Activates based on cards held in hand
}

/// Core trait for all joker effects
pub trait JokerEffect {
    /// The type of activation for this joker
    fn activation_type(&self) -> ActivationType;
    /// Apply the joker's effect to the game state
    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()>;

    /// Optional method for checking if a joker can be applied
    fn can_apply(&self, _game_state: &GameState) -> bool {
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
    explain_enabled: bool,
) -> GameResult<()> {
    match joker_card.edition {
        Some(Edition::Foil) => {
            *chips += 50.0;
            explain_dbg_bool!(
                explain_enabled,
                "{} Foil +50 Chips ({} x {})",
                joker_card.joker,
                chips,
                mult
            );
        }
        Some(Edition::Holographic) => {
            *mult += 10.0;
            explain_dbg_bool!(
                explain_enabled,
                "{} Holographic +10 Mult ({} x {})",
                joker_card.joker,
                chips,
                mult
            );
        }
        Some(Edition::Polychrome) => {
            *mult *= 1.5;
            explain_dbg_bool!(
                explain_enabled,
                "{} Polychrome x1.5 Mult ({} x {})",
                joker_card.joker,
                chips,
                mult
            );
        }
        None => (),
    }
    Ok(())
}

/// Helper function to apply joker effects in the proper order
pub fn process_jokers(game_state: &mut GameState) -> GameResult<()> {
    // Stage 1: Process joker editions (Foil, Holographic) before independent activation
    for joker_card in &game_state.round.jokers {
        if let Some(Edition::Foil) | Some(Edition::Holographic) = joker_card.edition {
            apply_joker_edition(
                joker_card,
                &mut game_state.chips,
                &mut game_state.mult,
                game_state.explain_enabled,
            )?;
        }
    }
    // Stage 2: Process independent jokers
    for joker_card in &game_state.round.jokers.to_vec() {
        let joker_effect = create_joker_effect(joker_card.joker);

        if joker_effect.activation_type() == ActivationType::Independent
            && joker_effect.can_apply(game_state)
        {
            joker_effect.apply(game_state, joker_card)?;
        }
    }
    // Stage 3: Process Polychrome editions after all jokers have been applied
    for joker_card in &game_state.round.jokers {
        if let Some(Edition::Polychrome) = joker_card.edition {
            apply_joker_edition(
                joker_card,
                &mut game_state.chips,
                &mut game_state.mult,
                game_state.explain_enabled,
            )?;
        }
    }

    Ok(())
}
