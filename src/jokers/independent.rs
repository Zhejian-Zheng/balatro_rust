use crate::util::{
    get_rank_count,
    is_flush,
    is_straight,
    // rank_numeric
};

/// src/jokers/independent.rs
/// This file is used for:
/// Implement jokers that effect the game independently
use ortalib::{Card, Chips, Enhancement, Joker, JokerCard, Mult, Rank, Round, Suit, SuitColor};

/// A trait defining how a Independent Jokers effect are applied during a round.
pub trait IdpJokerEffect {
    fn apply(
        &self,
        round: &Round,
        scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        jokers: &[JokerCard],
    ) -> (Chips, Mult);
}

pub struct JokerEffect;
pub struct JollyJokerEffect;
pub struct ZanyJokerEffect;
pub struct MadJokerEffect;
pub struct CrazyJokerEffect;
pub struct DrollJokerEffect;
pub struct SlyJokerEffect;
pub struct WilyJokerEffect;
pub struct CleverJokerEffect;
pub struct DeviousJokerEffect;
pub struct CraftyJokerEffect;
pub struct BlackboardEffect;
pub struct FlowerPotEffect;
pub struct ElseJokerEffect;

impl IdpJokerEffect for JokerEffect {
    fn apply(
        &self,
        _round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        (chips, mult + 4.0)
    }
}

impl IdpJokerEffect for JollyJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        if has_pair(cards_played) {
            (chips, mult + 8.0)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for ZanyJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        if has_three_of_a_kind(cards_played) {
            (chips, mult + 12.0)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for MadJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        if has_two_pair(cards_played) {
            (chips, mult + 10.0)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for CrazyJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;

        let mut has_fourfingers = false;
        let mut has_shortcut = false;
        for joker in jokers.iter() {
            if joker.joker == Joker::FourFingers {
                has_fourfingers = true
            };
            if joker.joker == Joker::Shortcut {
                has_shortcut = true
            };
        }

        if is_straight(cards_played, has_fourfingers, has_shortcut) {
            (chips, mult + 12.0)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for DrollJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        let mut has_fourfingers = false;
        let mut has_smeared = false;
        for joker in jokers.iter() {
            if joker.joker == Joker::FourFingers {
                has_fourfingers = true
            };
            if joker.joker == Joker::SmearedJoker {
                has_smeared = true
            };
        }

        if is_flush(cards_played, has_fourfingers, has_smeared) {
            (chips, mult + 10.0)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for SlyJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        if has_pair(cards_played) {
            (chips + 50.0, mult)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for WilyJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        if has_three_of_a_kind(cards_played) {
            (chips + 100.0, mult)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for CleverJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        if has_two_pair(cards_played) {
            (chips + 80.0, mult)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for DeviousJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        let mut has_fourfingers = false;
        let mut has_shortcut = false;
        for joker in jokers.iter() {
            if joker.joker == Joker::FourFingers {
                has_fourfingers = true
            };
            if joker.joker == Joker::Shortcut {
                has_shortcut = true
            };
        }

        if is_straight(cards_played, has_fourfingers, has_shortcut) {
            (chips + 100.0, mult)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for CraftyJokerEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_played = &round.cards_played;
        let mut has_fourfingers = false;
        let mut has_smeared = false;
        for joker in jokers.iter() {
            if joker.joker == Joker::FourFingers {
                has_fourfingers = true
            };
            if joker.joker == Joker::SmearedJoker {
                has_smeared = true
            };
        }

        if is_flush(cards_played, has_fourfingers, has_smeared) {
            (chips + 80.0, mult)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for BlackboardEffect {
    fn apply(
        &self,
        round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let cards_in_hand = &round.cards_held_in_hand;
        if check_cards_all_spades_or_clubs(cards_in_hand) {
            (chips, mult * 3.0)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for FlowerPotEffect {
    fn apply(
        &self,
        _round: &Round,
        scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        let mut has_smeared = false;
        for joker in jokers.iter() {
            if joker.joker == Joker::SmearedJoker {
                has_smeared = true
            };
        }

        if check_contains_all_suit(scoring_cards, has_smeared) {
            (chips, mult * 3.0)
        } else {
            (chips, mult)
        }
    }
}

impl IdpJokerEffect for ElseJokerEffect {
    fn apply(
        &self,
        _round: &Round,
        _scoring_cards: &[Card],
        chips: Chips,
        mult: Mult,
        _jokers: &[JokerCard],
    ) -> (Chips, Mult) {
        (chips, mult)
    }
}

pub fn get_idp_joker_effect(joker_card: &JokerCard) -> Box<dyn IdpJokerEffect> {
    match joker_card.joker {
        Joker::Joker => Box::new(JokerEffect),
        Joker::JollyJoker => Box::new(JollyJokerEffect),
        Joker::ZanyJoker => Box::new(ZanyJokerEffect),
        Joker::MadJoker => Box::new(MadJokerEffect),
        Joker::CrazyJoker => Box::new(CrazyJokerEffect),
        Joker::DrollJoker => Box::new(DrollJokerEffect),
        Joker::SlyJoker => Box::new(SlyJokerEffect),
        Joker::WilyJoker => Box::new(WilyJokerEffect),
        Joker::CleverJoker => Box::new(CleverJokerEffect),
        Joker::DeviousJoker => Box::new(DeviousJokerEffect),
        Joker::CraftyJoker => Box::new(CraftyJokerEffect),
        Joker::Blackboard => Box::new(BlackboardEffect),
        Joker::FlowerPot => Box::new(FlowerPotEffect),
        _ => Box::new(ElseJokerEffect),
    }
}

pub fn idp_joker(
    joker_card: &JokerCard,
    round: &Round,
    scoring_cards: &[Card],
    chips: Chips,
    mult: Mult,
    jokers: &[JokerCard],
) -> (Chips, Mult) {
    let effect = get_idp_joker_effect(joker_card);
    effect.apply(round, scoring_cards, chips, mult, jokers)
}

// ----------------- Helper Functions ----------------- //

fn has_pair(cards_played: &[Card]) -> bool {
    let rank_counts = get_rank_count(cards_played);
    for (&_rank, &count) in rank_counts.iter() {
        if count >= 2 {
            return true;
        }
    }
    false
}

fn has_three_of_a_kind(cards_played: &[Card]) -> bool {
    let rank_counts = get_rank_count(cards_played);
    for (&_rank, &count) in rank_counts.iter() {
        if count >= 3 {
            return true;
        }
    }
    false
}

fn has_two_pair(cards_played: &[Card]) -> bool {
    let rank_counts = get_rank_count(cards_played);
    let mut pairs: Vec<Rank> = Vec::new();
    for (&rank, &count) in rank_counts.iter() {
        if count >= 2 {
            pairs.push(rank);
        }
    }
    if pairs.len() >= 2 {
        return true;
    }
    false
}

fn check_cards_all_spades_or_clubs(cards_in_hand: &[Card]) -> bool {
    for card in cards_in_hand {
        if card.suit == Suit::Hearts || card.suit == Suit::Diamonds {
            return false;
        }
        if card.enhancement == Some(Enhancement::Wild) {
            continue;
        }
    }
    true
}

fn check_contains_all_suit(scoring_cards: &[Card], has_smeared: bool) -> bool {
    if scoring_cards.len() < 4 {
        return false;
    }
    let mut has_clubs = false;
    let mut has_diamonds = false;
    let mut has_hearts = false;
    let mut has_spades = false;
    let mut wild_num = 0;

    let mut num_red = 0;
    let mut num_black = 0;

    for card in scoring_cards {
        if card.enhancement == Some(Enhancement::Wild) {
            wild_num += 1;
            continue;
        }

        if has_smeared {
            match card.suit.color() {
                SuitColor::Red => num_red += 1,
                SuitColor::Black => num_black += 1,
            }
        } else {
            match card.suit {
                Suit::Diamonds => has_diamonds = true,
                Suit::Clubs => has_clubs = true,
                Suit::Hearts => has_hearts = true,
                Suit::Spades => has_spades = true,
            }
        }
    }

    if has_smeared {
        if num_red >= 2 {
            has_diamonds = true;
            has_hearts = true;
        } else if num_red == 1 {
            has_diamonds = true;
        }

        if num_black >= 2 {
            has_clubs = true;
            has_spades = true;
        } else if num_red == 1 {
            has_clubs = true;
        }
    }

    // get number of missing suit
    let mut missing = 0;
    if !has_diamonds {
        missing += 1;
    }
    if !has_clubs {
        missing += 1;
    }
    if !has_hearts {
        missing += 1;
    }
    if !has_spades {
        missing += 1;
    }

    missing <= wild_num
}
