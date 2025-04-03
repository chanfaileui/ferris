//! # Poker Module
//!
//! This module contains functions for poker hand analysis and identification.
//!
//! ## Key Components
//! - `identify_hand()`: Determines the poker hand type from a set of cards
//! - `get_scoring_cards()`: Identifies which cards contribute to scoring
//! - `analyse_hand_conditions()`: Analyses hand for specific conditions (pairs, straights, etc.)
//!
//! ## Hand Analysis
//! The module supports standard poker hand analysis as well as special cases:
//! - Shortcut joker effects (allowing straights with gaps)
//! - Four Fingers joker effects (allowing 4-card hands)
//! - Smeared Joker effects (treating cards as having both suits of the same color)
//!
//! ## Helper Functions
//! Various helper functions support the analysis of specific hand types:
//! - Straight detection (regular and shortcut)
//! - Flush detection
//! - Pair/Three-of-a-kind/etc. detection

use crate::errors::GameResult;
use enum_iterator::Sequence;
use indexmap::IndexMap;
use ortalib::{Card, PokerHand, Rank, Suit};

/// Returns a IndexMap mapping each rank to the number of cards with that rank in played cards
/// For example, if five 10s are played, the result will be {10: 5}
fn group_rank(cards: &[Card]) -> IndexMap<Rank, usize> {
    let mut counts = IndexMap::new();
    for card in cards {
        *counts.entry(card.rank).or_insert(0) += 1;
    }
    counts
}

/// Returns a IndexMap mapping each rank to the cards with that rank in played cards
/// For example, if five 10s are played, the result will be {10: [10♥, 10♠, 10♦, 10♣, 10♥]}
fn group_by_rank(cards: &[Card]) -> IndexMap<Rank, Vec<&Card>> {
    let mut groups = IndexMap::new();
    for card in cards {
        groups.entry(card.rank).or_insert_with(Vec::new).push(card);
    }
    groups
}

/// Returns a IndexMap mapping each suit to the number of cards with that suit in played cards
/// For example, if five 10s are played, the result will be {♠: [10♠], ♣: [10♣], ♥: [10♥, 10♥], ♦: [10♦]}
fn group_by_suit(cards: &[Card], smeared_joker_active: bool) -> IndexMap<Suit, Vec<&Card>> {
    let mut suit_cards: IndexMap<Suit, Vec<&Card>> = IndexMap::new();

    // if there are any wild cards, we need to count them as all suits
    for card in cards {
        if card.enhancement == Some(ortalib::Enhancement::Wild) {
            for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
                suit_cards.entry(suit).or_default().push(card);
            }
        } else if smeared_joker_active {
            // With Smeared Joker, add the card to its own suit AND the suit of the same color
            suit_cards.entry(card.suit).or_default().push(card);
            let same_color_suit = card.suit.other_suit_of_same_color();
            suit_cards.entry(same_color_suit).or_default().push(card);
        } else {
            suit_cards.entry(card.suit).or_default().push(card);
        }
    }

    suit_cards
}

/// Determines if the cards form a flush (all cards of the same suit)
fn is_flush(cards: &[Card], smeared_joker_active: bool) -> bool {
    if cards.len() < 5 {
        return false;
    }

    // Group by suit, considering Wild cards as every suit
    let suit_groups = group_by_suit(cards, smeared_joker_active);

    // Check if any suit has enough cards for a flush
    suit_groups.values().any(|suit_cards| suit_cards.len() >= 5)
}

/// Determines if the cards form a straight (consecutive ranks)
fn is_straight(cards: &[Card]) -> bool {
    if cards.len() < 5 {
        return false; // Not enough cards for a straight
    }

    // get ranks and sort them
    let mut ranks: Vec<Rank> = cards.iter().map(|card| card.rank).collect();
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
fn has_three_two_pattern(cards: &[Card]) -> bool {
    // We need at least 5 cards for a 3+2 pattern
    if cards.len() < 5 {
        return false;
    }

    // Group cards by rank
    let rank_counts = group_rank(cards);

    // Need exactly 2 different ranks
    if rank_counts.len() != 2 {
        return false;
    }

    // Check if there's a 3-2 distribution
    let counts: Vec<usize> = rank_counts.values().copied().collect();
    counts.contains(&3) && counts.contains(&2)
}

/// Check if there's a 4-card flush in the hand
fn has_four_card_flush(cards: &[Card], smeared_joker_active: bool) -> bool {
    if cards.len() < 4 {
        return false;
    }

    // Group by suit
    let suit_groups = group_by_suit(cards, smeared_joker_active);
    // println!("4card flush suit_groups: {:?}", suit_groups);
    // Check if any suit appears at least 4 times
    suit_groups.values().any(|cards| cards.len() >= 4)
}

/// Check if there's a 4-card straight in the hand
fn has_four_card_straight(cards: &[Card]) -> bool {
    if cards.len() < 4 {
        return false;
    }

    // get ranks and sort them
    let mut ranks: Vec<Rank> = cards.iter().map(|card| card.rank).collect();
    ranks.sort();
    ranks.dedup();
    // If we don't have at least 4 unique ranks, no straight is possible
    if ranks.len() < 4 {
        return false;
    }

    // Special case: check for A-4 straight (Ace is low)
    if ranks.contains(&Rank::Ace)
        && ranks.contains(&Rank::Two)
        && ranks.contains(&Rank::Three)
        && ranks.contains(&Rank::Four)
    {
        return true;
    }

    // Check for 4 consecutive ranks in any window
    for window in ranks.windows(4) {
        let mut is_consecutive = true;

        for i in 0..3 {
            if let Some(expected_next) = window[i].next() {
                if expected_next != window[i + 1] {
                    is_consecutive = false;
                    break;
                }
            } else {
                // If there's no next rank, they can't be consecutive
                is_consecutive = false;
                break;
            }
        }

        if is_consecutive {
            return true;
        }
    }

    false // No 4-card straight found
}

/// Checks if the cards form a straight with the Shortcut joker active
/// Shortcut allows straights to be made with gaps of 1 rank
fn has_shortcut_straight(cards: &[Card]) -> bool {
    // Extract ranks, sort them, and remove duplicates to handle the case properly
    let mut ranks: Vec<Rank> = cards.iter().map(|card| card.rank).collect();
    ranks.sort();
    ranks.dedup(); // Remove duplicates to get unique ranks

    // Need at least 5 unique ranks
    if ranks.len() < 5 {
        return false;
    }

    // Convert ranks to their numeric values for easier gap calculation
    let rank_values: Vec<i32> = ranks.iter().map(|r| r.rank_value() as i32).collect();

    // Check all possible windows of 5 cards for valid shortcut straights
    'window_loop: for window_start in 0..=rank_values.len() - 5 {
        let window = &rank_values[window_start..window_start + 5];

        // For a 5-card shortcut straight, the total span should be at most 8
        // (4 positions with up to 4 gaps of 1 rank each)
        let total_span = window[4] - window[0];

        if total_span <= 8 {
            // Count individual gaps to ensure no gap is larger than 1
            for i in 0..4 {
                let gap = window[i + 1] - window[i] - 1;
                if gap > 1 {
                    // Gap too large, try next window
                    continue 'window_loop;
                }
            }
            // If we reach here, all gaps are 0 or 1, which is valid
            return true;
        }
    }

    // Special handling for Ace-low straights
    if ranks.contains(&Rank::Ace) {
        // Create a new array with Ace = 1 (low) instead of 14 (high)
        let mut low_ace_values: Vec<i32> = Vec::new();
        low_ace_values.push(1); // Add 1 (low Ace)

        // Add all non-Ace ranks
        for rank in &ranks {
            if *rank != Rank::Ace {
                low_ace_values.push(rank.rank_value() as i32);
            }
        }
        low_ace_values.sort();

        // Check for valid Ace-low shortcut straights
        'ace_window_loop: for window_start in 0..=low_ace_values.len() - 5 {
            let window = &low_ace_values[window_start..window_start + 5];

            // Same span logic as above
            let total_span = window[4] - window[0];

            if total_span <= 8 {
                // Check individual gaps
                for i in 0..4 {
                    let gap = window[i + 1] - window[i] - 1;
                    if gap > 1 {
                        continue 'ace_window_loop;
                    }
                }
                return true;
            }
        }
    }

    false
}

/// Checks if the cards form a 4-card straight with gaps (for Four Fingers + Shortcut)
fn has_four_card_shortcut_straight(cards: &[Card]) -> bool {
    if cards.len() < 4 {
        return false;
    }

    // Extract ranks, sort them, and remove duplicates
    let mut ranks: Vec<Rank> = cards.iter().map(|card| card.rank).collect();
    ranks.sort();
    ranks.dedup();

    // Need at least 4 unique ranks
    if ranks.len() < 4 {
        return false;
    }

    // Convert ranks to numeric values
    let rank_values: Vec<i32> = ranks.iter().map(|r| r.rank_value() as i32).collect();

    // Check all possible windows of 4 cards
    'window_loop: for window_start in 0..=rank_values.len() - 4 {
        let window = &rank_values[window_start..window_start + 4];

        // For a 4-card shortcut straight, the total span should be at most 6
        // (3 positions with up to 3 gaps of 1 rank each)
        let total_span = window[3] - window[0];

        if total_span <= 6 {
            // Check individual gaps to ensure no gap is larger than 1
            for i in 0..3 {
                let gap = window[i + 1] - window[i] - 1;
                if gap > 1 {
                    // Gap too large, try next window
                    continue 'window_loop;
                }
            }
            // All gaps are 0 or 1, valid shortcut straight
            return true;
        }
    }

    // Special case for Ace-low
    if ranks.contains(&Rank::Ace) {
        let mut low_ace_values = vec![1]; // Ace as 1

        // Add non-Ace ranks
        for rank in &ranks {
            if *rank != Rank::Ace {
                low_ace_values.push(rank.rank_value() as i32);
            }
        }
        low_ace_values.sort();

        // Check Ace-low windows
        'ace_window_loop: for window_start in 0..=low_ace_values.len() - 4 {
            if low_ace_values.len() < window_start + 4 {
                break;
            }

            let window = &low_ace_values[window_start..window_start + 4];
            let total_span = window[3] - window[0];

            if total_span <= 6 {
                // Check individual gaps
                for i in 0..3 {
                    let gap = window[i + 1] - window[i] - 1;
                    if gap > 1 {
                        continue 'ace_window_loop;
                    }
                }
                return true;
            }
        }
    }

    false
}

/// Identifies the poker hand type from a set of cards
///
/// This function analyses the cards and determines the poker hand type
/// based on the rules of Balatro. It supports standard poker hands
/// as well as special cases like shortcut straights and four-card straights.
pub fn identify_hand(
    cards: &[Card],
    four_fingers_active: bool,
    shortcut_active: bool,
    smeared_joker_active: bool,
) -> GameResult<PokerHand> {
    if cards.len() < 2 {
        // With only 0 or 1 card, it's always a High Card
        return Ok(PokerHand::HighCard);
    }

    // Get rank counts
    let rank_count = group_rank(cards);

    // For Five of a Kind, we need at least 5 cards of the same rank
    let all_same_rank = rank_count.len() == 1 && cards.len() >= 5;

    let has_flush = is_flush(cards, smeared_joker_active);
    let has_straight = is_straight(cards) || (shortcut_active && has_shortcut_straight(cards));
    let has_three_two = has_three_two_pattern(cards);
    let has_four_of_a_kind = rank_count.values().any(|&count| count >= 4);
    let has_three_of_a_kind = rank_count.values().any(|&count| count >= 3);

    // Count actual pairs (exactly 2 cards of the same rank)
    let pair_count = rank_count.values().filter(|&&count| count == 2).count();

    // Special case for exactly 2 cards of the same rank
    let is_simple_pair = cards.len() == 2 && rank_count.len() == 1;

    // Four Fingers joker support
    let has_four_card_flush = if four_fingers_active && cards.len() >= 4 {
        has_four_card_flush(cards, smeared_joker_active)
    } else {
        false
    };

    let has_four_card_straight = if four_fingers_active && cards.len() >= 4 {
        has_four_card_straight(cards) || (shortcut_active && has_four_card_shortcut_straight(cards))
    } else {
        false
    };

    // Effective conditions
    let effective_flush = has_flush || has_four_card_flush;
    let effective_straight = has_straight || has_four_card_straight;

    // Hand identification in order of precedence

    // 12. Flush Five (all same rank and suit)
    if all_same_rank && effective_flush {
        return Ok(PokerHand::FlushFive);
    }

    // 11. Flush House (full house with all same suit)
    if has_three_two && effective_flush {
        return Ok(PokerHand::FlushHouse);
    }

    // 10. Five of a Kind (all same rank)
    if all_same_rank {
        return Ok(PokerHand::FiveOfAKind);
    }

    // 9. Straight Flush
    if effective_straight && effective_flush {
        return Ok(PokerHand::StraightFlush);
    }

    // 8. Four of a Kind
    if has_four_of_a_kind {
        return Ok(PokerHand::FourOfAKind);
    }

    // 7. Full House
    if has_three_two {
        return Ok(PokerHand::FullHouse);
    }

    // 6. Flush
    if effective_flush {
        return Ok(PokerHand::Flush);
    }

    // 5. Straight
    if effective_straight {
        return Ok(PokerHand::Straight);
    }

    // 4. Three of a Kind
    if has_three_of_a_kind {
        return Ok(PokerHand::ThreeOfAKind);
    }

    // 3. Two Pair
    if pair_count >= 2 {
        return Ok(PokerHand::TwoPair);
    }

    // 2. Pair - a single pair or exactly two cards of the same rank
    if pair_count == 1 || is_simple_pair {
        return Ok(PokerHand::Pair);
    }

    // 1. High Card
    Ok(PokerHand::HighCard)
}

/// Helper function to find the cards that form a shortcut straight
fn find_shortcut_straight_cards(cards: &[Card]) -> Vec<Card> {
    // Extract ranks with their original card indices
    let mut rank_info: Vec<(usize, Rank, i32)> = cards
        .iter()
        .enumerate()
        .map(|(i, card)| (i, card.rank, card.rank.rank_value() as i32))
        .collect();

    // Sort by rank value
    rank_info.sort_by_key(|&(_, _, val)| val);

    // Get unique ranks (maintain original indices)
    let mut unique_rank_info = Vec::new();
    let mut last_rank = None;

    for &(idx, rank, value) in &rank_info {
        if last_rank != Some(rank) {
            unique_rank_info.push((idx, rank, value));
            last_rank = Some(rank);
        }
    }

    // Check standard straights with gaps
    if unique_rank_info.len() >= 5 {
        for window_start in 0..=unique_rank_info.len() - 5 {
            let window = &unique_rank_info[window_start..window_start + 5];
            let total_gap = window[4].2 - window[0].2 - 4; // Expected difference is 4

            if total_gap <= 1 {
                // We found a valid shortcut straight
                return window.iter().map(|&(i, _, _)| cards[i]).collect();
            }
        }
    }

    // Check for Ace-low straights with gaps
    if unique_rank_info
        .iter()
        .any(|&(_, rank, _)| rank == Rank::Ace)
    {
        // Find the Ace's index
        let ace_idx = unique_rank_info
            .iter()
            .position(|&(_, rank, _)| rank == Rank::Ace)
            .unwrap();
        let (orig_ace_idx, _, _) = unique_rank_info[ace_idx];

        // Create a new array with Ace = 1 (low) instead of 14 (high)
        let mut low_ace_info = vec![(orig_ace_idx, Rank::Ace, 1)];

        // Add all non-Ace ranks
        for &(idx, rank, value) in &unique_rank_info {
            if rank != Rank::Ace {
                low_ace_info.push((idx, rank, value));
            }
        }

        // Sort by rank value
        low_ace_info.sort_by_key(|&(_, _, val)| val);

        // Check for straights with the Ace as low
        if low_ace_info.len() >= 5 {
            for window_start in 0..=low_ace_info.len() - 5 {
                let window = &low_ace_info[window_start..window_start + 5];
                let total_gap = window[4].2 - window[0].2 - 4;

                if total_gap <= 1 {
                    // Valid straight with at most one gap
                    return window.iter().map(|&(i, _, _)| cards[i]).collect();
                }
            }
        }
    }

    // Fallback: if we couldn't identify a specific shortcut straight, return all cards
    // (This might not be correct for all cases, but prevents returning an empty result)
    cards.iter().take(5).cloned().collect()
}

/// Helper function to find the cards that form a 4-card shortcut straight
fn find_four_card_shortcut_straight(cards: &[Card]) -> Vec<Card> {
    // Similar to find_shortcut_straight_cards but for 4-card sequences
    let mut rank_indices: Vec<(usize, u8)> = cards
        .iter()
        .enumerate()
        .map(|(i, card)| (i, card.rank.rank_value() as u8))
        .collect();

    // Sort by rank value
    rank_indices.sort_by_key(|&(_, rank)| rank);

    // Get unique ranks (maintain original indices)
    let mut unique_ranks = Vec::new();
    let mut last_rank = 0;
    for &(idx, rank) in &rank_indices {
        if rank != last_rank {
            unique_ranks.push((idx, rank));
            last_rank = rank;
        }
    }

    // Special case: check for Ace-low shortcut straight
    if unique_ranks.iter().any(|&(_, r)| r == 14) {
        // Ace present
        let mut low_ranks = Vec::new();
        let mut ace_idx = 0;

        for &(idx, rank) in &unique_ranks {
            if rank == 14 {
                ace_idx = idx;
            } else if rank <= 5 {
                // Consider ranks 2-5 for Ace-low with a gap
                low_ranks.push((idx, rank));
            }
        }

        // Check for Ace-low shortcut straight (A-2-4-5 or similar)
        if low_ranks.len() >= 3 {
            // Sort low ranks
            low_ranks.sort_by_key(|&(_, r)| r);

            // Check if we can form a valid shortcut sequence
            for start in 0..=low_ranks.len() - 3 {
                let window = &low_ranks[start..start + 3];
                let total_span = window[2].1 - window[0].1;

                // For a shortcut straight with 3 cards, the span should be at most 4
                if total_span <= 4 {
                    // We found a valid sequence including Ace as low
                    let mut result = Vec::new();
                    result.push(cards[ace_idx]); // Add Ace

                    // Add the three cards from the window
                    for &(i, _) in window {
                        result.push(cards[i]);
                    }

                    // Return all 4 cards
                    return result;
                }
            }
        }
    }

    // Check for regular shortcut straight
    for start in 0..=unique_ranks.len() - 4 {
        let window = &unique_ranks[start..start + 4];
        let total_span = window[3].1 - window[0].1;

        // For a shortcut straight with 4 cards, the span should be at most 5
        if total_span <= 5 {
            // Check if there's at most one gap
            let mut gap_count = 0;
            for i in 0..3 {
                let gap = window[i + 1].1 - window[i].1 - 1;
                match gap {
                    0 => (), // No gap
                    1 => gap_count += 1,
                    _ => {
                        gap_count = 2; // Too big gap, invalid
                        break;
                    }
                }
            }

            if gap_count <= 1 {
                // We found a valid 4-card shortcut straight
                return window.iter().map(|&(i, _)| cards[i]).collect();
            }
        }
    }

    // Fallback: return the first 4 cards if we couldn't find a specific straight
    cards.iter().take(4).copied().collect()
}

/// Find cards that make a standard 4-card straight (without gaps)
fn find_four_card_straight(cards: &[Card]) -> Vec<Card> {
    let mut rank_indices: Vec<(usize, u8)> = cards
        .iter()
        .enumerate()
        .map(|(i, card)| (i, card.rank.rank_value() as u8))
        .collect();

    // Sort by rank value
    rank_indices.sort_by_key(|&(_, rank)| rank);

    // Check for consecutive sequences of 4 cards
    for window in rank_indices.windows(4) {
        if window[3].1 - window[0].1 == 3 {
            // Found 4 consecutive cards
            return window.iter().map(|&(i, _)| cards[i]).collect();
        }
    }

    // Check for A-2-3-4 straight
    let ace_indices: Vec<usize> = rank_indices
        .iter()
        .filter(|&&(_, rank)| rank == 14) // Ace
        .map(|&(i, _)| i)
        .collect();

    let low_cards: Vec<(usize, u8)> = rank_indices
        .iter()
        .filter(|&&(_, rank)| (2..=4).contains(&rank))
        .copied()
        .collect();

    if !ace_indices.is_empty()
        && low_cards.len() >= 3
        && low_cards.iter().any(|&(_, r)| r == 2)
        && low_cards.iter().any(|&(_, r)| r == 3)
        && low_cards.iter().any(|&(_, r)| r == 4)
    {
        let mut result = Vec::new();
        // Add the Ace
        result.push(cards[ace_indices[0]]);
        // Add the 2, 3, 4
        for &(i, r) in &low_cards {
            if (2..=4).contains(&r) && result.len() < 4 {
                result.push(cards[i]);
            }
        }
        return result;
    }

    // Fallback
    cards.iter().take(4).copied().collect()
}

/// Find cards forming a four-card flush
fn find_four_card_flush(cards: &[Card], smeared_joker_active: bool) -> Vec<Card> {
    // Group by suit
    let suit_groups = group_by_suit(cards, smeared_joker_active);

    // Find the first suit with at least 4 cards
    for (_, suit_cards) in suit_groups {
        if suit_cards.len() >= 4 {
            // Take the first 4 cards of that suit
            return suit_cards.iter().take(4).map(|&&card| card).collect();
        }
    }

    // Fallback
    Vec::new()
}

/// Find cards forming a shortcut straight flush (5 cards with at most one gap)
fn find_shortcut_straight_flush_cards(cards: &[Card], smeared_joker_active: bool) -> Vec<Card> {
    // Group by suit
    let suit_groups = group_by_suit(cards, smeared_joker_active);

    // Check each suit group for a shortcut straight
    for (_, suit_cards) in suit_groups {
        if suit_cards.len() >= 5 {
            let suit_cards_vec: Vec<Card> = suit_cards.iter().map(|&&c| c).collect();
            if has_shortcut_straight(&suit_cards_vec) {
                return find_shortcut_straight_cards(&suit_cards_vec);
            }
        }
    }

    // Fallback
    Vec::new()
}

/// Returns the cards that contribute to the scoring for a given poker hand
///
/// According to the rules, generally only the cards relevant to the poker hand
/// are scored, and all others are unscored. This function identifies which cards
/// should be scored based on the poker hand type.
pub fn get_scoring_cards(
    hand_type: &PokerHand,
    cards: &[Card],
    four_fingers_active: bool,
    shortcut_active: bool,
    smeared_joker_active: bool,
) -> Vec<Card> {
    match hand_type {
        PokerHand::HighCard => {
            // For high card, only the highest card scores
            let rank_map: IndexMap<Rank, Vec<&Card>> = group_by_rank(cards);
            let mut ranks: Vec<Rank> = rank_map.keys().copied().collect();
            ranks.sort_by(|a: &Rank, b: &Rank| b.cmp(a)); // Sort in descending order

            // Get the highest rank's cards
            if let Some(highest_rank) = ranks.first() {
                if let Some(cards) = rank_map.get(highest_rank) {
                    if !cards.is_empty() {
                        return vec![*cards[0]]; // Return only the first card of the highest rank
                    }
                }
            }
            vec![]
        }
        PokerHand::Pair => {
            // Find the pair
            group_by_rank(cards)
                .into_iter()
                .find_map(|(_, cards)| {
                    if cards.len() == 2 {
                        Some(cards.iter().map(|&card| *card).collect())
                    } else {
                        None
                    }
                })
                .unwrap_or_default()
        }
        PokerHand::TwoPair => {
            group_by_rank(cards)
                .into_iter()
                .filter_map(|(_, cards)| {
                    if cards.len() == 2 {
                        // This is a pair
                        Some(cards.iter().map(|&card| *card).collect::<Vec<Card>>())
                    } else {
                        None
                    }
                })
                .flatten() // Flatten the Vec<Vec<Card>> into Vec<Card>
                .collect()
        }
        PokerHand::ThreeOfAKind => {
            // Find three of a kind
            group_by_rank(cards)
                .into_iter()
                .find_map(|(_, cards)| {
                    if cards.len() == 3 {
                        Some(cards.iter().map(|&card| *card).collect())
                    } else {
                        None
                    }
                })
                .unwrap_or_default()
        }
        PokerHand::FourOfAKind => {
            // Find four of a kind
            group_by_rank(cards)
                .into_iter()
                .find_map(|(_, cards)| {
                    if cards.len() == 4 {
                        Some(cards.iter().map(|&card| *card).collect())
                    } else {
                        None
                    }
                })
                .unwrap_or_default()
        }
        PokerHand::Straight => {
            if shortcut_active {
                if is_straight(cards) {
                    // Regular 5-card straight
                    cards.to_vec()
                } else if has_shortcut_straight(cards) {
                    // Find the 5 cards that form a shortcut straight
                    find_shortcut_straight_cards(cards)
                } else if four_fingers_active && has_four_card_shortcut_straight(cards) {
                    // Find the 4 cards that form a shortcut straight with Four Fingers
                    find_four_card_shortcut_straight(cards)
                } else if four_fingers_active && has_four_card_straight(cards) {
                    // Standard 4-card straight with Four Fingers
                    find_four_card_straight(cards)
                } else {
                    // Fallback
                    cards.to_vec()
                }
            } else {
                // Use the original logic for regular straights
                if four_fingers_active && !is_straight(cards) && has_four_card_straight(cards) {
                    // Find the 4 cards that form a straight
                    find_four_card_straight(cards)
                } else {
                    // Regular 5-card straight
                    cards.to_vec()
                }
            }
        }
        PokerHand::Flush => {
            if four_fingers_active
                && !is_flush(cards, smeared_joker_active)
                && has_four_card_flush(cards, smeared_joker_active)
            {
                // Find the suit with at least 4 cards
                let suit_groups = group_by_suit(cards, smeared_joker_active);
                if let Some((_, suit_cards)) = suit_groups
                    .iter()
                    .find(|(_, suit_cards)| suit_cards.len() >= 4 && suit_cards.len() < 5)
                {
                    // Take the first 4 cards of that suit
                    return suit_cards.iter().take(4).map(|&&card| card).collect();
                }
                vec![]
            } else {
                // Regular 5-card flush
                cards.to_vec()
            }
        }
        PokerHand::StraightFlush => {
            // For a straight flush with Four Fingers active, we need to carefully combine
            // the cards from both the flush and straight components
            if four_fingers_active {
                // First, identify the flush component (5-card or 4-card)
                let flush_cards = if is_flush(cards, smeared_joker_active) {
                    // Regular 5-card flush - all cards of the same suit
                    cards
                        .iter()
                        .filter(|card| {
                            let dominant_suit = cards
                                .iter()
                                .filter(|c| c.enhancement != Some(ortalib::Enhancement::Wild))
                                .fold(std::collections::HashMap::new(), |mut map, c| {
                                    *map.entry(c.suit).or_insert(0) += 1;
                                    map
                                })
                                .into_iter()
                                .max_by_key(|(_, count)| *count)
                                .map(|(suit, _)| suit)
                                .unwrap_or(card.suit);

                            card.enhancement == Some(ortalib::Enhancement::Wild)
                                || card.suit == dominant_suit
                        })
                        .copied()
                        .collect::<Vec<Card>>()
                } else if has_four_card_flush(cards, smeared_joker_active) {
                    // 4-card flush
                    find_four_card_flush(cards, smeared_joker_active)
                } else {
                    Vec::new() // No flush component
                };

                // Next, identify the straight component
                let straight_cards = if is_straight(cards) {
                    // Regular 5-card straight
                    cards.to_vec()
                } else if shortcut_active && has_shortcut_straight(cards) {
                    // 5-card straight with gaps
                    find_shortcut_straight_cards(cards)
                } else if has_four_card_straight(cards) {
                    // 4-card straight
                    find_four_card_straight(cards)
                } else if shortcut_active && has_four_card_shortcut_straight(cards) {
                    // 4-card straight with gaps
                    find_four_card_shortcut_straight(cards)
                } else {
                    Vec::new() // No straight component
                };

                // Combine the two components to get all relevant cards
                let mut combined_cards = flush_cards;
                for card in straight_cards {
                    if !combined_cards.contains(&card) {
                        combined_cards.push(card);
                    }
                }

                // If we have a combined result, return it
                if !combined_cards.is_empty() {
                    return combined_cards;
                }

                // Fallback to all cards
                return cards.to_vec();
            }

            // Without Four Fingers active
            if shortcut_active
                && find_shortcut_straight_flush_cards(cards, smeared_joker_active).len() == 5
            {
                return find_shortcut_straight_flush_cards(cards, smeared_joker_active);
            }

            // Default case - return all cards
            cards.to_vec()
        } // For these hands, all cards are scored
        PokerHand::FiveOfAKind
        | PokerHand::FlushHouse
        | PokerHand::FlushFive
        | PokerHand::FullHouse => cards.to_vec(),
    }
}

/// Analyses a hand of cards to determine what poker hand conditions exist
/// This is useful for jokers that activate based on the presence of certain hand conditions
#[derive(Debug, Default)]
pub struct HandConditions {
    pub contains_pair: bool,
    pub contains_two_pair: bool,
    pub contains_three_of_a_kind: bool,
    pub contains_straight: bool,
    pub contains_flush: bool,
}

/// Analyses a hand of cards to determine what poker hand conditions exist
/// This is useful for jokers that activate based on the presence of certain hand conditions
pub fn analyse_hand_conditions(
    cards: &[Card],
    four_fingers_active: bool,
    shortcut_active: bool,
    smeared_joker_active: bool,
) -> GameResult<HandConditions> {
    let mut conditions = HandConditions::default();

    // Analyse ranks to find pairs and three-of-a-kinds
    let rank_counts = group_rank(cards);

    // Check for pairs and three-of-a-kinds
    let mut different_pairs = std::collections::HashSet::new();

    for (&rank, &count) in &rank_counts {
        // A pair is defined as 2 or more cards of the same rank (for joker activation)
        // This is important for jokers like Jolly Joker that activate when hand "contains a pair"
        if count >= 2 {
            conditions.contains_pair = true;
            different_pairs.insert(rank);
        }

        // Three of a kind is 3+ cards of the same rank
        if count >= 3 {
            conditions.contains_three_of_a_kind = true;
        }
    }

    // Special case: two cards of the same rank always forms a pair
    if cards.len() == 2 && rank_counts.len() == 1 {
        conditions.contains_pair = true;
    }

    // Two Pair requires two different ranks with pairs
    conditions.contains_two_pair = different_pairs.len() >= 2;

    // Check for straight
    conditions.contains_straight = is_straight(cards)
        || (four_fingers_active && has_four_card_straight(cards))
        || (shortcut_active && has_shortcut_straight(cards))
        || (four_fingers_active && shortcut_active && has_four_card_shortcut_straight(cards));

    // Check for flush
    conditions.contains_flush = is_flush(cards, smeared_joker_active)
        || (four_fingers_active && has_four_card_flush(cards, smeared_joker_active));

    Ok(conditions)
}
