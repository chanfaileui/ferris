mod game;

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
    println!("main {:?}", &opts);
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round, opts.explain); // pass in explain flag as well

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

fn score(round: Round, explain: bool) -> (Chips, Mult) {
    let mut game = game::GameState::new(round, explain);
    let result = game.score();

    if explain {
        for step in game.get_explanation() {
            println!("{}", step);
        }
    }

    result
}
