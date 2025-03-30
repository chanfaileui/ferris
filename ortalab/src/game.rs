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

    /// Returns a HashMap mapping each rank to the number of cards with that rank in played cards
    fn group_rank(&self) -> HashMap<ortalib::Rank, usize> {
        let rank_counts = self
            .round
            .cards_played
            .iter()
            .map(|card| card.rank)
            .counts();
        rank_counts
    }

    /// Returns a HashMap mapping each rank to the cards with that rank in played cards
    fn group_by_rank(&self) -> HashMap<Rank, Vec<&Card>> {
        self.round
            .cards_played
            .iter()
            .into_group_map_by(|card| card.rank)
    }

    /// Returns a HashMap mapping each rank to the number of cards with that rank in played cards
    ///
    /// # Returns
    /// * HashMap<Rank, usize> - Maps ranks to their counts
    fn group_suit(&self) -> HashMap<ortalib::Suit, usize> {
        let suit_counts = self
            .round
            .cards_played
            .iter()
            .map(|card| card.suit)
            .counts();
        suit_counts
    }

    /// Returns a HashMap mapping each suit to the number of cards with that suit in played cards
    ///
    /// # Returns
    /// * HashMap<Suit, Vec<&Card>> - Maps suits to vectors of cards with that suit
    fn group_by_suit(&self) -> HashMap<Suit, Vec<&Card>> {
        self.round
            .cards_played
            .iter()
            .into_group_map_by(|card| card.suit)
    }

    fn is_sorted(&self) -> bool {
        let cards: &Vec<Card> = &self.round.cards_played;
        cards.windows(2).all(|w| w[0].rank < w[1].rank)
    }

    fn is_sequential(&self) -> bool {
        if !self.is_sorted() {
            return false;
        }
        let cards: &Vec<Card> = &self.round.cards_played;
        let first_value: f64 = cards[0].rank.rank_value();

        cards
            .iter()
            .enumerate()
            .all(|(i, card)| card.rank.rank_value() == first_value + i as f64)
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
            if suit_count.len() == 1 {
                return PokerHand::FlushFive;
            } else {
                return PokerHand::FiveOfAKind;
            }
        }

        // 2. Are all 5 cards the same suit?
        if suit_count.len() == 1 {
            // Check if sequential
            if self
                .round
                .cards_played
                .iter()
                .map(|card| card.rank)
                .sorted()
                .collect::<Vec<_>>()
                == self
                    .round
                    .cards_played
                    .iter()
                    .map(|card| card.rank)
                    .sorted()
                    .collect::<Vec<_>>()
            {
                return PokerHand::StraightFlush;
            } else {
                // check if 3 + 2 pattern
            }
        }
    }

    fn scoring(&self) {
        todo!()
    }

    pub fn score(&self) -> (Chips, Mult) {
        let poker_hand: PokerHand = self.identify_hand();
        let score: (Chips, Mult) = poker_hand.hand_value();
        score
    }
}
