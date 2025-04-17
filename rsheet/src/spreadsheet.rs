use rsheet_lib::{cell_value::CellValue, command::CellIdentifier};

use crate::cell::Cell;
use std::collections::{HashMap, HashSet};

pub struct Spreadsheet {
    // cells themselves (Hashmap, key: value)
    cells: HashMap<CellIdentifier, Cell>,
    // cell -> dependecies
    // dependencies: HashMap<CellIdentifier, HashSet<CellIdentifier>>,
    // // dependencies -> cell
    // reverse_dependencies: HashMap<CellIdentifier, HashSet<CellIdentifier>>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            // dependencies: HashMap::new(),
            // reverse_dependencies: HashMap::new(),
        }
    }

    pub fn get(&self, cell: &CellIdentifier) -> CellValue {
        match self.cells.get(cell) {
            Some(cell) => cell.get_value().clone(),
            None => CellValue::None, // Default for empty cell
        }
    }

    pub fn set(&mut self, cell_identifier: CellIdentifier, cell: Cell) {
        self.cells.insert(cell_identifier, cell);
    }

    // pub fn add_dependency(&mut self, cell: CellIdentifier, dependency: CellIdentifier) {
    //     todo!()
    // }

    // pub fn remove_dependency(&mut self, cell: &CellIdentifier, dependency: &CellIdentifier) {
    //     todo!()
    // }

    pub fn cell_exists(&self, cell: &CellIdentifier) -> bool {
        self.cells.contains_key(cell)
    }

    // pub fn evaluate_cell(&mut self, cell_id: &CellIdentifier) -> Option<CellValue> {
    //     // This is a placeholder - you'll need to implement this logic
    //     // for re-evaluating cells based on their expressions and dependencies
    //     todo!()
    // }
}
