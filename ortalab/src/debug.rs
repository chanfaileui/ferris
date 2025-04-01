#[macro_export]
macro_rules! explain_dbg {
    ($state:expr, $($arg:tt)*) => {
        if $state.explain_enabled {
            println!($($arg)*);
        }
    };
}