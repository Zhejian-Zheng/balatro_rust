//! src/util/util.rs
//! This file is for storing help function and avoid the duplicate codes

use ortalib::{Card, Enhancement, Rank, Suit};
use std::collections::HashMap;

/// Check if the card is a wild card or not
/// # Parameters
/// * `card` - an variable with type Card
pub fn is_wild(card: &Card) -> bool {
    card.enhancement == Some(Enhancement::Wild)
}

/// Returns a HashMap mapping each Rank to its occurrence count.
pub fn get_rank_count(cards: &[Card]) -> HashMap<Rank, usize> {
    let mut count = HashMap::new();
    for card in cards {
        *count.entry(card.rank).or_insert(0) += 1;
    }
    count
}

/// Checks if all cards have the same suit.
pub fn is_flush(cards_played: &[Card], four_fingers: bool, has_smeared: bool) -> bool {
    let min_cards = if four_fingers { 4 } else { 5 };
    if cards_played.len() < min_cards {
        return false;
    }

    let suit_counts = get_suit_count(cards_played, has_smeared);
    // If any suit's count equals the total number of cards
    suit_counts.values().any(|&count| count >= min_cards)
}

/// Check if the card can make to become a straight
pub fn is_straight(cards: &[Card], has_fourfingers: bool, has_shortcut: bool) -> bool {
    let min_cards = if has_fourfingers { 4 } else { 5 };
    if cards.len() < min_cards {
        return false;
    }

    // Only keep the cards with unique rank
    let mut unique_cards = cards.to_vec();
    unique_cards.sort_by_key(rank_numeric);
    unique_cards.reverse();
    unique_cards.dedup_by_key(|c| c.rank);

    if unique_cards.len() < min_cards {
        return false;
    }

    let allowed_gap = if has_shortcut { 2 } else { 1 };

    if unique_cards.len() == 5 {
        if check_normal_ace_straight(&unique_cards, 5, allowed_gap) {
            return true;
        }
        if check_low_ace_straight(&unique_cards, 5, allowed_gap) {
            return true;
        }
    }

    if has_fourfingers && unique_cards.len() >= 4 {
        if check_normal_ace_straight(&unique_cards, 4, allowed_gap) {
            return true;
        }
        if check_low_ace_straight(&unique_cards, 4, allowed_gap) {
            return true;
        }
    }

    false
}

pub fn rank_numeric(card: &Card) -> u8 {
    match card.rank {
        Rank::Two => 2,
        Rank::Three => 3,
        Rank::Four => 4,
        Rank::Five => 5,
        Rank::Six => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine => 9,
        Rank::Ten => 10,
        Rank::Jack => 11,
        Rank::Queen => 12,
        Rank::King => 13,
        Rank::Ace => 14,
    }
}

/// -------------- Helper -------------- ///
/// Returns a HashMap mapping each Suit to its occurrence count.
fn get_suit_count(cards: &[Card], has_smeared: bool) -> HashMap<Suit, usize> {
    let suits = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
    let mut count: HashMap<Suit, usize> = suits.iter().map(|&s| (s, 0)).collect();

    for card in cards {
        if is_wild(card) {
            // if the card is wild, then all the suit number +1
            for value in count.values_mut() {
                *value += 1;
            }
        } else if has_smeared {
            // pub fn other_suit_of_same_color(&self) -> Suit
            // pub fn color(&self) -> SuitColor
            *count.entry(card.suit).or_insert(0) += 1;
            let other_suit = card.suit.other_suit_of_same_color();
            *count.entry(other_suit).or_insert(0) += 1;
        } else {
            *count.entry(card.suit).or_insert(0) += 1;
        }
    }
    count
}

fn check_normal_ace_straight(unique_cards: &[Card], straight_size: usize, allowed_gap: u8) -> bool {
    let num_cards = unique_cards.len();
    // if straight size is 5 start at 0
    // if size can be 4, we can start at 1 or 0
    for start in 0..=num_cards - straight_size {
        let mut is_seq = true;
        for i in start..(start + straight_size) {
            // when i is not the first element
            if i > start
                && rank_numeric(&unique_cards[i - 1]) - rank_numeric(&unique_cards[i]) > allowed_gap
            {
                is_seq = false;
                break;
            }
        }
        if is_seq {
            return true;
        }
    }
    false
}

fn check_low_ace_straight(unique_cards: &[Card], straight_size: usize, allowed_gap: u8) -> bool {
    let mut has_ace = false;
    for card in unique_cards.iter() {
        if card.rank == Rank::Ace {
            has_ace = true;
            break;
        }
    }

    if !has_ace {
        return false;
    }

    let ace_value = |c: &Card| {
        if c.rank == Rank::Ace {
            1
        } else {
            rank_numeric(c)
        }
    };

    let mut unique_cards_ace_low = unique_cards.to_vec();
    unique_cards_ace_low.sort_by_key(ace_value);
    unique_cards_ace_low.reverse();

    if unique_cards_ace_low.len() < straight_size {
        return false;
    }

    let num_cards = unique_cards_ace_low.len();

    for start in 0..=num_cards - straight_size {
        let mut is_seq = true;
        for i in start..(start + straight_size) {
            // when i is not the first element
            if i > start
                && ace_value(&unique_cards_ace_low[i - 1]) - ace_value(&unique_cards_ace_low[i])
                    > allowed_gap
            {
                is_seq = false;
                break;
            }
        }
        if is_seq {
            return true;
        }
    }
    false
}

// ------------------------------------------------------ //
// ------------------------ Test ------------------------ //
// ------------------------------------------------------ //
#[cfg(test)]
mod tests {
    use super::*;
    use ortalib::{Card, Enhancement, Rank, Suit};

    fn make_card(rank: Rank, suit: Suit, enhancement: Option<Enhancement>) -> Card {
        Card::new(rank, suit, enhancement, None)
    }

    #[test]
    fn test_is_wild() {
        let wild_card = make_card(Rank::Ace, Suit::Spades, Some(Enhancement::Wild));
        let normal_card = make_card(Rank::Ace, Suit::Spades, None);
        assert!(is_wild(&wild_card));
        assert!(!is_wild(&normal_card));
    }

    #[test]
    fn test_get_rank_count() {
        let card1 = make_card(Rank::Ace, Suit::Spades, None);
        let card2 = make_card(Rank::Ace, Suit::Hearts, None);
        let card3 = make_card(Rank::King, Suit::Diamonds, None);
        let cards = vec![card1, card2, card3];
        let count = get_rank_count(&cards);
        assert_eq!(count.get(&Rank::Ace), Some(&2));
        assert_eq!(count.get(&Rank::King), Some(&1));
    }

    #[test]
    fn test_is_flush() {
        // Test flush with cards all of the same suit (no wild or smeared cards).
        let card1 = make_card(Rank::Ace, Suit::Spades, None);
        let card2 = make_card(Rank::King, Suit::Spades, None);
        let card3 = make_card(Rank::Queen, Suit::Spades, None);
        let card4 = make_card(Rank::Jack, Suit::Spades, None);
        let card5 = make_card(Rank::Ten, Suit::Spades, None);
        let cards = vec![card1, card2, card3, card4, card5];
        let cards2 = vec![card1, card2, card3, card4];
        assert!(is_flush(&cards, false, false));
        assert!(is_flush(&cards2, true, false));

        // Test flush with a wild card.
        // A wild card is counted toward every suit.
        let wild_card = make_card(Rank::Two, Suit::Hearts, Some(Enhancement::Wild));
        let cards_with_wild = vec![wild_card, card1, card2, card3];
        // In four-fingers mode the minimum required cards is 4.
        assert!(is_flush(&cards_with_wild, true, false));
    }

    #[test]
    fn test_is_straight() {
        // Test flush with cards all of the same suit (no wild or smeared cards).
        let card14 = make_card(Rank::Ace, Suit::Spades, None);
        // let card13 = make_card(Rank::King, Suit::Spades, None);
        // let card12 = make_card(Rank::Queen, Suit::Spades, None);
        // let card11 = make_card(Rank::Jack, Suit::Spades, None);
        let card10 = make_card(Rank::Ten, Suit::Spades, None);
        // let card9 = make_card(Rank::Nine, Suit::Spades, None);
        let card8 = make_card(Rank::Eight, Suit::Spades, None);
        // let card7 = make_card(Rank::Seven, Suit::Spades, None);
        let card6 = make_card(Rank::Six, Suit::Spades, None);
        let card5 = make_card(Rank::Five, Suit::Spades, None);
        let card4 = make_card(Rank::Four, Suit::Spades, None);
        let card3 = make_card(Rank::Three, Suit::Spades, None);
        let card2 = make_card(Rank::Two, Suit::Spades, None);

        let cards_short_cut = vec![card2, card4, card6, card8, card10];
        let cards_shortcut_fourfinger = vec![card2, card4, card6, card8];
        let cards_low_ace = vec![card14, card2, card3, card4, card5];
        assert!(!is_straight(&cards_short_cut, false, false));
        assert!(is_straight(&cards_short_cut, false, true));
        assert!(is_straight(&cards_shortcut_fourfinger, true, true));
        assert!(is_straight(&cards_low_ace, true, false));
    }

    #[test]
    fn test_rank_numeric() {
        let card_two = make_card(Rank::Two, Suit::Hearts, None);
        assert_eq!(rank_numeric(&card_two), 2);
        let card_ace = make_card(Rank::Ace, Suit::Spades, None);
        assert_eq!(rank_numeric(&card_ace), 14);
    }
}
