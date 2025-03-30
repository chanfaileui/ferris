use enum_iterator::Sequence;
use itertools::Itertools;
use ortalib::{Card, Chips, Mult, PokerHand, Rank, Round, Suit};
use std::collections::HashMap;

// /// Result type for GameState operations
// type GameResult<T> = Result<T, GameError>;

// /// Custom error type for GameState operations
// #[derive(Debug)]
// pub enum GameError {
//     InvalidHand(String),
//     ScoringError(String),
// }

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

    /// Returns a HashMap mapping each rank to the number of cards with that rank in played cards
    /// For example, if five 10s are played, the result will be {10: 5}
    fn group_rank(&self) -> HashMap<Rank, usize> {
        self.round
            .cards_played
            .iter()
            .map(|card| card.rank)
            .counts()
    }

    /// Returns a HashMap mapping each rank to the cards with that rank in played cards
    /// For example, if five 10s are played, the result will be {10: [10♥, 10♠, 10♦, 10♣, 10♥]}
    fn group_by_rank(&self) -> HashMap<Rank, Vec<&Card>> {
        self.round
            .cards_played
            .iter()
            .into_group_map_by(|card| card.rank)
    }

    /// Returns a HashMap mapping each rank to the number of cards with that rank in played cards
    /// For example, {♣: 1, ♠: 1, ♥: 2, ♦: 1}
    fn group_suit(&self) -> HashMap<Suit, usize> {
        self.round
            .cards_played
            .iter()
            .map(|card| card.suit)
            .counts()
    }

    /// Returns a HashMap mapping each suit to the number of cards with that suit in played cards
    /// For example, if five 10s are played, the result will be {♠: [10♠], ♣: [10♣], ♥: [10♥, 10♥], ♦: [10♦]}
    fn group_by_suit(&self) -> HashMap<Suit, Vec<&Card>> {
        self.round
            .cards_played
            .iter()
            .into_group_map_by(|card| card.suit)
    }

    fn is_sequential(&self) -> bool {
        // get ranks and sort them
        let mut ranks: Vec<Rank> = self
            .round
            .cards_played
            .iter()
            .map(|card| card.rank)
            .collect();
        ranks.sort();

        // Special case: check for A-5 straight (Ace is low)
        if ranks[0] == Rank::Two
            && ranks[1] == Rank::Three
            && ranks[2] == Rank::Four
            && ranks[3] == Rank::Five
            && ranks[4] == Rank::Ace
        {
            return true;
        }

        for i in 0..ranks.len() - 1 {
            if let Some(next_rank) = ranks[i].next() {
                if next_rank != ranks[i + 1] {
                    return false;
                }
            } else {
                // Current rank doesn't have a next rank
                return false;
            }
        }

        true
    }

    /// Checks if the played cards form a 3+2 pattern (three cards of one rank, two of another)
    /// This is used to identify Full House and Flush House
    fn has_three_two_pattern(&self) -> bool {
        // We need at least 5 cards for a 3+2 pattern
        if self.round.cards_played.len() < 5 {
            return false;
        }

        // Group cards by rank
        let rank_counts = self.group_rank();

        // Need exactly 2 different ranks
        if rank_counts.len() != 2 {
            return false;
        }

        // Check if there's a 3-2 distribution
        let counts: Vec<usize> = rank_counts.values().copied().collect();
        counts.contains(&3) && counts.contains(&2)
    }

    fn identify_hand(&self) -> PokerHand {
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
            if self.is_sequential() {
                return PokerHand::StraightFlush;
            } else {
                // check if 3 + 2 pattern
                if self.has_three_two_pattern() {
                    return PokerHand::FlushHouse;
                } else {
                    return PokerHand::Flush;
                }
            }
        }

        // 3. Are 4 cards the same rank?
        if rank_count.values().any(|&count| count == 4) {
            return PokerHand::FourOfAKind;
        }

        // 4. Is it a 3+2 pattern? (Three of one rank, two of another)
        if self.has_three_two_pattern() {
            return PokerHand::FullHouse;
        }

        // 5. Are 5 cards sequential?
        if self.is_sequential() {
            return PokerHand::Straight;
        }

        // 6. Are 3 cards the same rank?
        for &count in rank_count.values() {
            if count == 3 {
                return PokerHand::ThreeOfAKind;
            }
        }

        // 7. Are there two pairs?
        if rank_count.values().filter(|&&count| count == 2).count() == 2 {
            return PokerHand::TwoPair;
        }

        // 8. Is there one pair?
        for &count in rank_count.values() {
            if count == 2 {
                return PokerHand::Pair;
            }
        }

        // if none of the above, it's a high card
        PokerHand::HighCard
    }

    /// Returns the cards that contribute to the scoring for a given poker hand
    ///
    /// According to the rules, generally only the cards relevant to the poker hand
    /// are scored, and all others are unscored. This function identifies which cards
    /// should be scored based on the poker hand type.
    pub fn get_scoring_cards(&self, hand_type: &PokerHand) -> Vec<&Card> {
        match hand_type {
            PokerHand::HighCard => {
                // For high card, only the highest card scores
                let rank_map = self.group_by_rank();
                let mut ranks: Vec<Rank> = rank_map.keys().cloned().collect();
                ranks.sort_by(|a, b| b.cmp(a)); // Sort in descending order

                // Get the highest rank's cards
                if let Some(highest_rank) = ranks.first() {
                    if let Some(cards) = rank_map.get(highest_rank) {
                        if !cards.is_empty() {
                            return vec![cards[0]]; // Return only the first card of the highest rank
                        }
                    }
                }
                vec![]
            }
            PokerHand::Pair => {
                todo!()
            }
            PokerHand::TwoPair => {
                todo!()
            }
            PokerHand::ThreeOfAKind => {
                todo!()
            }
            PokerHand::Straight | PokerHand::StraightFlush => {
                todo!()
            }
            PokerHand::Flush => {
                todo!()
            }
            PokerHand::FullHouse => {
                todo!()
            }
            PokerHand::FourOfAKind => {
                todo!()
            }
            PokerHand::FiveOfAKind | PokerHand::FlushHouse | PokerHand::FlushFive => {
                self.round.cards_played.iter().collect()
            }
        }
    }

    fn apply_card_scores(
        &mut self,
        scoring_cards: &[&Card],
        chips: Chips,
        mult: Mult,
    ) -> (Chips, Mult) {
        scoring_cards
            .iter()
            .map(|card| {
                let rank_chips = card.rank.rank_value();
                explain_steps.push(format!(
                    "{} +{} Chips ({} x {})",
                    card, rank_chips, *chips, *mult
                ));

                // Handle enhancements, editions, jokers
                // self.apply_card_effects(card);
                rank_chips
            })
            .sum()
        // // Process each scoring card
        // for card in scoring_cards {
        //     // Add base chips for the card's rank
        //     let rank_chips = card.rank.rank_value();
        //     chips += rank_chips;

        //     // Add explanation for the card's contribution
        //     self.explain_steps.push(format!(
        //         "{} +{} Chips ({} x {})",
        //         card, rank_chips, chips, mult
        //     ));
        //     // Apply card enhancements if present
        //     if let Some(enhancement) = card.enhancement {
        //         // Process enhancement effects
        //         // ... (enhancement logic)
        //     }

        //     // Apply card editions if present
        //     if let Some(edition) = card.edition {
        //         // Process edition effects
        //         // ... (edition logic)
        //     }

        //     // Apply "on scored" joker effects
        //     // ... (joker logic)
        // }

        // // Step 3: Process cards held in hand
        // // ... (held cards logic)

        // // Step 4: Process joker effects
        // // ... (joker logic)

        // chips
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

        let poker_hand: PokerHand = self.identify_hand();

        // Get base score values from the poker hand
        let (base_chips, base_mult) = poker_hand.hand_value();
        self.add_explanation(format!("{:?} ({} x {})", poker_hand, base_chips, base_mult));

        // Step 2: Process scoring cards
        let scoring_cards = self.get_scoring_cards(&poker_hand);
        let (chip_value, mult_value) =
            self.apply_card_scores(&scoring_cards, base_chips, base_mult);

        self.chips = chip_value;
        self.mult = mult_value;

        self.add_explanation(format!(
            "Final score: {} chips × {} mult = {}",
            self.chips,
            self.mult,
            self.chips * self.mult
        ));

        (self.chips, self.mult)
    }
}
