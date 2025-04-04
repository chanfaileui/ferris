//! # Balatro Poker Hand Scorer
//!
//! This application scores poker hands according to the rules of Balatro,
//! including the effects of jokers and card enhancements.

//! ## Usage
//! The application takes a YAML file describing a round (cards, jokers, etc.)
//! and outputs the final score after applying all rules and effects.
//!
//! ```
//! 6991 cargo run input.yaml [--explain]
//! ```
//!
//! The `--explain` flag enables detailed explanation of the scoring process.

mod debug;
mod errors;
mod game;
mod jokers;
mod modifiers;
mod poker;

use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::{Path, PathBuf},
};

use clap::Parser;
use ortalib::{Chips, Mult, Round};

#[derive(Parser, Debug)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (_chips, _mult) = score(round, opts.explain);
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

fn score(round: Round, explain: bool) -> (Chips, Mult) {
    let mut game = game::GameState::new(round, explain);
    let result = game.score();

    match result {
        Ok((chips, mult)) => {
            println!("{}", (chips * mult).floor());
            (chips, mult)
        }
        Err(e) => {
            eprintln!("Game error: {}", e);
            (0.0, 0.0)
        }
    }
}
