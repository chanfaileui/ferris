// src/jokers/medium.rs
use crate::errors::GameResult;
use crate::game::GameState;
use crate::jokers::ActivationType;
use crate::jokers::JokerEffect;
use ortalib::Card;
use ortalib::{Enhancement, JokerCard, Rank, Suit};

use crate::explain_dbg;

// Adds double the rank value of the lowest card held in hand to ✖ Mult
pub struct RaisedFist;

impl JokerEffect for RaisedFist {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnHeld
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        current_card: &Card,
    ) -> GameResult<()> {
        let rank_value = current_card.rank.rank_value();
        let mult_increase = 2.0 * rank_value;
        game_state.mult += mult_increase;

        let message = format!(
            "{} {} +{} Mult ({} x {})",
            joker_card.joker, current_card, mult_increase, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// ✖ Mult ×3 if all cards held in hand are ♠Spades or ♣Clubs
pub struct Blackboard;

impl JokerEffect for Blackboard {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        // If no cards in hand, effect applies
        if game_state.round.cards_held_in_hand.is_empty() {
            return true;
        }

        // Check if all cards are spades or clubs (or wild)
        game_state.round.cards_held_in_hand.iter().all(|card| {
            card.suit == Suit::Spades
                || card.suit == Suit::Clubs
                || card.enhancement == Some(Enhancement::Wild)
        })
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
        game_state.mult *= 3.0;
        let message = format!(
            "{} x3 Mult ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// Each King held in hand gives ✖ Mult ×1.5
pub struct Baron;

impl JokerEffect for Baron {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnHeld
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
        // Find all kings in hand
        for card in &game_state.round.cards_held_in_hand {
            if card.rank == Rank::King {
                game_state.mult *= 1.5;
                let message = format!(
                    "{} {} x1.5 Mult ({} x {})",
                    joker_card.joker, card, game_state.chips, game_state.mult
                );
                explain_dbg!(game_state, "{}", message);
            }
        }

        Ok(())
    }
}

// ✖ Mult +3 for each ♦Diamonds card played
pub struct GreedyJoker;

impl JokerEffect for GreedyJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
        // Count diamonds cards (including wild)
        let diamond_count = game_state
            .scoring_cards
            .iter()
            .filter(|card| {
                card.suit == Suit::Diamonds || card.enhancement == Some(Enhancement::Wild)
            })
            .count();

        if diamond_count > 0 {
            let mult_increase = 3.0 * (diamond_count as f64);
            game_state.mult += mult_increase;
            let message = format!(
                "{} +{} Mult ({} x {})",
                joker_card.joker, mult_increase, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }

        Ok(())
    }
}

// ✖ Mult +3 for each ♥Hearts card played
pub struct LustyJoker;

impl JokerEffect for LustyJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
        // Count hearts cards (including wild)
        let heart_count = game_state
            .scoring_cards
            .iter()
            .filter(|card| card.suit == Suit::Hearts || card.enhancement == Some(Enhancement::Wild))
            .count();

        if heart_count > 0 {
            let mult_increase = 3.0 * (heart_count as f64);
            game_state.mult += mult_increase;
            let message = format!(
                "{} +{} Mult ({} x {})",
                joker_card.joker, mult_increase, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }

        Ok(())
    }
}

// ✖ Mult +3 for each ♠Spades card played
pub struct WrathfulJoker;

impl JokerEffect for WrathfulJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
        // Count spades cards (including wild)
        let spade_count = game_state
            .scoring_cards
            .iter()
            .filter(|card| card.suit == Suit::Spades || card.enhancement == Some(Enhancement::Wild))
            .count();

        if spade_count > 0 {
            let mult_increase = 3.0 * (spade_count as f64);
            game_state.mult += mult_increase;
            let message = format!(
                "{} +{} Mult ({} x {})",
                joker_card.joker, mult_increase, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }

        Ok(())
    }
}

// ✖ Mult +3 for each ♣Clubs card played
pub struct GluttonousJoker;

impl JokerEffect for GluttonousJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        current_card: &Card,
    ) -> GameResult<()> {
        if current_card.suit == Suit::Clubs || current_card.enhancement == Some(Enhancement::Wild) {
            game_state.mult += 3.0;
            let message = format!(
                "{} {} +3 Mult ({} x {})",
                joker_card.joker, current_card, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }

        Ok(())
    }
}

// Each played Ace, 2, 3, 5, or 8 gives ✖ Mult +8 when scored
pub struct Fibonacci;

impl JokerEffect for Fibonacci {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
        // Check each scoring card for fibonacci numbers
        for card in &game_state.scoring_cards {
            if card.rank == Rank::Ace
                || card.rank == Rank::Two
                || card.rank == Rank::Three
                || card.rank == Rank::Five
                || card.rank == Rank::Eight
            {
                game_state.mult += 8.0;
                let message = format!(
                    "{} {} +8 Mult ({} x {})",
                    joker_card.joker, card, game_state.chips, game_state.mult
                );
                explain_dbg!(game_state, "{}", message);
            }
        }

        Ok(())
    }
}

// Played face cards give Chips +30 when scored
pub struct ScaryFace;

impl JokerEffect for ScaryFace {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        current_card: &Card,
    ) -> GameResult<()> {
        let is_face = game_state.pareidolia_active || current_card.rank.is_face();

        if is_face {
            game_state.chips += 30.0;
            let message = format!(
                "{} {} +30 Chips ({} x {})",
                joker_card.joker, current_card, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }

        Ok(())
    }
}

pub struct EvenSteven;

// Played even-ranked cards give Mult +4 when scored (10, 8, 6, 4, 2)
impl JokerEffect for EvenSteven {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        current_card: &Card,
    ) -> GameResult<()> {
        let is_even_rank = matches!(
            current_card.rank,
            Rank::Ten | Rank::Eight | Rank::Six | Rank::Four | Rank::Two
        );

        if is_even_rank {
            game_state.mult += 4.0;
            let message = format!(
                "{} {} +4 Mult ({} x {})",
                joker_card.joker, current_card, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }

        Ok(())
    }
}

pub struct OddTodd;

// Played odd-ranked cards give Mult +4 when scored (A, 9, 7, 5, 3)
impl JokerEffect for OddTodd {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        current_card: &Card,
    ) -> GameResult<()> {
        let is_odd_rank = matches!(
            current_card.rank,
            Rank::Ace | Rank::Nine | Rank::Seven | Rank::Five | Rank::Three
        );

        if is_odd_rank {
            game_state.chips += 31.0;
            let message = format!(
                "{} {} +31 Chips ({} x {})",
                joker_card.joker, current_card, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }

        Ok(())
    }
}

pub struct Photograph;

// First scoring face card gives Mult ×2 when scored
impl JokerEffect for Photograph {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        current_card: &Card,
    ) -> GameResult<()> {
        let is_face = game_state.pareidolia_active || current_card.rank.is_face();

        if is_face && !game_state.first_face_card_processed {
            game_state.mult *= 2.0;
            game_state.first_face_card_processed = true;
            let message = format!(
                "{} {} x2 Mult ({} x {})",
                joker_card.joker, current_card, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }
        Ok(())
    }
}

pub struct SmileyFace;

// Played face cards give Mult +5 when scored
impl JokerEffect for SmileyFace {
    fn activation_type(&self) -> ActivationType {
        ActivationType::OnScored
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        current_card: &Card,
    ) -> GameResult<()> {
        let is_face = game_state.pareidolia_active || current_card.rank.is_face();

        if is_face {
            game_state.mult += 5.0;
            let message = format!(
                "{} {} +5 Mult ({} x {})",
                joker_card.joker, current_card, game_state.chips, game_state.mult
            );
            explain_dbg!(game_state, "{}", message);
        }

        Ok(())
    }
}
pub struct FlowerPot;

// ✖ Mult ×3 if scoring cards contain a ♦Diamonds, ♣Clubs, ♥Hearts, and ♠Spades card
impl JokerEffect for FlowerPot {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        // Need to have at least 4 scoring cards
        if game_state.scoring_cards.len() < 4 {
            return false;
        }

        // Check if each suit is present in the scoring cards
        let has_diamonds = game_state
            .scoring_cards
            .iter()
            .any(|card| card.suit == Suit::Diamonds || card.enhancement == Some(Enhancement::Wild));
        let has_clubs = game_state
            .scoring_cards
            .iter()
            .any(|card| card.suit == Suit::Clubs || card.enhancement == Some(Enhancement::Wild));
        let has_hearts = game_state
            .scoring_cards
            .iter()
            .any(|card| card.suit == Suit::Hearts || card.enhancement == Some(Enhancement::Wild));
        let has_spades = game_state
            .scoring_cards
            .iter()
            .any(|card| card.suit == Suit::Spades || card.enhancement == Some(Enhancement::Wild));

        // Need all suits to be present
        has_diamonds && has_clubs && has_hearts && has_spades
    }

    fn apply(
        &self,
        game_state: &mut GameState,
        joker_card: &JokerCard,
        _current_card: &Card,
    ) -> GameResult<()> {
        game_state.mult *= 3.0;
        let message = format!(
            "{} x3 Mult ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}
