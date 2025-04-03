// src/jokers/complex.rs
use crate::errors::GameResult;
use crate::game::GameState;
use crate::jokers::ActivationType;
use crate::jokers::JokerEffect;
use crate::jokers::create_joker_effect;
use ortalib::Card;
use ortalib::Joker;
use ortalib::JokerCard;
use ortalib::Rank;
use ortalib::Suit;

use crate::explain_dbg;

// All Flushes and Straights can be made with 4 cards
pub struct FourFingers;

impl JokerEffect for FourFingers {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
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

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
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

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
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

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
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

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
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

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
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

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
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

/// Copies the ability of Joker to the right (i.e. below)
pub struct Blueprint;

fn follow_blueprint_chain(game_state: &GameState, start_index: usize) -> Option<(usize, Joker)> {
    let jokers = &game_state.round.jokers;

    // Start with the joker to the right of the Blueprint
    let mut current_index = start_index + 1;

    // Follow the chain of Blueprints
    while current_index < jokers.len() {
        let current_joker = jokers[current_index].joker;

        // If not a Blueprint, we found the target
        if current_joker != Joker::Blueprint {
            return Some((current_index, current_joker));
        }

        // Move to the next joker
        current_index += 1;
    }

    // Reached the end without finding a non-Blueprint
    None
}

impl JokerEffect for Blueprint {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
        // Find this blueprint's position in the jokers list
        if let Some(joker_index) = game_state.round.jokers.iter().position(|j| j == joker_card) {
            if let Some((_target_index, target_joker)) =
                follow_blueprint_chain(game_state, joker_index)
            {
                // Get the target joker's effect
                let effect = create_joker_effect(target_joker);

                // Create a placeholder card for potential use
                let placeholder_card = Card::new(Rank::Ace, Suit::Diamonds, None, None);

                // Handle different activation types
                match effect.activation_type() {
                    ActivationType::Independent => {
                        // Skip incompatible passive jokers
                        if target_joker != Joker::FourFingers
                            && target_joker != Joker::Shortcut
                            && target_joker != Joker::Pareidolia
                            && target_joker != Joker::Splash
                            && target_joker != Joker::SmearedJoker
                            && effect.can_apply(game_state)
                        {
                            effect.apply(game_state, joker_card, &placeholder_card)?;

                            explain_dbg!(
                                game_state,
                                "{} copies ability of {}",
                                joker_card.joker,
                                target_joker
                            );
                            return Ok(());
                        }
                    }
                    ActivationType::OnScored => {
                        game_state
                            .blueprint_copied_jokers
                            .push((*joker_card, target_joker));

                        explain_dbg!(
                            game_state,
                            "{} will copy OnScored ability of {}",
                            joker_card.joker,
                            target_joker
                        );
                        return Ok(());
                    }
                    ActivationType::OnHeld => {
                        game_state
                            .blueprint_held_jokers
                            .push((*joker_card, target_joker));

                        explain_dbg!(
                            game_state,
                            "{} will copy OnHeld ability of {}",
                            joker_card.joker,
                            target_joker
                        );
                        return Ok(());
                    }
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
