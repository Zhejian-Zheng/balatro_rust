//! src/poker/enhancement.rs
//! This file is used for:
//! Achieve the card enhancement functionality, which is for applying enhancement effects to a card
//!
use ortalib::{Card, Chips, Enhancement, Mult};

/// Applies an enhancement to the given chips and multiplier.
pub fn apply(card: &Card, chips: Chips, mult: Mult) -> (Chips, Mult) {
    match card.enhancement {
        Some(Enhancement::Bonus) => (chips + 30.0, mult),
        Some(Enhancement::Mult) => (chips, mult + 4.0),
        Some(Enhancement::Glass) => (chips, mult * 2.0),
        Some(Enhancement::Wild) => (chips, mult), // should be implement in the file of hand_determine
        Some(Enhancement::Steel) => (chips, mult), // should be implement in the score function
        _ => (chips, mult),
    }
}
