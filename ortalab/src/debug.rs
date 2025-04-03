//! # Debug Module
//!
//! This module provides macros for conditional debug output in the game.

/// Prints debug information based on a boolean flag.
#[macro_export]
macro_rules! explain_dbg_bool {
    ($enabled:expr, $($arg:tt)*) => {
        if $enabled {
            println!($($arg)*);
        }
    };
}

/// Prints debug information based on a game state's explain flag.
#[macro_export]
macro_rules! explain_dbg {
    ($state:expr, $($arg:tt)*) => {
        if $state.explain_enabled {
            println!($($arg)*);
        }
    };
}
