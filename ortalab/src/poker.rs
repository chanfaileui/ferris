use crate::errors::GameResult;
use enum_iterator::Sequence;
// use itertools::Itertools;
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

// /// Returns a IndexMap mapping each rank to the number of cards with that rank in played cards
// /// For example, {♣: 1, ♠: 1, ♥: 2, ♦: 1}
// fn group_suit(cards: &[Card]) -> IndexMap<Suit, usize> {
//     let mut suit_counts: IndexMap<Suit, usize> = IndexMap::new();

//     // if there are any wild cards, we need to count them as all suits
//     for card in cards {
//         if card.enhancement == Some(ortalib::Enhancement::Wild) {
//             for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
//                 *suit_counts.entry(suit).or_insert(0) += 1;
//             }
//         } else {
//             *suit_counts.entry(card.suit).or_insert(0) += 1;
//         }
//     }

//     suit_counts
// }

/// Returns a IndexMap mapping each suit to the number of cards with that suit in played cards
/// For example, if five 10s are played, the result will be {♠: [10♠], ♣: [10♣], ♥: [10♥, 10♥], ♦: [10♦]}
fn group_by_suit(cards: &[Card]) -> IndexMap<Suit, Vec<&Card>> {
    let mut suit_cards: IndexMap<Suit, Vec<&Card>> = IndexMap::new();

    // if there are any wild cards, we need to count them as all suits
    for card in cards {
        if card.enhancement == Some(ortalib::Enhancement::Wild) {
            for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
                suit_cards.entry(suit).or_default().push(card);
            }
        } else {
            suit_cards.entry(card.suit).or_default().push(card);
        }
    }

    suit_cards
}

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

// Check if there's a 4-card flush in the hand
fn has_four_card_flush(cards: &[Card]) -> bool {
    if cards.len() < 4 {
        return false;
    }

    // Group by suit
    let suit_groups = group_by_suit(cards);

    // Check if any suit appears at least 4 times
    suit_groups.values().any(|cards| cards.len() >= 4)
}

// Check if there's a 4-card straight in the hand
fn has_four_card_straight(cards: &[Card]) -> bool {
    if cards.len() < 4 {
        return false;
    }

    // Get unique ranks sorted
    let mut ranks: Vec<u8> = cards
        .iter()
        .map(|card| card.rank.rank_value() as u8)
        .collect();
    ranks.sort();
    ranks.dedup();

    // Check for 4 consecutive ranks
    for window in ranks.windows(4) {
        if window[3] - window[0] == 3 {
            return true;
        }
    }

    // Special case for A-2-3-4 (Ace low)
    if ranks.contains(&2) && ranks.contains(&3) && ranks.contains(&4) && ranks.contains(&14) {
        return true;
    }

    false
}

/// Determines if the cards form a flush (all cards of the same suit)
fn is_flush(cards: &[Card]) -> bool {
    if cards.len() < 5 {
        return false;
    }

    // Group by suit, considering Wild cards as every suit
    let suit_groups = group_by_suit(cards);

    // Check if any suit has enough cards for a flush
    suit_groups.values().any(|suit_cards| suit_cards.len() >= 5)
}

pub fn identify_hand(cards: &[Card], four_fingers_active: bool) -> GameResult<PokerHand> {
    // println!("group by rank: {:?}", group_rank(cards));
    // println!("group by rank: {:?}", group_by_rank(cards));
    // println!("group by suit: {:?}", group_suit(cards));
    // println!("group by suit: {:?}", group_by_suit(cards));

    let rank_count = group_rank(cards);
    let all_same_rank = rank_count.len() == 1;
    let has_flush = is_flush(cards);
    let has_straight = is_straight(cards);
    let has_three_two = has_three_two_pattern(cards);
    let has_four_of_a_kind = rank_count.values().any(|&count| count == 4);
    let has_three_of_a_kind = rank_count.values().any(|&count| count == 3);
    let pair_count = rank_count.values().filter(|&&count| count == 2).count();

    // Four Fingers joker support - check for 4-card patterns if active
    let has_four_card_flush = if four_fingers_active && cards.len() >= 4 {
        has_four_card_flush(cards)
    } else {
        false
    };

    let has_four_card_straight = if four_fingers_active && cards.len() >= 4 {
        has_four_card_straight(cards)
    } else {
        false
    };

    // Use combined flush check (5-card flush OR 4-card flush with Four Fingers)
    let effective_flush = has_flush || has_four_card_flush;

    // Use combined straight check (5-card straight OR 4-card straight with Four Fingers)
    let effective_straight = has_straight || has_four_card_straight;

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

    // 9. Straight Flush (sequential and same suit)
    if has_straight && effective_flush {
        return Ok(PokerHand::StraightFlush);
    }

    // 8. Four of a Kind
    if has_four_of_a_kind {
        return Ok(PokerHand::FourOfAKind);
    }

    // 7. Full House (three of one rank, two of another)
    if has_three_two {
        return Ok(PokerHand::FullHouse);
    }

    // 6. Flush (all same suit)
    if effective_flush {
        return Ok(PokerHand::Flush);
    }

    // 5. Straight (sequential cards)
    if effective_straight {
        return Ok(PokerHand::Straight);
    }

    // 4. Three of a Kind
    if has_three_of_a_kind {
        return Ok(PokerHand::ThreeOfAKind);
    }

    // 3. Two Pair
    if pair_count == 2 {
        return Ok(PokerHand::TwoPair);
    }

    // 2. Pair
    if pair_count == 1 {
        return Ok(PokerHand::Pair);
    }

    // 1. High Card (default when no other hand type matches)
    Ok(PokerHand::HighCard)
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
            if four_fingers_active && !is_straight(cards) && has_four_card_straight(cards) {
                // Find the 4 cards that form a straight
                let mut ranks: Vec<(usize, u8)> = cards
                    .iter()
                    .enumerate()
                    .map(|(i, card)| (i, card.rank.rank_value() as u8))
                    .collect();

                ranks.sort_by_key(|&(_, rank)| rank);

                // Check for consecutive sequences of 4 cards
                for window in ranks.windows(4) {
                    if window[3].1 - window[0].1 == 3 {
                        // Found 4 consecutive cards
                        return window.iter().map(|&(i, _)| cards[i]).collect();
                    }
                }

                // Check for A-2-3-4 straight
                let ace_indices: Vec<usize> = ranks
                    .iter()
                    .filter(|&&(_, rank)| rank == 14) // Ace
                    .map(|&(i, _)| i)
                    .collect();

                let low_cards: Vec<(usize, u8)> = ranks
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

                vec![]
            } else {
                // Regular 5-card straight
                cards.to_vec()
            }
        }
        PokerHand::Flush => {
            if four_fingers_active && !is_flush(cards) && has_four_card_flush(cards) {
                // Find the suit with at least 4 cards
                let suit_groups = group_by_suit(cards);
                if let Some((_, suit_cards)) = suit_groups
                    .iter()
                    .find(|(_, suit_cards)| suit_cards.len() >= 4 && suit_cards.len() < 5)
                {
                    // Take the first 4 cards of that suit
                    return suit_cards
                        .iter()
                        .take(4)
                        .map(|&&card| card)
                        .collect();
                }
                vec![]
            } else {
                // Regular 5-card flush
                cards.to_vec()
            }
        }
        PokerHand::StraightFlush => {
            if four_fingers_active && !(is_straight(cards) && is_flush(cards)) {
                // This is a complex case - we might have a 4-card straight and a 4-card flush
                // which might not be the same 4 cards

                // First check if we have a 4-card straight flush (same 4 cards)
                let suit_groups = group_by_suit(cards);

                for (_, suit_cards) in suit_groups {
                    if suit_cards.len() >= 4
                        && has_four_card_straight(
                            &suit_cards
                                .iter()
                                .map(|&&c| c)
                                .collect::<Vec<Card>>(),
                        )
                    {
                        // We found a 4-card straight flush
                        return suit_cards.iter().map(|&&c| c).collect();
                    }
                }

                // If we reach here, we might have a 4-card straight and a 4-card flush on different cards
                // The assignment suggests this should still count as a straight flush
                // This is the most complex case and would need detailed implementation
                // For simplicity, just return all cards for now
                cards.to_vec()
            } else {
                // Regular 5-card straight flush
                cards.to_vec()
            }
        }
        // For these hands, all cards are scored
        PokerHand::FiveOfAKind
        | PokerHand::FlushHouse
        | PokerHand::FlushFive
        | PokerHand::FullHouse => cards.to_vec(),
    }
}
#[derive(Debug, Default)]
pub struct HandConditions {
    pub contains_pair: bool,
    pub contains_two_pair: bool,
    pub contains_three_of_a_kind: bool,
    pub contains_straight: bool,
    pub contains_flush: bool,
}

/// Analyzes a hand of cards to determine what poker hand conditions exist
/// This is useful for jokers that activate based on the presence of certain hand conditions
pub fn analyze_hand_conditions(
    cards: &[Card],
    four_fingers_active: bool,
) -> GameResult<HandConditions> {
    let mut conditions = HandConditions::default();

    // Analyze ranks to find pairs and three-of-a-kinds
    let mut rank_counts = std::collections::HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
    }

    // Check for pairs and three-of-a-kinds
    let mut different_pairs = std::collections::HashSet::new();

    for (&rank, &count) in &rank_counts {
        if count >= 2 {
            conditions.contains_pair = true;
            different_pairs.insert(rank);
        }
        if count >= 3 {
            conditions.contains_three_of_a_kind = true;
        }
    }

    // Two Pair requires two different ranks with pairs
    conditions.contains_two_pair = different_pairs.len() >= 2;

    // Check for straight (5-card or 4-card with Four Fingers)
    conditions.contains_straight =
        is_straight(cards) || (four_fingers_active && has_four_card_straight(cards));

    // Check for flush (5-card or 4-card with Four Fingers)
    conditions.contains_flush =
        is_flush(cards) || (four_fingers_active && has_four_card_flush(cards));

    Ok(conditions)
}
