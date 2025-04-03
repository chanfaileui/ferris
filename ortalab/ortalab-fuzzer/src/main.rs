use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};
use std::time::Instant;

use ortalib::{Card, Edition, Enhancement, Joker, JokerCard, Rank, Suit};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use serde::Serialize;
use yaml_rust::YamlEmitter;

#[derive(Serialize)]
struct Round {
    cards_played: Vec<String>,
    cards_held_in_hand: Vec<String>,
    jokers: Vec<String>,
}

/// Generates a random card with optional enhancement and edition
fn random_card(rng: &mut impl Rng, allow_enhancement: bool, allow_edition: bool) -> String {
    let rank = match rng.gen_range(0..13) {
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

    let suit = match rng.gen_range(0..4) {
        0 => Suit::Spades,
        1 => Suit::Hearts,
        2 => Suit::Clubs,
        _ => Suit::Diamonds,
    };

    let card = Card::new(rank, suit, None, None);
    let mut result = card.to_string();

    if allow_enhancement && rng.gen_bool(0.3) {
        let enhancement = match rng.gen_range(0..5) {
            0 => Enhancement::Bonus,
            1 => Enhancement::Mult,
            2 => Enhancement::Wild,
            3 => Enhancement::Glass,
            _ => Enhancement::Steel,
        };
        result = format!("{} {}", result, enhancement);
    }

    if allow_edition && rng.gen_bool(0.3) {
        let edition = match rng.gen_range(0..3) {
            0 => Edition::Foil,
            1 => Edition::Holographic,
            _ => Edition::Polychrome,
        };
        result = format!("{} {}", result, edition);
    }

    result
}

/// Generates a random joker card with optional edition
fn random_joker(rng: &mut impl Rng, allow_edition: bool) -> String {
    let joker = match rng.gen_range(0..34) {
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

    let joker_card = JokerCard::new(joker, None);
    let mut result = joker_card.to_string();

    if allow_edition && rng.gen_bool(0.3) {
        let edition = match rng.gen_range(0..3) {
            0 => Edition::Foil,
            1 => Edition::Holographic,
            _ => Edition::Polychrome,
        };
        result = format!("{} {}", result, edition);
    }

    result
}

/// Generate a random round with specified parameters
fn generate_random_round(
    rng: &mut impl Rng,
    min_cards_played: usize,
    max_cards_played: usize,
    max_cards_in_hand: usize,
    max_jokers: usize,
    allow_enhancements: bool,
    allow_editions: bool,
) -> Round {
    let num_cards_played = rng.gen_range(min_cards_played..=max_cards_played);
    let num_cards_in_hand = rng.gen_range(0..=max_cards_in_hand);
    let num_jokers = rng.gen_range(0..=max_jokers);

    let mut cards_played = Vec::with_capacity(num_cards_played);
    for _ in 0..num_cards_played {
        cards_played.push(random_card(rng, allow_enhancements, allow_editions));
    }

    let mut cards_held_in_hand = Vec::with_capacity(num_cards_in_hand);
    for _ in 0..num_cards_in_hand {
        cards_held_in_hand.push(random_card(rng, allow_enhancements, allow_editions));
    }

    let mut jokers = Vec::with_capacity(num_jokers);
    for _ in 0..num_jokers {
        jokers.push(random_joker(rng, allow_editions));
    }

    Round {
        cards_played,
        cards_held_in_hand,
        jokers,
    }
}

/// Generate rounds targeting specific poker hands or joker combinations
fn generate_targeted_round(rng: &mut impl Rng) -> Round {
    let mut cards_played = Vec::new();
    let mut cards_held_in_hand = Vec::new();
    let mut jokers = Vec::new();

    // Define the target scenario (random choice among several interesting cases)
    let scenario = rng.gen_range(0..10);

    match scenario {
        0 => {
            // Scenario: Four Fingers + Shortcut + Wild cards
            jokers.push(JokerCard::new(Joker::FourFingers, None).to_string());
            jokers.push(JokerCard::new(Joker::Shortcut, None).to_string());

            // Add some cards that could form interesting combinations with these jokers
            let ranks = [Rank::Two, Rank::Four, Rank::Six, Rank::Eight, Rank::Queen];
            let suits = [
                Suit::Hearts,
                Suit::Hearts,
                Suit::Spades,
                Suit::Hearts,
                Suit::Hearts,
            ];

            for i in 0..5 {
                let mut card_str = Card::new(ranks[i], suits[i], None, None).to_string();
                if i == 2 && rng.gen_bool(0.8) {
                    card_str = format!("{} Wild", card_str);
                }
                cards_played.push(card_str);
            }
        }
        1 => {
            // Scenario: Test Ace handling in straights
            // Create a hand that's almost a straight with Ace
            let ranks = [Rank::Queen, Rank::King, Rank::Ace, Rank::Two, Rank::Three];
            let suits = [
                Suit::Spades,
                Suit::Hearts,
                Suit::Diamonds,
                Suit::Clubs,
                Suit::Hearts,
            ];

            for i in 0..5 {
                cards_played.push(Card::new(ranks[i], suits[i], None, None).to_string());
            }
        }
        2 => {
            // Scenario: Blueprint chain
            jokers.push(JokerCard::new(Joker::Blueprint, None).to_string());
            jokers.push(JokerCard::new(Joker::Blueprint, None).to_string());
            jokers.push(JokerCard::new(Joker::Joker, None).to_string());

            // Add some basic cards
            cards_played.push(Card::new(Rank::Ten, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Ten, Suit::Spades, None, None).to_string());
            cards_played.push(Card::new(Rank::Jack, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Queen, Suit::Diamonds, None, None).to_string());
            cards_played.push(Card::new(Rank::King, Suit::Clubs, None, None).to_string());
        }
        3 => {
            // Scenario: Retrigger chain
            jokers.push(JokerCard::new(Joker::SockAndBuskin, None).to_string());
            jokers.push(JokerCard::new(Joker::ScaryFace, None).to_string());

            // Add face cards to trigger the chain
            cards_played.push(Card::new(Rank::Jack, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Queen, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::King, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Jack, Suit::Spades, None, None).to_string());
            cards_played.push(Card::new(Rank::Queen, Suit::Diamonds, None, None).to_string());
        }
        4 => {
            // Scenario: Pareidolia + Sock and Buskin
            jokers.push(JokerCard::new(Joker::Pareidolia, None).to_string());
            jokers.push(JokerCard::new(Joker::SockAndBuskin, None).to_string());
            jokers.push(JokerCard::new(Joker::SmileyFace, None).to_string());

            // Add non-face cards
            cards_played.push(Card::new(Rank::Two, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Three, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Four, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Five, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Six, Suit::Hearts, None, None).to_string());
        }
        5 => {
            // Scenario: Smeared Joker + Flower Pot
            jokers.push(JokerCard::new(Joker::SmearedJoker, None).to_string());
            jokers.push(JokerCard::new(Joker::FlowerPot, None).to_string());

            // Add cards with mixed colors but fewer than all 4 suits
            cards_played.push(Card::new(Rank::Ten, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Jack, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Queen, Suit::Diamonds, None, None).to_string());
            cards_played.push(Card::new(Rank::King, Suit::Spades, None, None).to_string());
            cards_played.push(Card::new(Rank::Ace, Suit::Spades, None, None).to_string());
        }
        6 => {
            // Scenario: Raised Fist with multiple lowest cards in hand
            jokers.push(JokerCard::new(Joker::RaisedFist, None).to_string());

            // Add some cards played
            cards_played.push(Card::new(Rank::Ten, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Ten, Suit::Spades, None, None).to_string());
            cards_played.push(Card::new(Rank::Ten, Suit::Clubs, None, None).to_string());
            cards_played.push(Card::new(Rank::King, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::King, Suit::Diamonds, None, None).to_string());

            // Add multiple lowest ranked cards in hand
            cards_held_in_hand.push(Card::new(Rank::Two, Suit::Hearts, None, None).to_string());
            cards_held_in_hand.push(Card::new(Rank::Two, Suit::Spades, None, None).to_string());
            cards_held_in_hand.push(Card::new(Rank::Three, Suit::Clubs, None, None).to_string());
        }
        7 => {
            // Scenario: Four Fingers + Splash
            jokers.push(JokerCard::new(Joker::FourFingers, None).to_string());
            jokers.push(JokerCard::new(Joker::Splash, None).to_string());

            // Add cards that form a 4-card flush and some high-value extras
            cards_played.push(Card::new(Rank::Two, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Five, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Ten, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::King, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Ace, Suit::Spades, None, None).to_string());
        }
        8 => {
            // Scenario: Five of a Kind with enhancements
            let rank = if rng.gen_bool(0.5) {
                Rank::Ace
            } else {
                Rank::King
            };
            let mut suits = vec![
                Suit::Hearts,
                Suit::Diamonds,
                Suit::Clubs,
                Suit::Spades,
                Suit::Hearts,
            ];
            suits.shuffle(rng);

            for i in 0..5 {
                let mut card_str = Card::new(rank, suits[i], None, None).to_string();
                if rng.gen_bool(0.7) {
                    // Add random enhancement
                    let enhancement = match rng.gen_range(0..5) {
                        0 => Enhancement::Bonus,
                        1 => Enhancement::Mult,
                        2 => Enhancement::Wild,
                        3 => Enhancement::Glass,
                        _ => Enhancement::Steel,
                    };
                    card_str = format!("{} {}", card_str, enhancement);
                }
                cards_played.push(card_str);
            }
        }
        9 => {
            // Scenario: Wild cards making multiple possible hands
            // Create a hand with several Wild cards that could be interpreted in different ways
            let mut card_strs = Vec::new();
            card_strs.push(Card::new(Rank::Ten, Suit::Hearts, None, None).to_string() + " Wild");
            card_strs.push(Card::new(Rank::Jack, Suit::Spades, None, None).to_string() + " Wild");
            card_strs.push(Card::new(Rank::Queen, Suit::Diamonds, None, None).to_string());
            card_strs.push(Card::new(Rank::King, Suit::Clubs, None, None).to_string());
            card_strs.push(Card::new(Rank::Ace, Suit::Hearts, None, None).to_string());

            cards_played = card_strs;
        }
        _ => {
            // Fallback to a simple random hand
            cards_played.push(Card::new(Rank::Ten, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Jack, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Queen, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::King, Suit::Hearts, None, None).to_string());
            cards_played.push(Card::new(Rank::Ace, Suit::Hearts, None, None).to_string());
        }
    }

    Round {
        cards_played,
        cards_held_in_hand,
        jokers,
    }
}

/// Save a round to a YAML file
fn save_round_to_yaml(round: &Round, path: &Path) -> io::Result<()> {
    let yaml_str = serde_yaml::to_string(round).unwrap();
    fs::write(path, yaml_str)?;
    Ok(())
}

/// Run a command and return its output
fn run_command(cmd: &str, args: &[&str]) -> io::Result<Output> {
    Command::new(cmd).args(args).output()
}

/// Run the reference solution on a given round file
fn run_reference_solution(round_path: &Path) -> io::Result<String> {
    let output = run_command("6991", &["ortalab", &round_path.to_string_lossy()])?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Reference solution failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}

/// Run your solution on a given round file
fn run_your_solution(round_path: &Path) -> io::Result<String> {
    let output = run_command("./target/debug/ortalab", &[&round_path.to_string_lossy()])?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Your solution failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}

/// Create the test directory if it doesn't exist
fn ensure_test_dir() -> io::Result<()> {
    let test_dir = Path::new("fuzzer_tests");
    if !test_dir.exists() {
        fs::create_dir(test_dir)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Ortalab Fuzzer - Testing your solution against the reference implementation");

    // Create a directory for the test YAML files
    ensure_test_dir()?;

    // Build your solution
    println!("Building your solution...");
    let build_output = run_command("cargo", &["build"])?;
    if !build_output.status.success() {
        eprintln!("Failed to build your solution");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Build failed".to_string(),
        ));
    }

    // Initialize RNG and test parameters
    let mut rng = thread_rng();
    let num_random_tests = 50;
    let num_targeted_tests = 50;
    let mut failed_tests = Vec::new();
    let mut passed_count = 0;

    println!(
        "Running {} random tests and {} targeted tests...",
        num_random_tests, num_targeted_tests
    );

    // Create test log file
    let log_file_path = Path::new("fuzzer_tests/test_log.txt");
    let mut log_file = File::create(log_file_path)?;

    // Run random tests
    for i in 0..num_random_tests {
        let test_name = format!("random_test_{:03}", i);
        let round_path = Path::new("fuzzer_tests").join(format!("{}.yml", test_name));

        // Generate a random round
        let round = generate_random_round(
            &mut rng, 1,    // min_cards_played
            5,    // max_cards_played
            5,    // max_cards_in_hand
            5,    // max_jokers
            true, // allow_enhancements
            true, // allow_editions
        );

        // Save the round to a YAML file
        save_round_to_yaml(&round, &round_path)?;

        // Run both solutions and compare results
        match (
            run_reference_solution(&round_path),
            run_your_solution(&round_path),
        ) {
            (Ok(ref_result), Ok(your_result)) => {
                if ref_result == your_result {
                    passed_count += 1;
                    writeln!(log_file, "PASS: {} (Score: {})", test_name, ref_result)?;
                } else {
                    failed_tests.push((test_name.clone(), ref_result.clone(), your_result.clone()));
                    writeln!(
                        log_file,
                        "FAIL: {} (Reference: {}, Yours: {})",
                        test_name, ref_result, your_result
                    )?;
                }
            }
            (Err(ref_err), _) => {
                writeln!(log_file, "ERROR (Reference): {} - {}", test_name, ref_err)?;
            }
            (_, Err(your_err)) => {
                writeln!(
                    log_file,
                    "ERROR (Your solution): {} - {}",
                    test_name, your_err
                )?;
            }
        }

        // Print progress
        if (i + 1) % 10 == 0 {
            println!("Completed {}/{} random tests", i + 1, num_random_tests);
        }
    }

    // Run targeted tests
    for i in 0..num_targeted_tests {
        let test_name = format!("targeted_test_{:03}", i);
        let round_path = Path::new("fuzzer_tests").join(format!("{}.yml", test_name));

        // Generate a targeted round focusing on specific edge cases
        let round = generate_targeted_round(&mut rng);

        // Save the round to a YAML file
        save_round_to_yaml(&round, &round_path)?;

        // Run both solutions and compare results
        match (
            run_reference_solution(&round_path),
            run_your_solution(&round_path),
        ) {
            (Ok(ref_result), Ok(your_result)) => {
                if ref_result == your_result {
                    passed_count += 1;
                    writeln!(log_file, "PASS: {} (Score: {})", test_name, ref_result)?;
                } else {
                    failed_tests.push((test_name.clone(), ref_result.clone(), your_result.clone()));
                    writeln!(
                        log_file,
                        "FAIL: {} (Reference: {}, Yours: {})",
                        test_name, ref_result, your_result
                    )?;
                }
            }
            (Err(ref_err), _) => {
                writeln!(log_file, "ERROR (Reference): {} - {}", test_name, ref_err)?;
            }
            (_, Err(your_err)) => {
                writeln!(
                    log_file,
                    "ERROR (Your solution): {} - {}",
                    test_name, your_err
                )?;
            }
        }

        // Print progress
        if (i + 1) % 10 == 0 {
            println!("Completed {}/{} targeted tests", i + 1, num_targeted_tests);
        }
    }

    // Print summary
    let total_tests = num_random_tests + num_targeted_tests;
    println!("\nTesting complete!");
    println!(
        "Passed: {}/{} tests ({:.1}%)",
        passed_count,
        total_tests,
        (passed_count as f64 / total_tests as f64) * 100.0
    );

    if !failed_tests.is_empty() {
        println!("\nFailed tests:");
        for (test_name, ref_result, your_result) in &failed_tests {
            println!(
                "  {}: Reference={}, Yours={}",
                test_name, ref_result, your_result
            );
        }

        // Save a detailed report of failed tests
        let failed_report_path = Path::new("fuzzer_tests/failed_tests_report.txt");
        let mut failed_report = File::create(failed_report_path)?;

        writeln!(failed_report, "Failed Tests Report")?;
        writeln!(failed_report, "==================\n")?;

        for (test_name, ref_result, your_result) in &failed_tests {
            writeln!(failed_report, "Test: {}", test_name)?;
            writeln!(failed_report, "Reference result: {}", ref_result)?;
            writeln!(failed_report, "Your result: {}", your_result)?;

            // Read the YAML file to include in the report
            let yaml_path = Path::new("fuzzer_tests").join(format!("{}.yml", test_name));
            let yaml_content = fs::read_to_string(yaml_path)?;
            writeln!(failed_report, "\nTest case YAML:")?;
            writeln!(failed_report, "{}", yaml_content)?;
            writeln!(failed_report, "\n{}", "-".repeat(50))?;
        }

        println!(
            "\nDetailed report of failed tests saved to: {:?}",
            failed_report_path
        );
    } else {
        println!("All tests passed! Your implementation matches the reference solution.");
    }

    println!("\nTest log saved to: {:?}", log_file_path);

    Ok(())
}
