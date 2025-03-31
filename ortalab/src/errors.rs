use std::fmt;

#[derive(Debug)]
pub enum GameError {
    InvalidHand(String),
    // ScoringError(String),
    // ParseError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidHand(msg) => write!(f, "Invalid hand: {}", msg),
            // GameError::ScoringError(msg) => write!(f, "Scoring error: {}", msg),
            // GameError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for GameError {}

pub type GameResult<T> = Result<T, GameError>;
