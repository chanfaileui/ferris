use itertools::Itertools;
use ortalib::{Card, Chips, Mult, PokerHand, Rank, Round, Suit};
use std::collections::HashMap;

pub struct GameState {
    round: Round,               // The round data (from ortalib)
    chips: Chips,               // Current chip value during scoring
    mult: Mult,                 // Current multiplier during scoring
    explain_steps: Vec<String>, // Tracks explanation steps if needed
}

impl GameState {
    pub fn new(round: Round, _explain: bool) -> Self {
        Self {
            round,
            chips: 0.0,
            mult: 0.0,
            explain_steps: Vec::new(),
        }
    }

    // get function?!
    pub fn get_explanation(&self) -> &[String] {
        &self.explain_steps
    }

    fn group_rank(&self) -> HashMap<ortalib::Rank, usize> {
        let rank_counts = self
            .round
            .cards_played
            .iter()
            .map(|card| card.rank)
            .counts();
        rank_counts
    }
    fn group_by_rank(&self) -> HashMap<Rank, Vec<&Card>> {
        self.round
            .cards_played
            .iter()
            .into_group_map_by(|card| card.rank)
    }

    fn group_suit(&self) -> HashMap<ortalib::Suit, usize> {
        let suit_counts = self
            .round
            .cards_played
            .iter()
            .map(|card| card.suit)
            .counts();
        suit_counts
    }
    fn group_by_suit(&self) -> HashMap<Suit, Vec<&Card>> {
        self.round
            .cards_played
            .iter()
            .into_group_map_by(|card| card.suit)
    }

    fn identify_hand(&self) -> PokerHand {
        println!("ROUNDDDD {:?}", self.round);
        println!("cards_played {:?}", self.round.cards_played);
        println!("cards held in hand {:?}", self.round.cards_held_in_hand);
        println!("jokers! {:?}", self.round.jokers);
        println!("{:?}", self.group_rank());
        println!("{:?}", self.group_by_rank());
        println!("{:?}", self.group_suit());
        println!("{:?}", self.group_by_suit());

        let rank_count: HashMap<ortalib::Rank, usize> = self.group_rank();
        let suit_count: HashMap<ortalib::Suit, usize> = self.group_suit();

        // 1. Are all 5 cards the same rank?
        if rank_count.len() == 1 {
            // are they the same suit?
            todo!()
            if suit_count.len() == 1 {

            }
        }

        todo!()
    }

    fn scoring(&self) {
        todo!()
    }

    pub fn score(&self) -> (Chips, Mult) {
        let poker_hand: PokerHand = self.identify_hand();
        self.scoring();
        let score: (Chips, Mult) = poker_hand.hand_value();
        self.scoring();
    }
}
