use crate::errors::{GameError, GameResult};
use crate::jokers;
use crate::modifiers::{apply_edition, apply_enhancement, apply_steel_enhancement};
use crate::poker::{get_scoring_cards, identify_hand};

// Import from external crates
use ortalib::{Card, Chips, Enhancement, Mult, PokerHand, Round};

#[derive(Debug)]
pub struct GameState {
    pub round: Round,               // The round data (from ortalib)
    pub chips: Chips,               // Current chip value during scoring
    pub mult: Mult,                 // Current multiplier during scoring
    pub explain_steps: Vec<String>, // Tracks explanation steps if needed
    pub explain_enabled: bool,      // Whether to track explain the scoring steps

    // Poker hand analysis fields
    pub scoring_cards: Vec<Card>, // Cards that contribute to the poker hand
    pub contains_pair: bool,      // If the hand contains a pair
    pub contains_two_pair: bool,  // If the hand contains two different pairs
    pub contains_three_of_a_kind: bool, // If the hand contains three of a kind
    pub contains_straight: bool,  // If the hand contains a straight
    pub contains_flush: bool,     // If the hand contains a flush

    // Joker effect tracking fields
    pub four_fingers_active: bool,  // Four Fingers joker is active
    pub shortcut_active: bool,      // Shortcut joker is active
    pub pareidolia_active: bool,    // Pareidolia joker is active
    pub splash_active: bool,        // Splash joker is active
    pub smeared_joker_active: bool, // Smeared Joker is active

    // Retrigger tracking
    pub mime_retriggers: usize, // Number of Mime retriggers to apply
    pub sock_and_buskin_retriggers: usize, // Number of Sock and Buskin retriggers

    // Used for tracking Photograph joker
    pub first_face_card_processed: bool,
}

impl GameState {
    pub fn new(round: Round, explain: bool) -> Self {
        Self {
            round,
            chips: 0.0,
            mult: 0.0,
            explain_steps: Vec::new(),
            explain_enabled: explain,

            scoring_cards: Vec::new(),
            contains_pair: false,
            contains_two_pair: false,
            contains_three_of_a_kind: false,
            contains_straight: false,
            contains_flush: false,

            four_fingers_active: false,
            shortcut_active: false,
            pareidolia_active: false,
            splash_active: false,
            smeared_joker_active: false,

            mime_retriggers: 0,
            sock_and_buskin_retriggers: 0,

            first_face_card_processed: false,
        }
    }

    pub fn get_explanation(&self) -> &[String] {
        &self.explain_steps
    }

    /// Adds an explanation step if explanation is enabled
    fn add_explanation(&mut self, step: String) {
        if self.explain_enabled {
            self.explain_steps.push(step);
        }
    }

    /// Analyzes the hand and marks what poker hand conditions exist
    fn analyze_hand_conditions(&mut self) -> GameResult<()> {
        let cards = &self.round.cards_played;

        let analyze_ranks = |cards: &[Card]| todo!();

        let contains_straight = |cards: &[Card], shortcut_active: bool| -> bool {
            // Implementation would check for straights
            // Including logic for the Shortcut joker if active
            // This is a simplified placeholder
            false // Placeholder
        };

        // Check if hand contains a flush
        let contains_flush = |cards: &[Card], smeared_joker_active: bool| -> bool {
            // Implementation would check for flushes
            // Including logic for the Smeared Joker if active
            // This is a simplified placeholder
            false // Placeholder
        };

        // Analyze the hand
        let (has_pair, has_two_pair, has_three) = analyze_ranks(cards);
        let has_straight = contains_straight(cards, self.shortcut_active);
        let has_flush = contains_flush(cards, self.smeared_joker_active);

        // Set the condition flags
        self.contains_pair = has_pair;
        self.contains_two_pair = has_two_pair;
        self.contains_three_of_a_kind = has_three;
        self.contains_straight = has_straight;
        self.contains_flush = has_flush;

        Ok(())
    }

    /// Process "OnScored" jokers for a specific card
    fn process_on_scored_jokers(&mut self, card: &Card) -> GameResult<()> {
        todo!()
    }

    /// Process "OnHeld" jokers for a specific card
    fn process_on_held_jokers(&mut self, card: &Card) -> GameResult<()> {
        todo!()
    }

    pub fn score(&mut self) -> GameResult<(Chips, Mult)> {
        // dbg!("ROUNDDDD {:?}", &self.round);
        // dbg!("cards_played {:?}", &self.round.cards_played);
        // dbg!("cards held in hand {:?}", &self.round.cards_held_in_hand);
        // dbg!("jokers! {:?}", &self.round.jokers);

        // Basic check
        if self.round.cards_played.is_empty() {
            return Ok((0.0, 0.0));
        }

        // Step 1: Identify the poker hand
        let poker_hand: PokerHand = identify_hand(&self.round.cards_played)
            .map_err(|e| GameError::InvalidHand(e.to_string()))?;
        let (base_chips, base_mult) = poker_hand.hand_value();

        let mut explanations = Vec::new();
        explanations.push(format!("{:?} ({} x {})", poker_hand, base_chips, base_mult));

        // Step 3: Initialize with base values
        let mut chips = base_chips;
        let mut mult = base_mult;

        // // Step 2: Get scoring cards
        // let scoring_cards: Vec<Card> = get_scoring_cards(&poker_hand, &self.round.cards_played);
        // Step 3: Analyze hand conditions for joker effects
        self.analyze_hand_conditions()?;

        // Step 4: Determine scoring cards
        if self.splash_active {
            // With Splash joker, all played cards score
            self.scoring_cards = self.round.cards_played.clone();
        } else {
            // Otherwise, only cards that form the poker hand
            self.scoring_cards = get_scoring_cards(&poker_hand, &self.round.cards_played);
        }


        // Step 4: Process each card separately to avoid borrowing conflicts
        for card in &self.scoring_cards {
            let rank_chips: f64 = card.rank.rank_value();
            chips += rank_chips;

            explanations.push(format!(
                "{}{} +{} Chips ({} x {})",
                card.rank, card.suit, rank_chips, chips, mult
            ));

            // Apply card enhancements if present
            if card.enhancement.is_some() {
                let enh_explanations = apply_enhancement(card, &mut chips, &mut mult)?;
                explanations.extend(enh_explanations);
            }

            // Apply card editions if present
            if card.edition.is_some() {
                let edition_explanations = apply_edition(card, &mut chips, &mut mult)?;
                explanations.extend(edition_explanations);
            }
            // Process "OnScored" jokers for this card
            // self.process_on_scored_jokers(card)?;
        }

        // Step 5: Process cards held in hand
        for card in &self.round.cards_held_in_hand {
            if let Some(Enhancement::Steel) = card.enhancement {
                let steel_explanations = apply_steel_enhancement(card, &mut chips, &mut mult)?;
                explanations.extend(steel_explanations);
            }
            // Process "OnHeld" jokers for this card
            // self.process_on_held_jokers(card)?;
        }

        // Step 6: Process jokers (independent activation)
        jokers::process_jokers(self)?;

        // Step 7: Save and mutate explanantion, chips, mult
        for explanation in explanations {
            self.add_explanation(explanation);
        }
        self.chips = chips;
        self.mult = mult;
        Ok((self.chips, self.mult))
    }
}
