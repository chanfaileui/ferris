use rsheet_lib::{cell_value::CellValue, command::CellIdentifier};

use crate::cell::Cell;
use std::collections::{HashMap, HashSet};

pub struct Spreadsheet {
    // cells themselves (Hashmap, key: value)
    cells: HashMap<CellIdentifier, Cell>,
    // cell -> dependecies
    dependencies: HashMap<CellIdentifier, HashSet<CellIdentifier>>,
    // dependencies -> cell
    reverse_dependencies: HashMap<CellIdentifier, HashSet<CellIdentifier>>,
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self::new()
    }
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
        }
    }

    pub fn get(&self, cell: &CellIdentifier) -> CellValue {
        match self.cells.get(cell) {
            Some(cell) => cell.value().clone(),
            None => CellValue::None, // Default for empty cell
        }
    }

    pub fn set(&mut self, cell_identifier: CellIdentifier, cell: Cell) {
        self.cells.insert(cell_identifier, cell);
    }

    pub fn cell_exists(&self, cell: &CellIdentifier) -> bool {
        self.cells.contains_key(cell)
    }

    pub fn evaluate_cell(
        &mut self,
        cell_id: CellIdentifier,
        cell: Cell,
        dependencies: HashSet<CellIdentifier>,
    ) {
        // Store the cell
        self.cells.insert(cell_id, cell);

        // Update dependencies map
        self.dependencies.insert(cell_id, dependencies.clone());

        // Update reverse dependencies
        for dep in dependencies {
            self.reverse_dependencies
                .entry(dep)
                .or_default()
                .insert(cell_id);
        }
    }
}
