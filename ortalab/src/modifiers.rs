//! # Modifiers Module
//!
//! This module handles card modifiers such as enhancements and editions.
//! It provides functions to apply various card modifiers to the game state.

use crate::errors::GameResult;
use ortalib::{Card, Chips, Edition, Enhancement, Mult};

use crate::explain_dbg_bool;

/// Applies enhancement effects to the game state
pub fn apply_enhancement(
    card: &Card,
    chips: &mut Chips,
    mult: &mut Mult,
    explain_enabled: bool,
) -> GameResult<()> {
    match card.enhancement {
        Some(Enhancement::Bonus) => {
            *chips += 30.0;
            explain_dbg_bool!(
                explain_enabled,
                "{} +30 Chips ({} x {})",
                card,
                *chips,
                *mult
            );
        }
        Some(Enhancement::Mult) => {
            *mult += 4.0;
            explain_dbg_bool!(explain_enabled, "{} +4 Mult ({} x {})", card, *chips, *mult);
        }
        Some(Enhancement::Glass) => {
            *mult *= 2.0;
            explain_dbg_bool!(explain_enabled, "{} x2 Mult ({} x {})", card, *chips, *mult);
        }
        Some(Enhancement::Steel) => {
            // Steel enhancement is handled in apply_steel_enhancement function
        }
        Some(Enhancement::Wild) => {
            // Wild card effects are handled during hand identification
        }
        None => (),
    }
    Ok(())
}

/// Applies edition effects to the game state
pub fn apply_edition(
    card: &Card,
    chips: &mut Chips,
    mult: &mut Mult,
    explain_enabled: bool,
) -> GameResult<()> {
    match card.edition {
        Some(Edition::Foil) => {
            *chips += 50.0;
            explain_dbg_bool!(
                explain_enabled,
                "{} +50 Chips ({} x {})",
                card,
                *chips,
                *mult
            );
        }
        Some(Edition::Holographic) => {
            *mult += 10.0;
            explain_dbg_bool!(
                explain_enabled,
                "{} +10 Mult ({} x {})",
                card,
                *chips,
                *mult
            );
        }
        Some(Edition::Polychrome) => {
            *mult *= 1.5;
            explain_dbg_bool!(
                explain_enabled,
                "{} x1.5 Mult ({} x {})",
                card,
                *chips,
                *mult
            );
        }
        None => (),
    }
    Ok(())
}

/// Applies Steel enhancement effects for cards held in hand
pub fn apply_steel_enhancement(
    card: &Card,
    chips: &mut Chips,
    mult: &mut Mult,
    explain_enabled: bool,
) -> GameResult<()> {
    if let Some(Enhancement::Steel) = card.enhancement {
        *mult *= 1.5;
        explain_dbg_bool!(
            explain_enabled,
            "{} x1.5 Mult ({} x {})",
            card,
            *chips,
            *mult
        );
    }
    Ok(())
}
