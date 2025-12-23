use crate::util::{
    // get_rank_count,
    // is_flush,
    rank_numeric,
};
/// src/jokers/on_held.rs
/// This file is used for:
/// Implement the jokers that will effect the game by the on_held cards:
use ortalib::{Card, Chips, Joker, JokerCard, Mult, Rank};

/// A trait defining how a On held Jokers effect are applied during a round.
pub trait OnHeldJokerEffect {
    fn apply(&self, cards_in_hand: &[Card], chips: Chips, mult: Mult) -> (Chips, Mult);
}

pub struct RaisedFistEffect;
pub struct BaronEffect;
pub struct MimeEffect;
pub struct ElseJokerEffect;

impl OnHeldJokerEffect for RaisedFistEffect {
    fn apply(&self, cards_in_hand: &[Card], chips: Chips, mult: Mult) -> (Chips, Mult) {
        let lowest_rank_card = get_lowest_rank(cards_in_hand);
        let lowest_rank_value = lowest_rank_card.rank.rank_value();
        (chips, mult + (lowest_rank_value * 2.0))
    }
}

impl OnHeldJokerEffect for BaronEffect {
    fn apply(&self, cards_in_hand: &[Card], chips: Chips, mult: Mult) -> (Chips, Mult) {
        let mut new_mult = mult;
        for card in cards_in_hand {
            if card.rank == Rank::King {
                new_mult *= 1.5;
            }
        }
        (chips, new_mult)
    }
}

impl OnHeldJokerEffect for MimeEffect {
    fn apply(&self, _cards_in_hand: &[Card], chips: Chips, mult: Mult) -> (Chips, Mult) {
        (chips, mult)
    }
}

impl OnHeldJokerEffect for ElseJokerEffect {
    fn apply(&self, _cards_in_hand: &[Card], chips: Chips, mult: Mult) -> (Chips, Mult) {
        (chips, mult)
    }
}

pub fn get_on_held_joker_effect(joker_card: &JokerCard) -> Box<dyn OnHeldJokerEffect> {
    match joker_card.joker {
        Joker::RaisedFist => Box::new(RaisedFistEffect),
        Joker::Baron => Box::new(BaronEffect),
        Joker::Mime => Box::new(MimeEffect),
        _ => Box::new(ElseJokerEffect),
    }
}

pub fn on_held_joker(
    joker_card: &JokerCard,
    cards_in_hand: &[Card],
    chips: Chips,
    mult: Mult,
) -> (Chips, Mult) {
    let effect = get_on_held_joker_effect(joker_card);
    effect.apply(cards_in_hand, chips, mult)
}

// ----------------- Helper Functions ----------------- //

fn get_lowest_rank(cards: &[Card]) -> &Card {
    let mut lowest_rank = rank_numeric(&cards[0]);
    let mut lowest_rank_card = &cards[0];
    for card in cards {
        if rank_numeric(card) < lowest_rank {
            lowest_rank = rank_numeric(card);
            lowest_rank_card = card
        }
    }
    lowest_rank_card
}
