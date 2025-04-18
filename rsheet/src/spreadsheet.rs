use rsheet_lib::{cell_expr, cell_value::CellValue, command::CellIdentifier};

use crate::{cell::Cell, eval::parse_variables_with_deps};
use std::collections::{HashMap, HashSet};

pub struct Spreadsheet {
    // cells themselves (Hashmap, key: value)
    cells: HashMap<CellIdentifier, Cell>,
    // cell -> dependecies (cell depends on these cells)
    dependencies: HashMap<CellIdentifier, HashSet<CellIdentifier>>,
    // dependency -> cells (what cells depend on this cell)
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

    pub fn get(&self, cell: &CellIdentifier) -> Option<&Cell> {
        match self.cells.get(cell) {
            Some(cell) => Some(cell),
            None => None,
        }
    }

    pub fn get_value(&self, cell: &CellIdentifier) -> CellValue {
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

    pub fn update_dependencies(&mut self, initial_cell: CellIdentifier) {
        // Use a queue to track cells that need updating
        let mut cells_to_update = Vec::new();
        cells_to_update.push(initial_cell);

        // Keep track of cells we've already processed
        let mut processed = HashSet::new();

        while let Some(current_cell) = cells_to_update.pop() {
            if !processed.insert(current_cell) {
                continue;
            }

            // Check if this cell has any dependent cells
            if let Some(dependent_cells) = self.reverse_dependencies.get(&current_cell) {
                for &dependent_cell_id in dependent_cells {
                    // Recalculate the dependent cell
                    // Get the expression, dependencies, etc.
                    // Then evaluate and update
                    if let Some(cell) = self.cells.get(&dependent_cell_id) {
                        if let Some(expr_str) = cell.expr() {
                            let cell_expr = cell_expr::CellExpr::new(expr_str);
                            let cell_variables = cell_expr.find_variable_names();

                            let (variables, _) = parse_variables_with_deps(self, cell_variables);
                            match cell_expr.evaluate(&variables) {
                                Ok(new_value) => {
                                    let new_cell =
                                        Cell::new_with_expr(expr_str.to_string(), new_value);

                                    // Check timestamp to prevent overwriting newer updates
                                    if let Some(existing_cell) = self.cells.get(&dependent_cell_id)
                                    {
                                        if existing_cell.timestamp() > new_cell.timestamp() {
                                            // Existing cell is newer, don't update
                                            continue;
                                        }
                                    }
                                    // Update the cell
                                    self.cells.insert(dependent_cell_id, new_cell);

                                    // Add dependent's dependents to the queue
                                    cells_to_update.push(dependent_cell_id);
                                }
                                Err(e) => {
                                    let error_cell = Cell::new_with_expr(
                                        expr_str.clone(),
                                        CellValue::Error(format!("{:?}", e)),
                                    );

                                    // Update the cell with the error
                                    self.cells.insert(dependent_cell_id, error_cell);

                                    // Propagate the error to dependents
                                    cells_to_update.push(dependent_cell_id);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
