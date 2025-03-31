use crate::errors::GameResult;
use ortalib::{Card, Chips, Edition, Enhancement, Mult};

pub fn apply_enhancement(
    card: &Card,
    chips: &mut Chips,
    mult: &mut Mult,
    add_explanation: impl Fn(String),
) -> GameResult<()> {
    todo!()
}

pub fn apply_edition(
    card: &Card,
    chips: &mut Chips,
    mult: &mut Mult,
    add_explanation: impl Fn(String),
) -> GameResult<()> {
    todo!()
}
