use cell::Cell;
use rsheet_lib::cell_expr::{CellArgument, CellExpr};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;
use spreadsheet::Spreadsheet;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;

use log::info;

mod cell;
#[macro_use]
mod debug;
mod spreadsheet;

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let spreadsheet = Arc::new(RwLock::new(Spreadsheet::new()));

    loop {
        match manager.accept_new_connection() {
            Connection::NewConnection { reader, writer } => {
                let spreadsheet_clone = Arc::clone(&spreadsheet);
                thread::spawn(move || {
                    if let Err(e) = handle_connection(reader, writer, spreadsheet_clone) {
                        eprintln!("Error in connection handler: {}", e);
                    }
                })
            }
            Connection::NoMoreConnections => {
                // There are no more new connections to accept.
                return Ok(());
            }
        };
    }
}

pub fn handle_connection<R, W>(
    mut recv: R,
    mut send: W,
    spreadsheet: Arc<RwLock<Spreadsheet>>,
) -> Result<(), Box<dyn Error>>
where
    R: Reader,
    W: Writer,
{
    loop {
        info!("Just got message");
        match recv.read_message() {
            ReadMessageResult::Message(msg) => {
                // rsheet_lib already contains a FromStr<Command> (i.e. parse::<Command>)
                // implementation for parsing the get and set commands. This is just a
                // demonstration of how to use msg.parse::<Command>, you may want/have to
                // change this code.
                let maybe_reply: Option<Reply> = match msg.parse::<Command>() {
                    Ok(command) => match command {
                        Command::Get { cell_identifier } => {
                            let sheet = spreadsheet.read().unwrap();
                            let val = sheet.get(&cell_identifier);
                            Some(Reply::Value(
                                cell_identifier_to_string(cell_identifier),
                                val,
                            ))
                        }
                        Command::Set {
                            cell_identifier,
                            cell_expr,
                        } => {
                            let cell_expr = CellExpr::new(&cell_expr);
                            let cell_variables = cell_expr.find_variable_names();

                            let mut variables: HashMap<String, CellArgument> = HashMap::new();
                            if !cell_variables.is_empty() {
                                let sheet_read = spreadsheet.read().unwrap();
                                for cell_variable in cell_variables {
                                    if cell_variable.contains("_") {
                                        let range: Vec<&str> = cell_variable.split("_").collect();
                                        let range1 = match CellIdentifier::from_str(range[0]) {
                                            Ok(identifier) => identifier,
                                            Err(_) => continue,
                                        };
                                        let range2 = match CellIdentifier::from_str(range[1]) {
                                            Ok(identifier) => identifier,
                                            Err(_) => continue,
                                        };

                                        if range1.col == range2.col {
                                            let mut vector_values: Vec<CellValue> = Vec::new();
                                            for row in range1.row..=range2.row {
                                                let cell_id = CellIdentifier {
                                                    row,
                                                    col: range1.col,
                                                };
                                                let value = sheet_read.get(&cell_id);
                                                vector_values.push(value);
                                            }

                                            variables.insert(
                                                cell_variable,
                                                CellArgument::Vector(vector_values),
                                            );
                                        } else if range1.row == range2.row {
                                            let mut vector_values = Vec::new();
                                            for col in range1.col..=range2.col {
                                                let cell_id = CellIdentifier {
                                                    row: range1.row,
                                                    col,
                                                };
                                                let value = sheet_read.get(&cell_id);
                                                vector_values.push(value);
                                            }
                                            // log::info!("Debug: Vector values: {:?}", vector_values);
                                            variables.insert(
                                                cell_variable,
                                                CellArgument::Vector(vector_values),
                                            );
                                        } else {
                                            let mut matrix_values: Vec<Vec<CellValue>> = Vec::new();
                                            for col in range1.col..=range2.col {
                                                let mut col_values = Vec::new();
                                                for row in range1.row..=range2.row {
                                                    let cell_id = CellIdentifier { col, row };
                                                    let value = sheet_read.get(&cell_id);
                                                    col_values.push(value);
                                                }
                                                matrix_values.push(col_values);
                                            }
                                            variables.insert(
                                                cell_variable,
                                                CellArgument::Matrix(matrix_values),
                                            );
                                        }
                                    } else {
                                        let cell_identifier =
                                            match CellIdentifier::from_str(&cell_variable) {
                                                Ok(identifier) => identifier,
                                                Err(_) => continue, // Skip invalid identifiers
                                            };
                                        let val = sheet_read.get(&cell_identifier);
                                        variables.insert(cell_variable, CellArgument::Value(val));
                                    }
                                }
                            }

                            let mut sheet = spreadsheet.write().unwrap();
                            match cell_expr.evaluate(&variables) {
                                Ok(value) => {
                                    sheet.set(cell_identifier, Cell::new(value));
                                    None
                                }
                                Err(e) => Some(Reply::Error(format!("Evaluation error: {:?}", e))),
                            }
                        }
                    },
                    Err(e) => Some(Reply::Error(format!("Invalid key provided: {:?}", e))),
                };

                if let Some(reply) = maybe_reply {
                    match send.write_message(reply) {
                        WriteMessageResult::Ok => {
                            // Message successfully sent, continue.
                        }
                        WriteMessageResult::ConnectionClosed => {
                            // The connection was closed. This is not an error, but
                            // should terminate this connection.
                            break;
                        }
                        WriteMessageResult::Err(e) => {
                            // An unexpected error was encountered.
                            return Err(Box::new(e));
                        }
                    }
                }
            }
            ReadMessageResult::ConnectionClosed => {
                // The connection was closed. This is not an error, but
                // should terminate this connection.
                break;
            }
            ReadMessageResult::Err(e) => {
                // An unexpected error was encountered.
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}

fn cell_identifier_to_string(identifer: CellIdentifier) -> String {
    let col_name = column_number_to_name(identifer.col);
    let row_number = identifer.row + 1;
    format!("{}{}", col_name, row_number)
}
