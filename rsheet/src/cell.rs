use rsheet_lib::{cell_expr::CellExpr, cell_value::CellValue};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cell {
    expr: Option<String>, // have to call CellExpr to evaluate
    value: CellValue,
    timestamp: u64, // used to prevent older updates overwriting newer ones
}

impl Cell {
    pub fn new(value: &CellValue) -> Self {
        Self {
            expr: None,
            value: value.clone(),
            timestamp: Self::current_timestamp(),
        }
    }

    pub fn new_with_expr(expr: String, value: CellValue) -> Self {
        Self {
            expr: Some(expr),
            value,
            timestamp: Self::current_timestamp(),
        }
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn value(&self) -> &CellValue {
        &self.value
    }

    pub fn expr(&self) -> Option<&String> {
        self.expr.as_ref()
    }

    pub fn get_cell_expr(&self) -> Option<CellExpr> {
        self.expr.as_deref().map(CellExpr::new)
    }

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }
}
