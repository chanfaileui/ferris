//! # Error Handling Module
//!
//! This module provides error types and result aliases for the game.
//! It centralises error handling to provide consistent error reporting
//! throughout the application.
use std::fmt;

/// Represents errors that can occur during game operations
#[derive(Debug)]
pub enum GameError {
    /// Error indicating an invalid hand configuration with a descriptive message
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

/// A specialised Result type for game operations
///
/// This type alias is used throughout the codebase to represent
/// results of operations that might fail with a `GameError`.
pub type GameResult<T> = Result<T, GameError>;
