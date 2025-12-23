//! This library is use for achieve the scoring system for the ortalab assignment
//! Language: Rust

use crate::jokers::independent::idp_joker;
use crate::jokers::on_held::on_held_joker;
use crate::jokers::on_scored::on_scored_joker;
use crate::poker::edition;
use crate::poker::enhancement;
use crate::poker::hand_determine;
use ortalib::Enhancement;
use ortalib::{Card, Chips, Edition, Joker, JokerCard, Mult, PokerHand, Round};

/// Determine the poker hand formed by the cards played. This is done by finding the best poker
/// hand that can be made from the cards played. This may need to consider effect jokers such as
/// Four Fingers and Shortcut, and any played cards that are wild.
///
/// # Parameters
/// * `cards` - an vector of Card which represent the cards we want to check for the poker hand
/// * `jokers` - an vector of JokerCard which will effect the hand determintation.
///
/// # Returns
///
/// Returns a tuple `(Best_hand, Chips, Mult, Scoring_cards)` representing
// the best hand we found, the current chips and mult and the card which will give the scoring
pub fn evaluate_base_hand(
    cards: &[Card],
    jokers: &[JokerCard],
) -> (PokerHand, Chips, Mult, Vec<Card>) {
    let mut has_fourfinger = false;
    let mut has_shortcut = false;
    let mut has_smeared = false;

    for joker in jokers.iter() {
        if joker.joker == Joker::FourFingers {
            has_fourfinger = true
        };
        if joker.joker == Joker::Shortcut {
            has_shortcut = true
        };
        if joker.joker == Joker::SmearedJoker {
            has_smeared = true
        };
    }

    let (best_hand, scoring_cards) =
        hand_determine::determine_best_hand(cards, has_fourfinger, has_shortcut, has_smeared);
    let (base_chips, base_mult) = best_hand.hand_value();
    // println!("base_chips: {}, base_mult: {}", base_chips, base_mult);
    // println!("scoring card: {:?}", scoring_cards);
    (best_hand, base_chips, base_mult, scoring_cards)
}

/// This function is use for score each scoring card
/// Important: If the Splash joker is active, we need to score all the played cards
///
/// # Parameters
///
/// * `played_cards` - an vector of Card which represent the cards we played
/// * `scoring_cards` - an vector of Card which represent the cards that form up the poker hands
/// * `jokers` - an vector of JokerCard which will provides bonuses when calculating score cards.
/// * `chips` - current chips
/// * `mult` - current mult
///
/// # Returns
///
/// Returns a tuple `(Chips, Mult)` representing the updated chips and multiplier after calcuating
/// all scoring cards.
pub fn score_played_cards(
    played_cards: &[Card],
    scoring_cards: &[Card],
    chips: Chips,
    mult: Mult,
    jokers: &[JokerCard],
) -> (Chips, Mult) {
    let normalise_jokers = finalise_blue_print_joker(jokers);

    let (mut final_chips, mut final_mult) = scoring_face_card_retrigger_help(
        played_cards,
        scoring_cards,
        chips,
        mult,
        &normalise_jokers,
        false,
    );

    for joker in normalise_jokers.iter() {
        if joker.joker == Joker::SockAndBuskin {
            let (new_chips, new_mult) = scoring_face_card_retrigger_help(
                played_cards,
                scoring_cards,
                final_chips,
                final_mult,
                &normalise_jokers,
                true,
            );
            final_chips = new_chips;
            final_mult = new_mult;
        }
    }
    (final_chips, final_mult)
}

/// This function is use for check if the held card can provide any bonuses to the mark
///
/// # Parameters
///
/// * `held_cards` - an vector of Card which represent the cards we played
/// * `jokers` - an vector of JokerCard which will provides bonuses by check if the held cards meet the joker's condition .
/// * `chips` - current chips
/// * `mult` - current mult
///
/// # Returns
///
/// Returns a tuple `(Chips, Mult)` representing the updated chips and multiplier after check
/// if the held cards can bring some bonuses to the chips and mult
pub fn score_held_cards(
    held_cards: &[Card],
    chips: Chips,
    mult: Mult,
    jokers: &[JokerCard],
) -> (Chips, Mult) {
    let normalise_jokers = finalise_blue_print_joker(jokers);
    let (mut final_chips, mut final_mult) =
        held_retrigger_help(held_cards, chips, mult, &normalise_jokers);

    for joker in normalise_jokers.iter() {
        if joker.joker == Joker::Mime {
            let (new_chips, new_mult) =
                held_retrigger_help(held_cards, final_chips, final_mult, &normalise_jokers);
            final_chips = new_chips;
            final_mult = new_mult;
        }
    }
    (final_chips, final_mult)
}

/// Applies independent jokers and their editions effects to update chips and multiplier based on the current round.
///
/// This function processes the joker effects for a given round by first finalising the blueprint
/// jokers using [`finalise_blue_print_joker`]. Then, for each joker, it applies edition-specific
/// modifications and joker-specific effects:
///
/// # Parameters
///
/// - `round`: A reference to the current `Round` containing the game state and the list of jokers.
/// - `scoring_cards`: A slice of `Card` used for scoring in the current round.
/// - `chips`: The current chips count.
/// - `mult`: The current multiplier.
///
/// # Returns
///
/// Returns a tuple `(Chips, Mult)` representing the updated chips and multiplier after applying
/// all joker edition effects.
pub fn idp_joker_edition(
    round: &Round,
    scoring_cards: &[Card],
    mut chips: Chips,
    mut mult: Mult,
) -> (Chips, Mult) {
    let jokers = finalise_blue_print_joker(&round.jokers);

    // Edition (Foil or Holographic):
    for joker in jokers.iter() {
        if let Some(Edition::Foil | Edition::Holographic) = joker.edition {
            let (new_chips, new_mult) = edition::apply(joker, chips, mult);
            chips = new_chips;
            mult = new_mult;
        }

        if joker.joker != Joker::AbstractJoker {
            let (new_chips, new_mult) =
                idp_joker(joker, round, scoring_cards, chips, mult, &jokers);
            chips = new_chips;
            mult = new_mult;
        } else {
            mult += 3.0 * (jokers.len() as f64);
        }

        if let Some(Edition::Polychrome) = joker.edition {
            let (new_chips, new_mult) = edition::apply(joker, chips, mult);
            chips = new_chips;
            mult = new_mult;
        }
    }

    (chips, mult)
}

// ---------------- Help function ---------------- //

fn held_retrigger_help(
    held_cards: &[Card],
    chips: Chips,
    mult: Mult,
    jokers: &[JokerCard],
) -> (Chips, Mult) {
    let mut new_chips = chips;
    let mut new_mult = mult;
    for card in held_cards.iter() {
        if card.enhancement == Some(Enhancement::Steel) {
            new_mult *= 1.5;
        }
    }

    for joker in jokers.iter() {
        if joker.joker != Joker::Mime {
            let (joker_chips, joker_mult) = on_held_joker(joker, held_cards, new_chips, new_mult);
            new_chips = joker_chips;
            new_mult = joker_mult;
        }
    }
    (new_chips, new_mult)
}

fn is_face_card(card: &Card, has_pareidolia: bool) -> bool {
    if has_pareidolia {
        true
    } else {
        card.rank.is_face()
    }
}

fn scoring_face_card_retrigger_help(
    played_cards: &[Card],
    scoring_cards: &[Card],
    mut chips: Chips,
    mut mult: Mult,
    jokers: &[JokerCard],
    only_face_cards: bool,
) -> (Chips, Mult) {
    let mut has_splash = false;
    for joker in jokers.iter() {
        if joker.joker == Joker::Splash {
            has_splash = true;
            break;
        }
    }

    let mut has_pareidolia = false;
    for joker in jokers.iter() {
        if joker.joker == Joker::Pareidolia {
            has_pareidolia = true;
            break;
        }
    }

    let cards_to_score = if has_splash {
        played_cards
    } else {
        scoring_cards
    };

    for card in cards_to_score.iter() {
        if only_face_cards && !is_face_card(card, has_pareidolia) {
            continue;
        }

        chips += card.rank.rank_value();

        let (en_chips, en_mult) = enhancement::apply(card, chips, mult);
        chips = en_chips;
        mult = en_mult;

        // println!("card: {}, en_chips: {}, en_mult: {}", card, chips, mult);

        let (ed_chips, ed_mult) = edition::apply(card, chips, mult);
        chips = ed_chips;
        mult = ed_mult;
    }

    for joker in jokers.iter() {
        if joker.joker != Joker::SockAndBuskin {
            let (os_chips, os_mult) = on_scored_joker(
                joker,
                played_cards,
                cards_to_score,
                chips,
                mult,
                has_pareidolia,
            );
            chips = os_chips;
            mult = os_mult;
        }
    }
    (chips, mult)
}

fn finalise_blue_print_joker(jokers: &[JokerCard]) -> Vec<JokerCard> {
    let mut new_joker_list = jokers.to_vec();

    if new_joker_list.len() > 1 {
        let mut i = new_joker_list.len() - 1;
        while i > 0 {
            if new_joker_list[i - 1].joker == Joker::Blueprint {
                let right_joker = new_joker_list[i].joker;
                if right_joker != Joker::SmearedJoker
                    && right_joker != Joker::Splash
                    && right_joker != Joker::Pareidolia
                    && right_joker != Joker::Shortcut
                    && right_joker != Joker::FourFingers
                {
                    let new_joker = JokerCard::new(right_joker, new_joker_list[i - 1].edition);
                    new_joker_list[i - 1] = new_joker;
                }
            }
            i -= 1
        }
    }

    new_joker_list
}
