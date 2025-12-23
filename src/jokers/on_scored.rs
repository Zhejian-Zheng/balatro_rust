/// src/jokers/on_scored.rs
/// This file is used for:
/// Implement the jokers that will effect the game by the on_held cards:
use ortalib::{Card, Chips, Enhancement, Joker, JokerCard, Mult, Rank, Suit};

/// A trait defining how a on scored Jokers effect are applied during a round.
pub trait OnScoredJokerEffect {
    fn apply(
        &self,
        cards_played: &[Card],
        cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        has_pareidolia: bool,
    ) -> (Chips, Mult);
}

pub struct GreedyJokerEffect;
pub struct LustyJokerEffect;
pub struct WrathfulJokerEffect;
pub struct GluttonousJokerEffect;
pub struct FibonacciEffect;
pub struct ScaryFaceEffect;
pub struct EvenStevenEffect;
pub struct OddToddEffect;
pub struct PhotographEffect;
pub struct SmileyFaceEffect;
pub struct SockAndBuskinEffect;
pub struct ElseJokerEffect;

impl OnScoredJokerEffect for GreedyJokerEffect {
    fn apply(
        &self,
        cards_played: &[Card],
        _cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let new_chips = chips;
        let mut new_mult = mult;
        for card in cards_played {
            if card.suit == Suit::Diamonds || card.enhancement == Some(Enhancement::Wild) {
                new_mult += 3.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for LustyJokerEffect {
    fn apply(
        &self,
        cards_played: &[Card],
        _cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let new_chips = chips;
        let mut new_mult = mult;
        for card in cards_played {
            if card.suit == Suit::Hearts || card.enhancement == Some(Enhancement::Wild) {
                new_mult += 3.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for WrathfulJokerEffect {
    fn apply(
        &self,
        cards_played: &[Card],
        _cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let new_chips = chips;
        let mut new_mult = mult;
        for card in cards_played {
            if card.suit == Suit::Spades || card.enhancement == Some(Enhancement::Wild) {
                new_mult += 3.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for GluttonousJokerEffect {
    fn apply(
        &self,
        cards_played: &[Card],
        _cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let new_chips = chips;
        let mut new_mult = mult;
        for card in cards_played {
            if card.suit == Suit::Clubs || card.enhancement == Some(Enhancement::Wild) {
                new_mult += 3.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for FibonacciEffect {
    fn apply(
        &self,
        _cards_played: &[Card],
        cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let new_chips = chips;
        let mut new_mult = mult;
        for card in cards_scored {
            if card.rank == Rank::Ace
                || card.rank == Rank::Two
                || card.rank == Rank::Three
                || card.rank == Rank::Five
                || card.rank == Rank::Eight
            {
                new_mult += 8.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for ScaryFaceEffect {
    fn apply(
        &self,
        _cards_played: &[Card],
        cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let mut new_chips = chips;
        let new_mult = mult;
        for card in cards_scored {
            if has_pareidolia || card.rank.is_face() {
                new_chips += 30.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for EvenStevenEffect {
    fn apply(
        &self,
        _cards_played: &[Card],
        cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let new_chips = chips;
        let mut new_mult = mult;
        for card in cards_scored {
            if card.rank == Rank::Two
                || card.rank == Rank::Four
                || card.rank == Rank::Six
                || card.rank == Rank::Eight
                || card.rank == Rank::Ten
            {
                new_mult += 4.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for OddToddEffect {
    fn apply(
        &self,
        _cards_played: &[Card],
        cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let mut new_chips = chips;
        let new_mult = mult;
        for card in cards_scored {
            if card.rank == Rank::Ace
                || card.rank == Rank::Three
                || card.rank == Rank::Five
                || card.rank == Rank::Seven
                || card.rank == Rank::Nine
            {
                new_chips += 31.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for PhotographEffect {
    fn apply(
        &self,
        _cards_played: &[Card],
        cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let new_chips = chips;
        let mut new_mult = mult;
        for card in cards_scored {
            if has_pareidolia || card.rank.is_face() {
                new_mult *= 2.0;
                break;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for SmileyFaceEffect {
    fn apply(
        &self,
        _cards_played: &[Card],
        cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        has_pareidolia: bool,
    ) -> (Chips, Mult) {
        let new_chips = chips;
        let mut new_mult = mult;
        for card in cards_scored {
            if has_pareidolia || card.rank.is_face() {
                new_mult += 5.0;
            }
        }
        (new_chips, new_mult)
    }
}

impl OnScoredJokerEffect for SockAndBuskinEffect {
    fn apply(
        &self,
        _cards_played: &[Card],
        _cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        (chips, mult)
    }
}

impl OnScoredJokerEffect for ElseJokerEffect {
    fn apply(
        &self,
        _cards_played: &[Card],
        _cards_scored: &[Card],
        chips: Chips,
        mult: Mult,
        _has_pareidolia: bool,
    ) -> (Chips, Mult) {
        (chips, mult)
    }
}

pub fn get_on_scored_joker_effect(joker_card: &JokerCard) -> Box<dyn OnScoredJokerEffect> {
    match joker_card.joker {
        Joker::GreedyJoker => Box::new(GreedyJokerEffect),
        Joker::LustyJoker => Box::new(LustyJokerEffect),
        Joker::WrathfulJoker => Box::new(WrathfulJokerEffect),
        Joker::GluttonousJoker => Box::new(GluttonousJokerEffect),
        Joker::Fibonacci => Box::new(FibonacciEffect),
        Joker::ScaryFace => Box::new(ScaryFaceEffect),
        Joker::EvenSteven => Box::new(EvenStevenEffect),
        Joker::OddTodd => Box::new(OddToddEffect),
        Joker::Photograph => Box::new(PhotographEffect),
        Joker::SmileyFace => Box::new(SmileyFaceEffect),
        Joker::SockAndBuskin => Box::new(SockAndBuskinEffect),
        _ => Box::new(ElseJokerEffect),
    }
}

pub fn on_scored_joker(
    joker_card: &JokerCard,
    cards_played: &[Card],
    cards_scored: &[Card],
    chips: Chips,
    mult: Mult,
    has_pareidolia: bool,
) -> (Chips, Mult) {
    let effect = get_on_scored_joker_effect(joker_card);
    effect.apply(cards_played, cards_scored, chips, mult, has_pareidolia)
}
