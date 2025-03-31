use crate::errors::GameResult;
use ortalib::{Card, Chips, Edition, Enhancement, Mult};

pub fn apply_enhancement(
    card: &Card,
    chips: &mut Chips,
    mult: &mut Mult,
) -> GameResult<Vec<String>> {
    let mut explanations = Vec::new();
    match card.enhancement {
        Some(Enhancement::Bonus) => {
            *chips += 30.0;
            explanations.push(format!("{} +30 Chips ({} x {})", card, *chips, *mult));
        }
        Some(Enhancement::Mult) => {
            *mult += 4.0;
            explanations.push(format!("{} +4 Mult ({} x {})", card, *chips, *mult));
        }
        Some(Enhancement::Glass) => {
            *mult *= 2.0;
            explanations.push(format!("{} x2 Mult ({} x {})", card, *chips, *mult));
        }
        Some(Enhancement::Steel) => {
            // ✖️ Mult×1.5 if this card is held in hand
            // This is handled elsewhere for cards held in hand
        }
        Some(Enhancement::Wild) => {
            // Wild card effects are handled during hand identification
        }
        None => (),
    }
    Ok(explanations)
}

pub fn apply_edition(card: &Card, chips: &mut Chips, mult: &mut Mult) -> GameResult<Vec<String>> {
    let mut explanations = Vec::new();
    match card.edition {
        Some(Edition::Foil) => {
            *chips += 50.0;
            explanations.push(format!("{} +50 Chips ({} x {})", card, *chips, *mult));
        }
        Some(Edition::Holographic) => {
            *mult += 10.0;
            explanations.push(format!("{} +10 Mult ({} x {})", card, *chips, *mult));
        }
        Some(Edition::Polychrome) => {
            *mult *= 1.5;
            explanations.push(format!("{} x1.5 Mult ({} x {})", card, *chips, *mult));
        }
        None => (),
    }
    Ok(explanations)
}

// Function to handle Steel enhancement for cards held in hand
pub fn apply_steel_enhancement(
    card: &Card,
    chips: &mut Chips,
    mult: &mut Mult,
) -> GameResult<Vec<String>> {
    let mut explanations = Vec::new();
    if let Some(Enhancement::Steel) = card.enhancement {
        *mult *= 1.5;
        explanations.push(format!("{} Steel x1.5 Mult ({} x {})", card, *chips, *mult));
    }
    Ok(explanations)
}
