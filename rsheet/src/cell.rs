use std::time::{SystemTime, UNIX_EPOCH};

use rsheet_lib::cell_value::CellValue;

pub struct Cell {
    expr: Option<String>, // have to call CellExpr to evaluate
    value: CellValue,
    timestamp: u64, // used to prevent older updates overwriting newer ones
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Self {
            expr: None,
            value,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn new_with_expr(expr: String, value: CellValue) -> Self {
        Self {
            expr: Some(expr),
            value,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn value(&self) -> &CellValue {
        &self.value
    }

    pub fn expr(&self) -> Option<&String> {
        self.expr.as_ref()
    }
}
