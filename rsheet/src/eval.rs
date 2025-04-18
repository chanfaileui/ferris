use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use log::info;
use rsheet_lib::{cell_expr::CellArgument, cell_value::CellValue, command::CellIdentifier};

use crate::spreadsheet::Spreadsheet;

// Extract this into a helper function
pub fn parse_variables_with_deps(
    spreadsheet: &Spreadsheet,
    cell_variables: Vec<String>,
) -> (HashMap<String, CellArgument>, HashSet<CellIdentifier>) {
    let mut variables: HashMap<String, CellArgument> = HashMap::new();
    let mut dependencies: HashSet<CellIdentifier> = HashSet::new();

    for cell_variable in cell_variables {
        if cell_variable.contains('_') {
            parse_range_variable_with_deps(
                spreadsheet,
                &cell_variable,
                &mut variables,
                &mut dependencies,
            );
        } else {
            parse_scalar_variable_with_deps(
                spreadsheet,
                &cell_variable,
                &mut variables,
                &mut dependencies,
            );
        }
    }
    (variables, dependencies)
}

fn parse_range_variable_with_deps(
    spreadsheet: &Spreadsheet,
    cell_variable: &String,
    variables: &mut HashMap<String, CellArgument>,
    dependencies: &mut HashSet<CellIdentifier>,
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

            dependencies.insert(cell_id);
            let value = spreadsheet.get_value(&cell_id);
            vector_values.push(value);
        }
        info!("Debug: Vector values: {:?}", vector_values);
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

            dependencies.insert(cell_id);
            let value = spreadsheet.get_value(&cell_id);
            vector_values.push(value);
        }
        info!("Debug: Vector values: {:?}", vector_values);
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
                dependencies.insert(cell_id);

                let value = spreadsheet.get_value(&cell_id);
                col_values.push(value);
            }
            matrix_values.push(col_values);
        }
        info!("Debug: Matrix values: {:?}", matrix_values);
        variables.insert(
            cell_variable.to_string(),
            CellArgument::Matrix(matrix_values),
        );
    }
}

fn parse_scalar_variable_with_deps(
    spreadsheet: &Spreadsheet,
    cell_variable: &String,
    variables: &mut HashMap<String, CellArgument>,
    dependencies: &mut HashSet<CellIdentifier>,
) {
    let cell_identifier = match CellIdentifier::from_str(cell_variable) {
        Ok(identifier) => identifier,
        Err(_) => return,
    };
    let val = spreadsheet.get_value(&cell_identifier);
    variables.insert(cell_variable.to_string(), CellArgument::Value(val));
    dependencies.insert(cell_identifier);
}
