// src/jokers/basic.rs
use crate::errors::GameResult;
use crate::game::GameState;
use crate::jokers::ActivationType;
use crate::jokers::JokerEffect;
use ortalib::JokerCard;

use crate::explain_dbg;

// ✖ Mult +4
pub struct Joker;

impl JokerEffect for Joker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.mult += 4.0;
        let message = format!(
            "{} +4 Mult ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// ✖ Mult +8 if cards played contains a Pair
pub struct JollyJoker;

impl JokerEffect for JollyJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_pair
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.mult += 8.0;
        let message = format!(
            "{} +8 Mult ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// ✖ Mult +12 if cards played contains a Three of a Kind
pub struct ZanyJoker;

impl JokerEffect for ZanyJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_three_of_a_kind
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.mult += 12.0;
        let message = format!(
            "{} +12 Mult ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// ✖ Mult +10 if cards played contains a Two Pair
pub struct MadJoker;

impl JokerEffect for MadJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_two_pair
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.mult += 10.0;
        let message = format!(
            "{} +10 Mult ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// ✖ Mult +12 if cards played contains a Straight
pub struct CrazyJoker;

impl JokerEffect for CrazyJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_straight
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.mult += 12.0;
        let message = format!(
            "{} +12 Mult ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// ✖ Mult +10 if cards played contains a Flush
pub struct DrollJoker;

impl JokerEffect for DrollJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_flush
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.mult += 10.0;
        let message = format!(
            "{} +10 Mult ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// 🪙 Chips +50 if cards played contains a Pair
pub struct SlyJoker;

impl JokerEffect for SlyJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_pair
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.chips += 50.0;
        let message = format!(
            "{} +50 Chips ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// 🪙 Chips +100 if cards played contains a Three of a Kind
pub struct WilyJoker;

impl JokerEffect for WilyJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_three_of_a_kind
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.chips += 100.0;
        let message = format!(
            "{} +100 Chips ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// 🪙 Chips +80 if cards played contains a Two Pair
pub struct CleverJoker;

impl JokerEffect for CleverJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_two_pair
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.chips += 80.0;
        let message = format!(
            "{} +80 Chips ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// 🪙 Chips +100 if cards played contains a Straight
pub struct DeviousJoker;

impl JokerEffect for DeviousJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_straight
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.chips += 100.0;
        let message = format!(
            "{} +100 Chips ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// 🪙 Chips +80 if cards played contains a Flush
pub struct CraftyJoker;

impl JokerEffect for CraftyJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn can_apply(&self, game_state: &GameState) -> bool {
        game_state.contains_flush
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        game_state.chips += 80.0;
        let message = format!(
            "{} +80 Chips ({} x {})",
            joker_card.joker, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}

// ✖ Mult +3 for each Joker card
pub struct AbstractJoker;

impl JokerEffect for AbstractJoker {
    fn activation_type(&self) -> ActivationType {
        ActivationType::Independent
    }

    fn apply(&self, game_state: &mut GameState, joker_card: &JokerCard) -> GameResult<()> {
        let joker_count = game_state.round.jokers.len();
        let mult_increase = 3.0 * (joker_count as f64);
        game_state.mult += mult_increase;
        let message = format!(
            "{} +{} Mult ({} x {})",
            joker_card.joker, mult_increase, game_state.chips, game_state.mult
        );
        explain_dbg!(game_state, "{}", message);
        Ok(())
    }
}
