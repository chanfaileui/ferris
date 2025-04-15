use rsheet_lib::command::CellIdentifier;

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

impl Spreadsheet {
    pub fn new() -> Self {
        Spreadsheet {
            cells: HashMap::new(),
            dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
        }
    }
    pub fn get(&self, cell: &CellIdentifier) -> Option<&Cell> {
        self.cells.get(cell)
    }
    pub fn set(&mut self, cell: CellIdentifier, value: Cell) {
        self.cells.insert(cell, value);
    }
    pub fn add_dependency(&mut self, cell: CellIdentifier, dependency: CellIdentifier) {
        todo!()
    }
    pub fn remove_dependency(&mut self, cell: &CellIdentifier, dependency: &CellIdentifier) {
        todo!()
    }
}
