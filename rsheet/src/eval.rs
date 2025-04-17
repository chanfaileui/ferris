use std::{collections::HashMap, str::FromStr};

use rsheet_lib::{cell_expr::CellArgument, cell_value::CellValue, command::CellIdentifier};

use crate::spreadsheet::Spreadsheet;

// Extract this into a helper function
pub fn parse_variables(
    spreadsheet: &Spreadsheet,
    cell_variables: Vec<String>,
) -> HashMap<String, CellArgument> {
    let mut variables: HashMap<String, CellArgument> = HashMap::new();

    for cell_variable in cell_variables {
        if cell_variable.contains('_') {
            parse_range_variable(spreadsheet, &cell_variable, &mut variables);
        } else {
            parse_scalar_variable(spreadsheet, &cell_variable, &mut variables);
        }
    }
    variables
}

fn parse_range_variable(
    spreadsheet: &Spreadsheet,
    cell_variable: &String,
    variables: &mut HashMap<String, CellArgument>,
) {
    let range: Vec<&str> = cell_variable.split("_").collect();
    let range1 = match CellIdentifier::from_str(range[0]) {
        Ok(identifier) => identifier,
        Err(_) => return,
    };
    let range2 = match CellIdentifier::from_str(range[1]) {
        Ok(identifier) => identifier,
        Err(_) => return,
    };

    if range1.col == range2.col {
        let mut vector_values: Vec<CellValue> = Vec::new();
        for row in range1.row..=range2.row {
            let cell_id = CellIdentifier {
                row,
                col: range1.col,
            };
            let value = spreadsheet.get(&cell_id);
            vector_values.push(value);
        }

        variables.insert(
            cell_variable.to_string(),
            CellArgument::Vector(vector_values),
        );
    } else if range1.row == range2.row {
        let mut vector_values = Vec::new();
        for col in range1.col..=range2.col {
            let cell_id = CellIdentifier {
                row: range1.row,
                col,
            };
            let value = spreadsheet.get(&cell_id);
            vector_values.push(value);
        }
        // log::info!("Debug: Vector values: {:?}", vector_values);
        variables.insert(
            cell_variable.to_string(),
            CellArgument::Vector(vector_values),
        );
    } else {
        let mut matrix_values: Vec<Vec<CellValue>> = Vec::new();
        for col in range1.col..=range2.col {
            let mut col_values = Vec::new();
            for row in range1.row..=range2.row {
                let cell_id = CellIdentifier { col, row };
                let value = spreadsheet.get(&cell_id);
                col_values.push(value);
            }
            matrix_values.push(col_values);
        }
        variables.insert(
            cell_variable.to_string(),
            CellArgument::Matrix(matrix_values),
        );
    }
}

fn parse_scalar_variable(
    spreadsheet: &Spreadsheet,
    cell_variable: &String,
    variables: &mut HashMap<String, CellArgument>,
) {
    let cell_identifier = match CellIdentifier::from_str(cell_variable) {
        Ok(identifier) => identifier,
        Err(_) => return, // Skip invalid identifiers
    };
    let val = spreadsheet.get(&cell_identifier);
    variables.insert(cell_variable.to_string(), CellArgument::Value(val));
}
