use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

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
    let (range1, range2) = match (
        CellIdentifier::from_str(range[0]),
        CellIdentifier::from_str(range[1]),
    ) {
        (Ok(r1), Ok(r2)) => (r1, r2),
        _ => return,
    };

    let mut collect_values = |rows: std::ops::RangeInclusive<u32>,
                              cols: std::ops::RangeInclusive<u32>|
     -> Vec<CellValue> {
        let mut values = Vec::new();
        for row in rows {
            for col in cols.clone() {
                let cell_id = CellIdentifier { row, col };
                dependencies.insert(cell_id);
                values.push(spreadsheet.get_value(&cell_id));
            }
        }
        values
    };

    if range1.col == range2.col {
        // Vertical column
        let values = collect_values(range1.row..=range2.row, range1.col..=range1.col);
        variables.insert(cell_variable.to_string(), CellArgument::Vector(values));
    } else if range1.row == range2.row {
        // Horizontal row
        let values = collect_values(range1.row..=range1.row, range1.col..=range2.col);
        variables.insert(cell_variable.to_string(), CellArgument::Vector(values));
    } else {
        let mut matrix_values: Vec<Vec<CellValue>> = Vec::new();
        for row in range1.row..=range2.row {
            let row_values = collect_values(row..=row, range1.col..=range2.col);
            matrix_values.push(row_values);
        }
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
