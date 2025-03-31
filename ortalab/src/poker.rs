use crate::errors::{GameError, GameResult};
use itertools::Itertools;
use ortalib::{Card, Enhancement, PokerHand, Rank, Suit};
use std::collections::HashMap;

pub fn identify_hand(cards: &[Card]) -> GameResult<PokerHand> {
    todo!()
}

pub fn get_scoring_cards(hand_type: &PokerHand, cards: &[Card]) -> Vec<Card> {
    todo!()
}
