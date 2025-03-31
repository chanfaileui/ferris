use crate::errors::GameResult;
use ortalib::{Card, Chips, Edition, Enhancement, Mult};

pub fn apply_joker(
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
