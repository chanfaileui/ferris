use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use ortalib::{Card, Edition, Enhancement, Joker, JokerCard, Rank, Suit};
use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
struct Round {
    cards_played: Vec<String>,
    cards_held_in_hand: Vec<String>,
    jokers: Vec<String>,
}

/// Generates a round with random cards and jokers
fn generate_random_round(rng: &mut impl Rng) -> Round {
    // Determine number of components
    let num_cards_played = rng.random_range(1..=5);
    let num_cards_in_hand = rng.random_range(0..=5);
    let num_jokers = rng.random_range(0..=5);

    // Generate cards played
    let mut cards_played = Vec::new();
    for _ in 0..num_cards_played {
        let rank = match rng.random_range(0..13) {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            _ => Rank::Ace,
        };

        let suit = match rng.random_range(0..4) {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Clubs,
            _ => Suit::Diamonds,
        };

        let mut card = Card::new(rank, suit, None, None).to_string();

        // Add enhancement with 30% probability
        if rng.random_bool(0.3) {
            let enhancement = match rng.random_range(0..5) {
                0 => Enhancement::Bonus,
                1 => Enhancement::Mult,
                2 => Enhancement::Wild,
                3 => Enhancement::Glass,
                _ => Enhancement::Steel,
            };
            card = format!("{} {}", card, enhancement);
        }

        // Add edition with 30% probability
        if rng.random_bool(0.3) {
            let edition = match rng.random_range(0..3) {
                0 => Edition::Foil,
                1 => Edition::Holographic,
                _ => Edition::Polychrome,
            };
            card = format!("{} {}", card, edition);
        }

        cards_played.push(card);
    }

    // Generate cards held in hand
    let mut cards_held_in_hand = Vec::new();
    for _ in 0..num_cards_in_hand {
        let rank = match rng.random_range(0..13) {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            _ => Rank::Ace,
        };

        let suit = match rng.random_range(0..4) {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Clubs,
            _ => Suit::Diamonds,
        };

        let mut card = Card::new(rank, suit, None, None).to_string();

        // Add enhancement with 30% probability
        if rng.random_bool(0.3) {
            let enhancement = match rng.random_range(0..5) {
                0 => Enhancement::Bonus,
                1 => Enhancement::Mult,
                2 => Enhancement::Wild,
                3 => Enhancement::Glass,
                _ => Enhancement::Steel,
            };
            card = format!("{} {}", card, enhancement);
        }

        // Add edition with 30% probability
        if rng.random_bool(0.3) {
            let edition = match rng.random_range(0..3) {
                0 => Edition::Foil,
                1 => Edition::Holographic,
                _ => Edition::Polychrome,
            };
            card = format!("{} {}", card, edition);
        }

        cards_held_in_hand.push(card);
    }

    // Generate jokers
    let mut jokers = Vec::new();
    for _ in 0..num_jokers {
        let joker_type = rng.random_range(0..34);
        let joker = match joker_type {
            0 => Joker::Joker,
            1 => Joker::JollyJoker,
            2 => Joker::ZanyJoker,
            3 => Joker::MadJoker,
            4 => Joker::CrazyJoker,
            5 => Joker::DrollJoker,
            6 => Joker::SlyJoker,
            7 => Joker::WilyJoker,
            8 => Joker::CleverJoker,
            9 => Joker::DeviousJoker,
            10 => Joker::CraftyJoker,
            11 => Joker::AbstractJoker,
            12 => Joker::RaisedFist,
            13 => Joker::Blackboard,
            14 => Joker::Baron,
            15 => Joker::GreedyJoker,
            16 => Joker::LustyJoker,
            17 => Joker::WrathfulJoker,
            18 => Joker::GluttonousJoker,
            19 => Joker::Fibonacci,
            20 => Joker::ScaryFace,
            21 => Joker::EvenSteven,
            22 => Joker::OddTodd,
            23 => Joker::Photograph,
            24 => Joker::SmileyFace,
            25 => Joker::FlowerPot,
            26 => Joker::FourFingers,
            27 => Joker::Shortcut,
            28 => Joker::Mime,
            29 => Joker::Pareidolia,
            30 => Joker::Splash,
            31 => Joker::SockAndBuskin,
            32 => Joker::SmearedJoker,
            _ => Joker::Blueprint,
        };

        let mut joker_card = JokerCard::new(joker, None).to_string();

        // Add edition with 30% probability
        if rng.random_bool(0.3) {
            let edition = match rng.random_range(0..3) {
                0 => Edition::Foil,
                1 => Edition::Holographic,
                _ => Edition::Polychrome,
            };
            joker_card = format!("{} {}", joker_card, edition);
        }

        jokers.push(joker_card);
    }

    Round {
        cards_played,
        cards_held_in_hand,
        jokers,
    }
}

fn run_test(test_number: usize, test_dir: &Path) -> io::Result<bool> {
    let test_name = format!("test_{:03}", test_number);
    let round_path = test_dir.join(format!("{}.yml", test_name));

    // Create a random round
    let mut rng = rand::rng();
    let round = generate_random_round(&mut rng);

    // Save to YAML
    let yaml_str = serde_yaml::to_string(&round).unwrap();
    fs::write(&round_path, yaml_str)?;

    // Run reference solution
    let ref_output = Command::new("6991")
        .args(["ortalab", &round_path.to_string_lossy()])
        .output()?;

    let ref_result = if ref_output.status.success() {
        String::from_utf8_lossy(&ref_output.stdout)
            .trim()
            .to_string()
    } else {
        return Ok(false); // Reference solution failed
    };

    // Run your solution
    let your_output = Command::new("cargo")
        .args(["run", "--", &round_path.to_string_lossy()])
        .output()?;

    let your_result = if your_output.status.success() {
        String::from_utf8_lossy(&your_output.stdout)
            .trim()
            .to_string()
    } else {
        return Ok(false); // Your solution failed
    };

    Ok(ref_result == your_result)
}

fn main() -> io::Result<()> {
    println!("Ortalab Fuzzer");

    // Create test directory
    let test_dir = Path::new("fuzzer_tests");
    if !test_dir.exists() {
        fs::create_dir(test_dir)?;
    }

    // Build your solution
    println!("Building your solution...");
    let build_output = Command::new("cargo").arg("build").output()?;
    if !build_output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Build failed"));
    }

    // Run tests
    let num_tests = 50;
    let mut passed = 0;
    let mut failed = Vec::new();

    println!("Running {} tests...", num_tests);

    let mut log_file = File::create(test_dir.join("test_log.txt"))?;

    for i in 0..num_tests {
        print!("Test {}/{}...\r", i + 1, num_tests);
        io::stdout().flush()?;

        match run_test(i, test_dir) {
            Ok(true) => {
                passed += 1;
                writeln!(log_file, "PASS: test_{:03}", i)?;
            }
            Ok(false) => {
                failed.push(i);
                writeln!(log_file, "FAIL: test_{:03}", i)?;
            }
            Err(e) => {
                writeln!(log_file, "ERROR: test_{:03} - {}", i, e)?;
            }
        }
    }

    println!("\nTesting complete!");
    println!(
        "Passed: {}/{} tests ({:.1}%)",
        passed,
        num_tests,
        (passed as f64 / num_tests as f64) * 100.0
    );

    if !failed.is_empty() {
        println!("\nFailed tests: {:?}", failed);
        println!("Check fuzzer_tests/test_log.txt for details");
    } else {
        println!("All tests passed!");
    }

    Ok(())
}
