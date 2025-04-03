use std::fmt;

#[derive(Debug)]
pub enum GameError {
    InvalidHand(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidHand(msg) => write!(f, "Invalid hand: {}", msg),
        }
    }
}

impl std::error::Error for GameError {}

pub type GameResult<T> = Result<T, GameError>;
