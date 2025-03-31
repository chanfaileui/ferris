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

fn is_sequential(cards: &[Card]) -> bool {
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

pub fn identify_hand(cards: &[Card]) -> GameResult<PokerHand> {
    // println!("group by rank: {:?}", group_rank(cards));
    // println!("group by rank: {:?}", group_by_rank(cards));
    // println!("group by suit: {:?}", group_suit(cards));
    // println!("group by suit: {:?}", group_by_suit(cards));

    let rank_count: IndexMap<ortalib::Rank, usize> = group_rank(cards);
    // let suit_count: IndexMap<ortalib::Suit, usize> = group_suit(cards);

    // 1. Are all 5 cards the same rank?
    if rank_count.len() == 1 {
        // are they the same suit?
        if is_flush(cards) {
            return Ok(PokerHand::FlushFive);
        } else {
            return Ok(PokerHand::FiveOfAKind);
        }
    }

    // 2. Are all 5 cards the same suit?
    if is_flush(cards) {
        // Check if sequential
        if has_three_two_pattern(cards) {
            return Ok(PokerHand::FlushHouse);
        } else if is_sequential(cards) {
            return Ok(PokerHand::StraightFlush);
        } else {
            return Ok(PokerHand::Flush);
        }
    }

    // 3. Are 4 cards the same rank?
    if rank_count.values().any(|&count| count == 4) {
        return Ok(PokerHand::FourOfAKind);
    }

    // 4. Is it a 3+2 pattern? (Three of one rank, two of another)
    if has_three_two_pattern(cards) {
        return Ok(PokerHand::FullHouse);
    }

    // 5. Are 5 cards sequential?
    if is_sequential(cards) {
        return Ok(PokerHand::Straight);
    }

    // 6. Are 3 cards the same rank?
    for &count in rank_count.values() {
        if count == 3 {
            return Ok(PokerHand::ThreeOfAKind);
        }
    }

    // 7. Are there two pairs?
    if rank_count.values().filter(|&&count| count == 2).count() == 2 {
        return Ok(PokerHand::TwoPair);
    }

    // 8. Is there one pair?
    for &count in rank_count.values() {
        if count == 2 {
            return Ok(PokerHand::Pair);
        }
    }

    // if none of the above, it's a high card
    Ok(PokerHand::HighCard)
}

/// Returns the cards that contribute to the scoring for a given poker hand
///
/// According to the rules, generally only the cards relevant to the poker hand
/// are scored, and all others are unscored. This function identifies which cards
/// should be scored based on the poker hand type.
pub fn get_scoring_cards(hand_type: &PokerHand, cards: &[Card]) -> Vec<Card> {
    match hand_type {
        PokerHand::HighCard => {
            // For high card, only the highest card scores
            let rank_map: IndexMap<Rank, Vec<&Card>> = group_by_rank(cards);
            let mut ranks: Vec<Rank> = rank_map.keys().cloned().collect();
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
                        let pair_cards = cards
                            .iter()
                            .map(|&card| {
                                dbg!(card);
                                *card
                            })
                            .collect::<Vec<Card>>();
                        Some(pair_cards)
                    } else {
                        None
                    }
                })
                .flatten() // Flatten the Vec<Vec<Card>> into Vec<Card>
                .collect()
        }
        PokerHand::ThreeOfAKind => {
            // Find the pair
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
            // Find the pair
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
        // for these hands, all cards are scored
        PokerHand::FiveOfAKind
        | PokerHand::FlushHouse
        | PokerHand::FlushFive
        | PokerHand::Straight
        | PokerHand::StraightFlush
        | PokerHand::Flush
        | PokerHand::FullHouse => cards.to_vec(),
    }
}
