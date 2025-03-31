use enum_iterator::Sequence;
use itertools::Itertools;
use ortalib::{Card, Chips, Mult, PokerHand, Rank, Round, Suit};
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

    pub fn score(&mut self) -> (Chips, Mult) {
        println!("ROUNDDDD {:?}", self.round);
        println!("cards_played {:?}", self.round.cards_played);
        println!("cards held in hand {:?}", self.round.cards_held_in_hand);
        println!("jokers! {:?}", self.round.jokers);
        println!("group by rank: {:?}", self.group_rank());
        println!("group by rank: {:?}", self.group_by_rank());
        println!("group by suit: {:?}", self.group_suit());
        println!("group by suit: {:?}", self.group_by_suit());
        // Basic check
        if self.round.cards_played.is_empty() {
            return (0.0, 0.0);
        }

        // Step 1: Identify the poker hand
        let poker_hand: PokerHand = self.identify_hand();
        let (base_chips, base_mult) = poker_hand.hand_value();
        self.add_explanation(format!("{:?} ({} x {})", poker_hand, base_chips, base_mult));

        // Step 2: Get scoring cards
        // Collect into a Vec to end the immutable borrow of self
        let scoring_cards: Vec<Card> = self.get_scoring_cards(&poker_hand).to_vec();

        // Step 3: Initialize with base values
        let mut chips = base_chips;
        let mult = base_mult;

        // Step 4: Process each card separately to avoid borrowing conflicts
        for card in scoring_cards {
            let rank_chips: f64 = card.rank.rank_value();
            chips += rank_chips;

            self.add_explanation(format!(
                "{} +{} Chips ({} x {})",
                card, rank_chips, chips, mult
            ));
        }

        self.chips = chips;
        self.mult = mult;
        (self.chips, self.mult)
    }
}
