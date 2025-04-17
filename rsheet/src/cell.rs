use rsheet_lib::cell_value::CellValue;

pub struct Cell {
    // expr: Option<String>, // have to call CellExpr to evaluate
    value: CellValue,
    timestamp: u64, // used to prevent older updates overwriting newer ones
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Self {
            // expr,
            value,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    pub fn get_value(&self) -> &CellValue {
        &self.value
    }
    pub fn set_value(&mut self, value: CellValue) {
        self.value = value;
        self.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}
