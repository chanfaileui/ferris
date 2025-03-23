use std::collections::HashMap;
use itertools::Itertools;
use ortalib::{Chips, Mult, Round};

pub struct GameState {
    round: Round,               // The round data (from ortalib)
    chips: Chips,               // Current chip value during scoring
    mult: Mult,                 // Current multiplier during scoring
    explain_steps: Vec<String>, // Tracks explanation steps if needed
}

impl GameState {
    pub fn new(round: Round, explain: bool) -> Self {
        Self {
            round,
            chips: 0.0,
            mult: 0.0,
            explain_steps: if explain { Vec::new() } else { Vec::new() },
        }
    }

    // get function?!
    pub fn get_explanation(&self) -> &[String] {
        &self.explain_steps
    }

    fn group_rank(&self, round: &Round) -> HashMap<ortalib::Rank, usize> {
        let rank_counts = round.cards_played.iter().map(|card| card.rank).counts();
        rank_counts
    }

    fn group_suit(&self, round: &Round) -> HashMap<ortalib::Suit, usize> {
        let suit_counts = round.cards_played.iter().map(|card| card.suit).counts();
        suit_counts
    }

    pub fn score(&self) -> (Chips, Mult) {
        println!("ROUNDDDD {:?}", self.round);
        println!("cards_played {:?}", self.round.cards_played);
        println!("cards held in hand {:?}", self.round.cards_held_in_hand);
        println!("jokers! {:?}", self.round.jokers);
        println!("{:?}", self.group_rank(self.round));
        println!("{:?}", self.group_suit(self.round));

        let rank_count: HashMap<ortalib::Rank, usize> = self.group_rank(self.round);
        let suit_count: HashMap<ortalib::Suit, usize> = self.group_suit(self.round);

        // 1. Are all 5 cards the same rank?
        if rank_count.len() == 1 {
            // are they the same suit?
            todo!()
        }

        todo!()
        // best one is
    }
}
