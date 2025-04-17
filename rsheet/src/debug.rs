// usage
// log_debug!("cell_variable", cell_variable);
// log_debug!("vector_values", vector_values);
// log_debug!("range1", range1);

macro_rules! log_debug {
    ($name:expr, $value:expr) => {{
        use std::fs::OpenOptions;
        use std::io::Write;
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug_log.txt")
        {
            let _ = writeln!(file, "{}: {:?}", $name, $value);
        }
    }};
}