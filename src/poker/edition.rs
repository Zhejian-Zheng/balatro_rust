//! src/poker/edition.rs
//! This file is used for:
//! Achieve the card edition functionality, which is for edit the card and make it increase the score

use ortalib::{Card, Chips, Edition, JokerCard, Mult};

// A trait for applying bonus effects on chips and multiplier based on card editions.
///
/// Types implementing this trait, such as `Card` and `JokerCard`, define how a bonus is applied
/// according to their edition. For example, a `Foil` edition might add a fixed amount of chips,
/// while a `Holographic` edition could increase the multiplier.
///
/// # Method
///
/// - `apply`: Consumes the current chips and multiplier values and returns new values after applying
///   the bonus effect.
pub trait Apply {
    fn apply(&self, chips: Chips, mult: Mult) -> (Chips, Mult);
}

impl Apply for Card {
    fn apply(&self, chips: Chips, mult: Mult) -> (Chips, Mult) {
        match self.edition {
            Some(Edition::Foil) => (chips + 50.0, mult),
            Some(Edition::Holographic) => (chips, mult + 10.0),
            Some(Edition::Polychrome) => (chips, mult * 1.5),
            _ => (chips, mult),
        }
    }
}

impl Apply for JokerCard {
    fn apply(&self, chips: Chips, mult: Mult) -> (Chips, Mult) {
        match self.edition {
            Some(Edition::Foil) => (chips + 50.0, mult),
            Some(Edition::Holographic) => (chips, mult + 10.0),
            Some(Edition::Polychrome) => (chips, mult * 1.5),
            _ => (chips, mult),
        }
    }
}

/// Applies the bonus effect for any type that implements the [`Apply`] trait.
///
/// # Parameters
///
/// - `card`: A reference to a value implementing the [`Apply`] trait.
/// - `chips`: The current chips count.
/// - `mult`: The current multiplier.
///
/// # Returns
///
/// A tuple `(Chips, Mult)` representing the updated chips and multiplier values after applying the effect.
///
/// A tuple `(Chips, Mult)` representing the updated chips and multiplier values after applying the effect.
///
/// # Examples
///
/// ```
/// some crates import ...
/// let card = Card {
///     edition: Some(Edition::Foil),
/// };
///
/// let chips = Chips::new(100.0);
/// let mult = Mult::new(1.0);
///
/// let (new_chips, new_mult) = apply(&card, chips, mult);
///
/// // For a Foil edition, 50.0 chips are added and multiplier remains unchanged
/// assert_eq!(new_chips, chips + 50.0);
/// assert_eq!(new_mult, mult);
/// ```
pub fn apply<T: Apply>(card: &T, chips: Chips, mult: Mult) -> (Chips, Mult) {
    card.apply(chips, mult)
}
