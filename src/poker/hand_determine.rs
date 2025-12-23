//! src/poker/hand_determine.rs
//! This file is used for identifying and classifying poker hands.
//! Its core functionality is to analyse a set of cards and determine the best possible poker hand they form.

use crate::util::{get_rank_count, is_flush, rank_numeric};
use ortalib::{Card, PokerHand, Rank};

/// Determine the best possible poker hand and return a tuple containing:
/// - the identified PokerHand variant, and
/// - a vector of cards that contribute to the scoring of that hand.
///
/// # Parameters
/// * `cards` - an vector of card which the player is handing out.
///
/// # Return
/// * `PokerHand` - the poker hand the cards can form
/// * `Vec<Card>` - the cards we need in the played card to form up the poker hand
pub fn determine_best_hand(
    cards: &[Card],
    has_fourfingers: bool,
    has_shortcut: bool,
    has_smeared: bool,
) -> (PokerHand, Vec<Card>) {
    if let Some(flush_five) = check_flush_five(cards, has_smeared) {
        return (PokerHand::FlushFive, flush_five);
    }

    if let Some(flush_house) = check_flush_house(cards, has_fourfingers, has_smeared) {
        return (PokerHand::FlushHouse, flush_house);
    }

    if let Some(five_of_a_kind) = check_five_of_a_kind(cards) {
        return (PokerHand::FiveOfAKind, five_of_a_kind);
    }

    if let Some(straight_flush) =
        check_straight_flush(cards, has_fourfingers, has_shortcut, has_smeared)
    {
        return (PokerHand::StraightFlush, straight_flush);
    }

    if let Some(four_of_a_kind) = check_four_of_a_kind(cards) {
        return (PokerHand::FourOfAKind, four_of_a_kind);
    }

    if let Some(full_house) = check_full_house(cards) {
        return (PokerHand::FullHouse, full_house);
    }

    if let Some(flush) = check_flush(cards, has_fourfingers, has_smeared) {
        return (PokerHand::Flush, flush);
    }

    if let Some(straight) = check_straight(cards, has_fourfingers, has_shortcut) {
        return (PokerHand::Straight, straight);
    }

    if let Some(three_of_a_kind) = check_three_of_a_kind(cards) {
        return (PokerHand::ThreeOfAKind, three_of_a_kind);
    }

    if let Some(two_pair) = check_two_pair(cards) {
        return (PokerHand::TwoPair, two_pair);
    }

    if let Some(one_pair) = check_one_pair(cards) {
        return (PokerHand::Pair, one_pair);
    }

    // Default: High Card – return the card with the highest rank.
    let high_card = cards
        .iter()
        .max_by_key(|c| c.rank)
        .cloned()
        .expect("Can not get a scored card from an empty played card set");

    (PokerHand::HighCard, vec![high_card])
}

// ----------------- Helper Functions ----------------- //
fn check_flush_five(cards: &[Card], has_smeared: bool) -> Option<Vec<Card>> {
    // check if all the suit and rank of the cards are the same
    // if the card number is less than 5, it is impossible to form flushfive
    if cards.len() < 5 {
        return None;
    }

    // in this case we can not have has_fourfingers is true
    if !is_flush(cards, false, has_smeared) {
        return None;
    }

    check_five_of_a_kind(cards)
}

fn check_flush_house(
    cards: &[Card],
    has_fourfingers: bool,
    has_smeared: bool,
) -> Option<Vec<Card>> {
    // Flush House requires exactly 5 cards.
    if cards.len() < 5 {
        return None;
    }

    // check if this is a flush
    if !is_flush(cards, has_fourfingers, has_smeared) {
        return None;
    }

    // check if this is a full house.
    check_full_house(cards)
}

fn check_five_of_a_kind(cards: &[Card]) -> Option<Vec<Card>> {
    if cards.len() < 5 {
        return None;
    }
    let rank_counts = get_rank_count(cards);
    for (&rank, &count) in rank_counts.iter() {
        if count >= 5 {
            let selected: Vec<Card> = cards
                .iter()
                .filter(|c| c.rank == rank)
                .cloned()
                .take(5)
                .collect();
            if selected.len() == 5 {
                return Some(selected);
            }
        }
    }

    None
}

fn check_straight_flush(
    cards: &[Card],
    has_fourfingers: bool,
    has_shortcut: bool,
    has_smeared: bool,
) -> Option<Vec<Card>> {
    if !is_flush(cards, has_fourfingers, has_smeared) {
        return None;
    }

    check_straight(cards, has_fourfingers, has_shortcut)
}

fn check_four_of_a_kind(cards: &[Card]) -> Option<Vec<Card>> {
    if cards.len() < 4 {
        return None;
    }

    let rank_counts = get_rank_count(cards);
    for (&rank, &count) in rank_counts.iter() {
        if count >= 4 {
            let selected: Vec<Card> = cards
                .iter()
                .filter(|c| c.rank == rank)
                .cloned()
                .take(4)
                .collect();
            if selected.len() == 4 {
                return Some(selected);
            }
        }
    }

    None
}

fn check_full_house(cards: &[Card]) -> Option<Vec<Card>> {
    if cards.len() != 5 {
        return None;
    }

    let rank_counts = get_rank_count(cards);

    // we need exactly two different ranks
    if rank_counts.len() != 2 {
        return None;
    }

    let mut found_three = false;
    let mut found_two = false;
    for &count in rank_counts.values() {
        if count == 3 {
            found_three = true;
        } else if count == 2 {
            found_two = true;
        }
    }

    if found_three && found_two {
        return Some(cards.to_vec());
    }

    None
}

fn check_flush(cards: &[Card], has_fourfingers: bool, has_smeared: bool) -> Option<Vec<Card>> {
    if is_flush(cards, has_fourfingers, has_smeared) {
        return Some(cards.to_vec());
    }
    None
}

fn check_two_pair(cards: &[Card]) -> Option<Vec<Card>> {
    // Find the first pair.
    if let Some(first_pair) = check_one_pair(cards) {
        let mut remaining_cards = cards.to_vec();
        // Remove the cards in the first pair from the cards
        for card in &first_pair {
            if let Some(index) = remaining_cards.iter().position(|c| c == card) {
                remaining_cards.remove(index);
            }
        }
        // Try to find a second pair in the remaining cards.
        if let Some(second_pair) = check_one_pair(&remaining_cards) {
            // Combine both pairs
            let mut two_pair = first_pair;
            two_pair.extend(second_pair);
            return Some(two_pair);
        }
    }
    None
}

fn check_straight(cards: &[Card], has_fourfingers: bool, has_shortcut: bool) -> Option<Vec<Card>> {
    let min_cards = if has_fourfingers { 4 } else { 5 };
    if cards.len() < min_cards {
        return None;
    }

    // Only keep the cards with unique rank
    let mut unique_cards = cards.to_vec();
    unique_cards.sort_by_key(rank_numeric);
    unique_cards.reverse();
    unique_cards.dedup_by_key(|c| c.rank);

    if unique_cards.len() < min_cards {
        return None;
    }

    let allowed_gap = if has_shortcut { 2 } else { 1 };

    if unique_cards.len() == 5 {
        if let Some(seq) = check_normal_ace_straight(&unique_cards, 5, allowed_gap) {
            return Some(seq);
        }
        if let Some(seq) = check_low_ace_straight(&unique_cards, 5, allowed_gap) {
            return Some(seq);
        }
    }

    if has_fourfingers && unique_cards.len() >= 4 {
        if let Some(seq) = check_normal_ace_straight(&unique_cards, 4, allowed_gap) {
            return Some(seq);
        }
        if let Some(seq) = check_low_ace_straight(&unique_cards, 4, allowed_gap) {
            return Some(seq);
        }
    }

    None
}

fn check_normal_ace_straight(
    unique_cards: &[Card],
    straight_size: usize,
    allowed_gap: u8,
) -> Option<Vec<Card>> {
    let num_cards = unique_cards.len();
    // if straight size is 5 start at 0
    // if size can be 4, we can start at 1 or 0
    for start in 0..=num_cards - straight_size {
        let mut is_seq = true;
        let mut score_cards = Vec::new();
        for i in start..(start + straight_size) {
            // when i is not the first element
            if i > start
                && rank_numeric(&unique_cards[i - 1]) - rank_numeric(&unique_cards[i]) > allowed_gap
            {
                is_seq = false;
                break;
            }
            score_cards.push(unique_cards[i]);
        }
        if is_seq {
            return Some(score_cards);
        }
    }
    None
}

fn check_low_ace_straight(
    unique_cards: &[Card],
    straight_size: usize,
    allowed_gap: u8,
) -> Option<Vec<Card>> {
    let mut has_ace = false;
    for card in unique_cards.iter() {
        if card.rank == Rank::Ace {
            has_ace = true;
            break;
        }
    }

    if !has_ace {
        return None;
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
        return None;
    }

    let num_cards = unique_cards_ace_low.len();

    for start in 0..=num_cards - straight_size {
        let mut is_seq = true;
        let mut score_cards = Vec::new();
        for i in start..(start + straight_size) {
            // when i is not the first element
            if i > start
                && ace_value(&unique_cards_ace_low[i - 1]) - ace_value(&unique_cards_ace_low[i])
                    > allowed_gap
            {
                is_seq = false;
                break;
            }
            score_cards.push(unique_cards_ace_low[i]);
        }
        if is_seq {
            return Some(score_cards);
        }
    }
    None
}

fn check_three_of_a_kind(cards: &[Card]) -> Option<Vec<Card>> {
    if cards.len() < 3 {
        return None;
    }
    let rank_counts = get_rank_count(cards);
    for (&rank, &count) in rank_counts.iter() {
        if count >= 3 {
            let selected: Vec<Card> = cards
                .iter()
                .filter(|c| c.rank == rank)
                .cloned()
                .take(3)
                .collect();
            if selected.len() == 3 {
                return Some(selected);
            }
        }
    }
    None
}

fn check_one_pair(cards: &[Card]) -> Option<Vec<Card>> {
    let rank_counts = get_rank_count(cards);
    for (&rank, &count) in rank_counts.iter() {
        if count >= 2 {
            let selected: Vec<Card> = cards
                .iter()
                .filter(|c| c.rank == rank)
                .cloned()
                .take(2)
                .collect();
            if selected.len() == 2 {
                return Some(selected);
            }
        }
    }

    None
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
    fn test_flush_five() {
        let wild_card = make_card(Rank::Ace, Suit::Diamonds, Some(Enhancement::Wild));
        let card1 = make_card(Rank::Ace, Suit::Spades, None);
        let card2 = make_card(Rank::Ace, Suit::Clubs, None);
        let card3 = make_card(Rank::Ace, Suit::Spades, None);
        let card4 = make_card(Rank::Ace, Suit::Spades, None);
        let cards = vec![
            card1.clone(),
            card2.clone(),
            card3.clone(),
            card4.clone(),
            wild_card.clone(),
        ];
        // When there is smeared joker
        let (hand, selected) = determine_best_hand(&cards, false, false, true);
        assert_eq!(hand, PokerHand::FlushFive);
        assert_eq!(selected.len(), 5);

        // when no Smeared Joker
        let (hand, selected) = determine_best_hand(&cards, false, false, false);
        assert_eq!(hand, PokerHand::FiveOfAKind);
        assert_eq!(selected.len(), 5);
    }

    #[test]
    fn test_flush_house() {
        let wild_card = make_card(Rank::Ace, Suit::Diamonds, Some(Enhancement::Wild));
        let card1 = make_card(Rank::Ace, Suit::Spades, None);
        let card2 = make_card(Rank::Ace, Suit::Clubs, None);
        let card3 = make_card(Rank::Two, Suit::Clubs, None);
        let card4 = make_card(Rank::Two, Suit::Spades, None);
        let cards = vec![
            card1.clone(),
            card2.clone(),
            card3.clone(),
            card4.clone(),
            wild_card.clone(),
        ];
        // When there is smeared joker
        let (hand, selected) = determine_best_hand(&cards, false, false, true);
        assert_eq!(hand, PokerHand::FlushHouse);
        assert_eq!(selected.len(), 5);

        // when no Smeared Joker
        let (hand, selected) = determine_best_hand(&cards, false, false, false);
        assert_eq!(hand, PokerHand::FullHouse);
        assert_eq!(selected.len(), 5);
    }

    #[test]
    fn test_five_of_a_kind() {
        let wild_card = make_card(Rank::Ace, Suit::Diamonds, Some(Enhancement::Wild));
        let card1 = make_card(Rank::Ace, Suit::Spades, None);
        let card2 = make_card(Rank::Ace, Suit::Clubs, None);
        let card3 = make_card(Rank::Ace, Suit::Diamonds, None);
        let card4 = make_card(Rank::Ace, Suit::Clubs, None);
        let card5 = make_card(Rank::Two, Suit::Spades, None);
        let cards1 = vec![
            card1.clone(),
            card2.clone(),
            card3.clone(),
            card4.clone(),
            wild_card.clone(),
        ];

        let cards2 = vec![
            card1.clone(),
            card2.clone(),
            card3.clone(),
            card5.clone(),
            wild_card.clone(),
        ];

        let (hand, selected) = determine_best_hand(&cards1, false, false, false);
        assert_eq!(hand, PokerHand::FiveOfAKind);
        assert_eq!(selected.len(), 5);

        let (hand, selected) = determine_best_hand(&cards2, false, false, false);
        assert_eq!(hand, PokerHand::FourOfAKind);
        assert_eq!(selected.len(), 4);
    }

    #[test]
    fn test_straight_flush() {
        let card1 = make_card(Rank::Ten, Suit::Spades, None);
        let card2_s = make_card(Rank::Jack, Suit::Spades, None);
        let card2_c = make_card(Rank::Jack, Suit::Clubs, None);
        let card2_wild = make_card(Rank::Jack, Suit::Clubs, Some(Enhancement::Wild));
        let card3 = make_card(Rank::Queen, Suit::Spades, None);
        let card4 = make_card(Rank::King, Suit::Spades, None);
        let card5 = make_card(Rank::Ace, Suit::Spades, None);

        // when Ace considered as a low card
        let card7 = make_card(Rank::Two, Suit::Spades, None);
        let card8_s = make_card(Rank::Three, Suit::Spades, None);
        let card8_c = make_card(Rank::Three, Suit::Clubs, None);
        let card9 = make_card(Rank::Four, Suit::Spades, None);
        let card10 = make_card(Rank::Five, Suit::Spades, None);

        let cards = vec![card1, card2_s, card3, card4, card5];
        let cards2 = vec![card1, card2_c, card3, card4, card5];
        let cards3 = vec![card1, card2_wild, card3, card4, card5];
        let cards4 = vec![card1, card2_s, card3, card4];

        let cards5 = vec![card5, card7, card8_s, card9, card10];
        let cards6 = vec![card5, card7, card8_c, card9, card10];
        // has_smeared set to true to let is_flush return true.
        let (hand, selected) = determine_best_hand(&cards, false, false, false);
        assert_eq!(hand, PokerHand::StraightFlush);
        assert_eq!(selected.len(), 5);

        let (hand, selected) = determine_best_hand(&cards2, false, false, false);
        assert_eq!(hand, PokerHand::Straight);
        assert_eq!(selected.len(), 5);

        // when smeared
        let (hand, selected) = determine_best_hand(&cards2, false, false, true);
        assert_eq!(hand, PokerHand::StraightFlush);
        assert_eq!(selected.len(), 5);

        let (hand, selected) = determine_best_hand(&cards3, false, false, false);
        assert_eq!(hand, PokerHand::StraightFlush);
        assert_eq!(selected.len(), 5);

        // when four finger
        let (hand, selected) = determine_best_hand(&cards4, true, false, false);
        assert_eq!(hand, PokerHand::StraightFlush);
        assert_eq!(selected.len(), 4);

        // when every effect joker is on
        let (hand, selected) = determine_best_hand(&cards4, true, true, true);
        assert_eq!(hand, PokerHand::StraightFlush);
        assert_eq!(selected.len(), 4);

        // when low ace case
        let (hand, selected) = determine_best_hand(&cards5, false, false, false);
        assert_eq!(hand, PokerHand::StraightFlush);
        assert_eq!(selected.len(), 5);

        // when low ace case with smeared
        let (hand, selected) = determine_best_hand(&cards6, false, false, true);
        assert_eq!(hand, PokerHand::StraightFlush);
        assert_eq!(selected.len(), 5);
    }

    #[test]
    fn test_three_of_a_kind() {
        let card1 = make_card(Rank::Ten, Suit::Spades, None);
        let card2 = make_card(Rank::Ten, Suit::Hearts, None);
        let card3 = make_card(Rank::Ten, Suit::Clubs, None);
        let card4 = make_card(Rank::Queen, Suit::Diamonds, None);
        let card5 = make_card(Rank::Ace, Suit::Spades, None);
        let cards = vec![card1, card2, card3, card4, card5];
        let (hand, selected) = determine_best_hand(&cards, false, false, false);
        assert_eq!(hand, PokerHand::ThreeOfAKind);
        assert_eq!(selected.len(), 3);
    }

    #[test]
    fn test_two_pair() {
        let card1 = make_card(Rank::Ten, Suit::Spades, None);
        let card2 = make_card(Rank::Ten, Suit::Hearts, None);
        let card3 = make_card(Rank::Queen, Suit::Clubs, None);
        let card4 = make_card(Rank::Queen, Suit::Diamonds, None);
        let card5 = make_card(Rank::Ace, Suit::Spades, None);
        let cards = vec![card1, card2, card3, card4, card5];
        let (hand, selected) = determine_best_hand(&cards, false, false, false);
        assert_eq!(hand, PokerHand::TwoPair);
        assert_eq!(selected.len(), 4);
    }

    #[test]
    fn test_pair() {
        let card1 = make_card(Rank::Ten, Suit::Spades, None);
        let card2 = make_card(Rank::Ten, Suit::Hearts, None);
        let card3 = make_card(Rank::Jack, Suit::Clubs, None);
        let card4 = make_card(Rank::Queen, Suit::Diamonds, None);
        let card5 = make_card(Rank::Ace, Suit::Spades, None);
        let cards = vec![card1, card2, card3, card4, card5];
        let (hand, selected) = determine_best_hand(&cards, false, false, false);
        assert_eq!(hand, PokerHand::Pair);
        assert_eq!(selected.len(), 2);
    }
}
