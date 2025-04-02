use crate::errors::{GameError, GameResult};
use crate::jokers;
use crate::modifiers::{apply_edition, apply_enhancement, apply_steel_enhancement};
use crate::poker::{analyze_hand_conditions, get_scoring_cards, identify_hand};

use crate::explain_dbg_bool;

// Import from external crates
use ortalib::{Card, Chips, Enhancement, Joker, Mult, PokerHand, Round};

#[derive(Debug)]
pub struct GameState {
    pub round: Round,          // The round data (from ortalib)
    pub chips: Chips,          // Current chip value during scoring
    pub mult: Mult,            // Current multiplier during scoring
    pub explain_enabled: bool, // Whether to track explain the scoring steps

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

    /// Process "OnScored" jokers for a specific card
    fn process_on_scored_jokers(&mut self, card: &Card) -> GameResult<()> {
        for joker_card in &self.round.jokers.clone() {
            let effect = jokers::create_joker_effect(joker_card.joker);
            if effect.activation_type() == jokers::ActivationType::OnScored
                && effect.can_apply(self)
            {
                effect.apply(self, joker_card, card)?;
            }
        }

        // Handle Sock and Buskin retriggers
        let retrigger_count = self.sock_and_buskin_retriggers;
        if retrigger_count > 0 && (self.pareidolia_active || card.rank.is_face()) {
            // Clear the retrigger counter to prevent infinite loops
            self.sock_and_buskin_retriggers = 0;

            // Apply retriggers
            for _ in 0..retrigger_count {
                // Re-apply the card's base chips
                let rank_chips: f64 = card.rank.rank_value();
                self.chips += rank_chips;

                explain_dbg_bool!(
                    self.explain_enabled,
                    "Retrigger: {} +{} Chips ({} x {})",
                    card,
                    rank_chips,
                    self.chips,
                    self.mult
                );

                // Re-apply card enhancements and editions
                if card.enhancement.is_some() {
                    apply_enhancement(card, &mut self.chips, &mut self.mult, self.explain_enabled)?;
                }

                if card.edition.is_some() {
                    apply_edition(card, &mut self.chips, &mut self.mult, self.explain_enabled)?;
                }

                // Re-apply "OnScored" jokers but exclude Sock and Buskin to prevent infinite loops
                for joker_card in &self.round.jokers.clone() {
                    if joker_card.joker != Joker::SockAndBuskin {
                        let effect = jokers::create_joker_effect(joker_card.joker);
                        if effect.activation_type() == jokers::ActivationType::OnScored
                            && effect.can_apply(self)
                        {
                            effect.apply(self, joker_card, card)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Process "OnHeld" jokers for a specific card
    fn process_on_held_jokers(&mut self, card: &Card) -> GameResult<()> {
        // Get applicable jokers
        for joker_card in &self.round.jokers.clone() {
            let effect = jokers::create_joker_effect(joker_card.joker);
            if effect.activation_type() == jokers::ActivationType::OnHeld && effect.can_apply(self)
            {
                // For Raised Fist joker, only apply it if the current card is the lowest in hand
                // AND it's the right-most (last) instance of the lowest rank
                if joker_card.joker == Joker::RaisedFist {
                    // Find the cards with the lowest rank in hand
                    let lowest_rank = self
                        .round
                        .cards_held_in_hand
                        .iter()
                        .min_by_key(|c| c.rank)
                        .map(|c| c.rank);

                    if let Some(lowest) = lowest_rank {
                        // Find all cards with this lowest rank
                        let lowest_cards: Vec<&Card> = self
                            .round
                            .cards_held_in_hand
                            .iter()
                            .filter(|c| c.rank == lowest)
                            .collect();

                        // Get the last (right-most) card with this rank
                        let right_most = lowest_cards.last();

                        // Only apply if the current card is the right-most lowest one
                        if let Some(right_most_card) = right_most {
                            if right_most_card.rank == card.rank
                                && right_most_card.suit == card.suit
                                && right_most_card.enhancement == card.enhancement
                            {
                                effect.apply(self, joker_card, card)?;
                            }
                        }
                    }
                } else {
                    // For all other OnHeld jokers, apply as normal
                    effect.apply(self, joker_card, card)?;
                }
            }
        }

        // Handle Mime retriggers
        let retrigger_count = self.mime_retriggers;
        if retrigger_count > 0 {
            // Clear the retrigger counter to prevent infinite loops
            self.mime_retriggers = 0;

            // Apply retriggers
            for _ in 0..retrigger_count {
                // Re-apply Steel enhancement if present
                if let Some(Enhancement::Steel) = &card.enhancement {
                    apply_steel_enhancement(
                        card,
                        &mut self.chips,
                        &mut self.mult,
                        self.explain_enabled,
                    )?;
                }

                // Re-apply "OnHeld" jokers but exclude Mime to prevent infinite loops
                for joker_card in &self.round.jokers.clone() {
                    if joker_card.joker != Joker::Mime {
                        let effect = jokers::create_joker_effect(joker_card.joker);
                        if effect.activation_type() == jokers::ActivationType::OnHeld
                            && effect.can_apply(self)
                        {
                            // Special handling for Raised Fist again
                            if joker_card.joker == Joker::RaisedFist {
                                let lowest_rank = self
                                    .round
                                    .cards_held_in_hand
                                    .iter()
                                    .min_by_key(|c| c.rank)
                                    .map(|c| c.rank);

                                if let Some(lowest) = lowest_rank {
                                    if card.rank == lowest {
                                        effect.apply(self, joker_card, card)?;
                                    }
                                }
                            } else {
                                effect.apply(self, joker_card, card)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn score(&mut self) -> GameResult<(Chips, Mult)> {
        // println!("cards_played {:?}", &self.round.cards_played);
        // println!("cards held in hand {:?}", &self.round.cards_held_in_hand);
        // println!("jokers! {:?}", &self.round.jokers);

        // Basic check
        if self.round.cards_played.is_empty() {
            return Ok((0.0, 0.0));
        }

        // Step 1: Process jokers first to set any flags
        //?

        // Check if any jokers are active
        self.four_fingers_active = self
            .round
            .jokers
            .iter()
            .any(|joker_card| joker_card.joker == Joker::FourFingers);
        self.shortcut_active = self
            .round
            .jokers
            .iter()
            .any(|joker_card| joker_card.joker == Joker::Shortcut);
        self.pareidolia_active = self
            .round
            .jokers
            .iter()
            .any(|joker_card| joker_card.joker == Joker::Pareidolia);
        self.splash_active = self
            .round
            .jokers
            .iter()
            .any(|joker_card| joker_card.joker == Joker::Splash);
        self.smeared_joker_active = self
            .round
            .jokers
            .iter()
            .any(|joker_card| joker_card.joker == Joker::SmearedJoker);

        self.first_face_card_processed = false;

        // Step 1: Identify the poker hand
        let poker_hand: PokerHand = identify_hand(
            &self.round.cards_played,
            self.four_fingers_active,
            self.shortcut_active,
            self.smeared_joker_active,
        )
        .map_err(|e| GameError::InvalidHand(e.to_string()))?;
        let (base_chips, base_mult) = poker_hand.hand_value();
        self.chips = base_chips;
        self.mult = base_mult;
        explain_dbg_bool!(
            self.explain_enabled,
            "{:?} ({} x {})",
            poker_hand,
            base_chips,
            base_mult
        );

        // // Step 2: Get scoring cards
        // let scoring_cards: Vec<Card> = get_scoring_cards(&poker_hand, &self.round.cards_played);
        // Step 3: Analyze hand conditions for joker effects
        let conditions = analyze_hand_conditions(
            &self.round.cards_played,
            self.four_fingers_active,
            self.shortcut_active,
            self.smeared_joker_active,
        )?;
        self.contains_pair = conditions.contains_pair;
        self.contains_two_pair = conditions.contains_two_pair;
        self.contains_three_of_a_kind = conditions.contains_three_of_a_kind;
        self.contains_straight = conditions.contains_straight;
        self.contains_flush = conditions.contains_flush;

        // Step 4: Determine scoring cards
        self.scoring_cards = if self.splash_active {
            // With Splash joker, all played cards score
            self.round.cards_played.to_vec()
        } else {
            // Otherwise, only cards that form the poker hand
            get_scoring_cards(
                &poker_hand,
                &self.round.cards_played,
                self.four_fingers_active,
                self.shortcut_active,
                self.smeared_joker_active,
            )
        };

        // Step 4: Process each card separately
        for card in self.scoring_cards.clone() {
            let rank_chips: f64 = card.rank.rank_value();
            self.chips += rank_chips;

            explain_dbg_bool!(
                self.explain_enabled,
                "{}{} +{} Chips ({} x {})",
                card.rank,
                card.suit,
                rank_chips,
                self.chips,
                self.mult
            );

            // Apply card enhancements if present
            if card.enhancement.is_some() {
                apply_enhancement(&card, &mut self.chips, &mut self.mult, self.explain_enabled)?;
            }

            // Apply card editions if present
            if card.edition.is_some() {
                apply_edition(&card, &mut self.chips, &mut self.mult, self.explain_enabled)?;
            }
            // Process "OnScored" jokers for this card
            self.process_on_scored_jokers(&card)?;
        }

        // Step 5: Process cards held in hand
        for card in self.round.cards_held_in_hand.clone() {
            if let Some(Enhancement::Steel) = &card.enhancement {
                apply_steel_enhancement(
                    &card,
                    &mut self.chips,
                    &mut self.mult,
                    self.explain_enabled,
                )?;
            }
            // Process "OnHeld" jokers for this card
            self.process_on_held_jokers(&card)?;
        }

        // Step 6: Process jokers (independent activation)
        jokers::process_jokers(self)?;

        Ok((self.chips, self.mult))
    }
}
