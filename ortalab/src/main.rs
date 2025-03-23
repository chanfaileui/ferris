use std::{
    collections::HashMap, error::Error, fs::File, io::{stdin, Read}, path::{Path, PathBuf}
};

use clap::Parser;
use ortalib::{Chips, Mult, Round};
use itertools::Itertools;

#[derive(Parser, Debug)]
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
    println!("Current {:?}", &opts);
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

fn group_rank(round: &Round) -> HashMap<ortalib::Rank, usize> {
    let rank_counts = round.cards_played.iter().map(|card| card.rank).counts();
    rank_counts
}

fn group_suit(round: &Round) -> HashMap<ortalib::Suit, usize> {
    let suit_counts = round.cards_played.iter().map(|card| card.suit).counts();
    suit_counts
}

fn score(round: Round) -> (Chips, Mult) {
    println!("ROUNDDDD {:?}", &round);
    println!("cards_played {:?}", &round.cards_played);
    println!("cards held in hand {:?}", &round.cards_held_in_hand);
    println!("jokers! {:?}", &round.jokers);
    println!("{:?}", group_rank(&round));
    println!("{:?}", group_suit(&round));

    // 
    todo!()
    // best one is 
}
