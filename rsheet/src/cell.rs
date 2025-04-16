use rsheet_lib::cell_value::CellValue;

pub struct Cell {
    pub expr: Option<String>, // have to call CellExpr to evaluate
    pub value: CellValue,
    pub timestamp: u64, // used to prevent older updates overwriting newer ones
}

impl Cell {
    pub fn new(expr: Option<String>, value: CellValue) -> Self {
        Self {
            expr,
            value,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
