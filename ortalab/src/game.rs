use crate::errors::{GameError, GameResult};
use crate::modifiers::{apply_edition, apply_enhancement};
use crate::poker::{get_scoring_cards, identify_hand};

// Import from external crates
use ortalib::{Card, Chips, Mult, PokerHand, Round};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameState {
    round: Round,               // The round data (from ortalib)
    chips: Chips,               // Current chip value during scoring
    mult: Mult,                 // Current multiplier during scoring
    explain_steps: Vec<String>, // Tracks explanation steps if needed
    explain_enabled: bool,      // Whether to track explain the scoring steps
}

impl GameState {
    pub fn new(round: Round, explain: bool) -> Self {
        Self {
            round,
            chips: 0.0,
            mult: 0.0,
            explain_steps: Vec::new(),
            explain_enabled: explain,
        }
    }

    // get function?!
    pub fn get_explanation(&self) -> &[String] {
        &self.explain_steps
    }

    /// Adds an explanation step if explanation is enabled
    fn add_explanation(&mut self, step: String) {
        if self.explain_enabled {
            self.explain_steps.push(step);
        }
    }

    pub fn score(&mut self) -> GameResult<(Chips, Mult)> {
        // println!("ROUNDDDD {:?}", self.round);
        // println!("cards_played {:?}", self.round.cards_played);
        // println!("cards held in hand {:?}", self.round.cards_held_in_hand);
        // println!("jokers! {:?}", self.round.jokers);

        // Basic check
        if self.round.cards_played.is_empty() {
            return Ok((0.0, 0.0));
        }

        // Step 1: Identify the poker hand
        let poker_hand: PokerHand = identify_hand(&self.round.cards_played)
            .map_err(|e| GameError::InvalidHand(e.to_string()))?;
        let (base_chips, base_mult) = poker_hand.hand_value();
        self.add_explanation(format!("{:?} ({} x {})", poker_hand, base_chips, base_mult));

        // Step 2: Get scoring cards
        let scoring_cards: Vec<Card> = get_scoring_cards(&poker_hand, &self.round.cards_played);

        // Step 3: Initialize with base values
        let mut chips = base_chips;
        let mut mult = base_mult;

        // Step 4: Process each card separately to avoid borrowing conflicts
        for card in scoring_cards {
            let rank_chips: f64 = card.rank.rank_value();
            chips += rank_chips;

            self.add_explanation(format!(
                "{}{} +{} Chips ({} x {})",
                card.rank, card.suit, rank_chips, chips, mult
            ));

            // Apply card enhancements if present
            if let Some(enhancement) = card.enhancement {
                let explanations = apply_enhancement(&card, &mut chips, &mut mult)?;
                for explanation in explanations {
                    self.add_explanation(explanation);
                }
            }

            // Apply card editions if present
            if let Some(edition) = card.edition {
                let explanations = apply_edition(&card, &mut chips, &mut mult)?;
                for explanation in explanations {
                    self.add_explanation(explanation);
                }
            }
        }

        // Step 3: Process cards held in hand
        // ... (held cards logic)

        // Step 4: Process joker effects
        // ... (joker logic)

        self.chips = chips;
        self.mult = mult;
        Ok((self.chips, self.mult))
    }
}
