#[macro_export]
macro_rules! explain_dbg_bool {
    ($enabled:expr, $($arg:tt)*) => {
        if $enabled {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! explain_dbg {
    ($state:expr, $($arg:tt)*) => {
        if $state.explain_enabled {
            println!($($arg)*);
        }
    };
}
