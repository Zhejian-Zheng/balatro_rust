use std::{
    error::Error,
    fs::File,
    io::{stdin, Read},
    path::{Path, PathBuf},
};

use clap::Parser;
use ortalib::{Chips, Mult, Round};

mod jokers;
mod poker;
mod util;

#[derive(Parser)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round);

    println!("{}", (chips * mult).floor());
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

mod scoring;
use scoring::{evaluate_base_hand, idp_joker_edition, score_held_cards, score_played_cards};

fn score(round: Round) -> (Chips, Mult) {
    // handle the blueprint and other effect jokers.
    // let jokers: Vec<JokerCard> = handle_jokers(&round.jokers);

    // 1. Determine the base hand
    // This will return the base chips, mult and the scoring cards
    let (_best_hand, base_chips, base_mult, scoring_cards) =
        evaluate_base_hand(&round.cards_played, &round.jokers);

    // 2. Score each played card
    let (chips_after_cards, mult_after_cards) = score_played_cards(
        &round.cards_played,
        &scoring_cards,
        base_chips,
        base_mult,
        &round.jokers,
    );

    // 3. Score held cards (e.g., apply Steel card enhancements)
    let (chips_after_held, mult_after_held) = score_held_cards(
        &round.cards_held_in_hand,
        chips_after_cards,
        mult_after_cards,
        &round.jokers,
    );

    // 4. Apply independent joker editions and other independent effects
    let (final_chips, final_mult) =
        idp_joker_edition(&round, &scoring_cards, chips_after_held, mult_after_held);

    (final_chips, final_mult)
}
