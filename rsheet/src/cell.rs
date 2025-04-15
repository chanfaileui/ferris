use rsheet_lib::cell_value::CellValue;

pub struct Cell {
    expr: Option<String>, // have to call CellExpr to evaluate
    value: CellValue,
    timestamp: u64, // used to prevent older updates overwriting newer ones
}

