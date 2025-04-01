// src/jokers/complex.rs
use crate::errors::GameResult;
use crate::game::GameState;
use crate::jokers::ActivationType;
use crate::jokers::JokerEffect;
use crate::jokers::create_joker_effect;
use ortalib::{Joker, JokerCard};

use crate::explain_dbg;

// All Flushes and Straights can be made with 4 cards
pub struct FourFingers;

impl JokerEffect for FourFingers {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        // This effect is passive and is handled during hand identification
        // Update the game state to indicate this joker is active
        game_state.four_fingers_active = true;

        // No direct scoring impact, just provides an explanation
        explain_dbg!(
            game_state,
            "{} allows Flushes and Straights with 4 cards",
            joker_card.joker
        );
        Ok(())
    }
}

// Allows Straights to be made with gaps of 1 rank
pub struct Shortcut;

impl JokerEffect for Shortcut {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        // This effect is passive and is handled during hand identification
        // Update the game state to indicate this joker is active
        game_state.shortcut_active = true;

        // No direct scoring impact, just provides an explanation
        explain_dbg!(
            game_state,
            "{} allows Straights with gaps of 1 rank",
            joker_card.joker
        );
        Ok(())
    }
}

// Retrigger all card held in hand abilities
pub struct Mime;

impl JokerEffect for Mime {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnHeld
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        // Mark for retrigger rather than directly applying effects
        // The actual retrigger will happen in the game scoring logic
        game_state.mime_retriggers += 1;

        explain_dbg!(
            game_state,
            "{} retriggers all card held in hand abilities",
            joker_card.joker
        );
        Ok(())
    }
}

// All cards are considered face cards
pub struct Pareidolia;

impl JokerEffect for Pareidolia {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        // This effect is passive and is handled during scoring
        // Update the game state to indicate this joker is active
        game_state.pareidolia_active = true;

        // No direct scoring impact, just provides an explanation
        explain_dbg!(
            game_state,
            "{} makes all cards count as face cards",
            joker_card.joker
        );
        Ok(())
    }
}

// Every played card counts in scoring
pub struct Splash;

impl JokerEffect for Splash {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        // This effect is passive and is handled during scoring card selection
        // Update the game state to indicate this joker is active
        game_state.splash_active = true;

        // No direct scoring impact, just provides an explanation
        explain_dbg!(
            game_state,
            "{} makes every played card count in scoring",
            joker_card.joker
        );
        Ok(())
    }
}

// Retrigger all scoring face cards
pub struct SockAndBuskin;

impl JokerEffect for SockAndBuskin {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        // Mark for retrigger rather than directly applying effects
        // The actual retrigger will happen in the game scoring logic
        game_state.sock_and_buskin_retriggers += 1;

        explain_dbg!(
            game_state,
            "{} retriggers all scoring face cards",
            joker_card.joker
        );
        Ok(())
    }
}

// ♥Hearts and ♦Diamonds count as the same suit, ♠Spades and ♣Clubs count as the same suit
pub struct SmearedJoker;

impl JokerEffect for SmearedJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        // This effect is passive and is handled during hand identification
        // Update the game state to indicate this joker is active
        game_state.smeared_joker_active = true;

        // No direct scoring impact, just provides an explanation
        explain_dbg!(
            game_state,
            "{} makes cards of the same color count as the same suit",
            joker_card.joker
        );
        Ok(())
    }
}

// Copies the ability of Joker to the right (i.e. below)
pub struct Blueprint;

impl JokerEffect for Blueprint {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        // Find the joker to the right
        let joker_index =
            game_state.round.jokers.iter().position(|j| {
                j.joker == Joker::Blueprint && j as *const _ == joker_card as *const _
            });

        if let Some(index) = joker_index {
            if index < game_state.round.jokers.len() - 1 {
                // There is a joker to the right
                let next_joker = game_state.round.jokers[index + 1];
                let next_joker_effect = create_joker_effect(next_joker.joker);

                // Check if the next joker is compatible (not a passive modifier)
                if next_joker_effect.activation_type() != ActivationType::Independent
                    || next_joker.joker != Joker::FourFingers
                        && next_joker.joker != Joker::Shortcut
                        && next_joker.joker != Joker::Pareidolia
                        && next_joker.joker != Joker::Splash
                        && next_joker.joker != Joker::SmearedJoker
                {
                    // Apply the copied joker effect
                    next_joker_effect.apply(game_state, joker_card)?;
                    explain_dbg!(
                        game_state,
                        "{} copies ability of {}",
                        joker_card.joker,
                        next_joker.joker
                    );
                    return Ok(());
                }
            }
        }

        // No joker to the right or incompatible joker
        explain_dbg!(
            game_state,
            "{} has no effect (no compatible joker to copy)",
            joker_card.joker
        );
        Ok(())
    }
}
