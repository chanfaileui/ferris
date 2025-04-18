use cell::Cell;
use eval::parse_variables_with_deps;
use rsheet_lib::cell_expr::{CellExpr, CellExprEvalError};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;
use spreadsheet::Spreadsheet;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use log::info;

mod cell;
mod eval;
mod spreadsheet;

pub struct UpdateMessage {
    cell_id: CellIdentifier,
}

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let spreadsheet = Arc::new(RwLock::new(Spreadsheet::new()));
    let (tx, rx) = mpsc::channel::<UpdateMessage>();
    let mut handles = Vec::new();

    // Spawn a thread to handle the spreadsheet updates
    let worker_spreadsheet = Arc::clone(&spreadsheet);
    let worker_handle = thread::spawn(move || {
        while let Ok(update_msg) = rx.recv() {
            if let Ok(mut spreadsheet) = worker_spreadsheet.write() {
                Spreadsheet::update_dependencies(&mut spreadsheet, update_msg.cell_id);
            }
        }
    });

    loop {
        match manager.accept_new_connection() {
            Connection::NewConnection { reader, writer } => {
                let spreadsheet_clone = Arc::clone(&spreadsheet);
                let tx_clone = tx.clone();
                let handle = thread::spawn(move || {
                    if let Err(e) = handle_connection(reader, writer, spreadsheet_clone, tx_clone) {
                        eprintln!("Error in connection handler: {}", e);
                    }
                });
                handles.push(handle);
            }
            Connection::NoMoreConnections => {
                // Wait for all worker threads to complete before exiting
                for handle in handles {
                    let _ = handle.join();
                }
                drop(tx);
                let _ = worker_handle.join();
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
    tx: mpsc::Sender<UpdateMessage>,
) -> Result<(), Box<dyn Error>>
where
    R: Reader,
    W: Writer,
{
    loop {
        info!("Just got message");
        match recv.read_message() {
            ReadMessageResult::Message(msg) => {
                let maybe_reply: Option<Reply> = match msg.parse::<Command>() {
                    Ok(command) => match command {
                        Command::Get { cell_identifier } => {
                            let sheet = spreadsheet.read().unwrap();
                            let val = sheet.get_value(&cell_identifier);
                            match val {
                                CellValue::Error(ref e) => {
                                    // TODO: a bit fragile - Checks if this is a dependency error
                                    if *e
                                        == format!(
                                            "{:?}",
                                            CellExprEvalError::VariableDependsOnError
                                        )
                                    {
                                        Some(Reply::Error(e.to_string()))
                                    } else {
                                        Some(Reply::Value(
                                            cell_identifier_to_string(cell_identifier),
                                            val,
                                        ))
                                    }
                                }
                                _ => {
                                    // For normal values, return the cell identifier and value
                                    Some(Reply::Value(
                                        cell_identifier_to_string(cell_identifier),
                                        val,
                                    ))
                                }
                            }
                        }
                        Command::Set {
                            cell_identifier,
                            cell_expr,
                        } => {
                            let cell_expr_obj = CellExpr::new(&cell_expr);
                            let cell_variables = cell_expr_obj.find_variable_names();

                            let (variables, dependencies) = if !cell_variables.is_empty() {
                                parse_variables_with_deps(
                                    &spreadsheet.read().unwrap(),
                                    cell_variables,
                                )
                            } else {
                                (HashMap::new(), HashSet::new())
                            };

                            let mut sheet = spreadsheet.write().unwrap();
                            match cell_expr_obj.evaluate(&variables) {
                                Ok(value) => {
                                    sheet.set(cell_identifier, Cell::new(&value));
                                    let cell = Cell::new_with_expr(cell_expr, value);
                                    sheet.evaluate_cell(cell_identifier, cell, dependencies);
                                }
                                Err(e) => {
                                    sheet.set(
                                        cell_identifier,
                                        Cell::new(&CellValue::Error(format!("{:?}", e))),
                                    );
                                }
                            }
                            tx.send(UpdateMessage {
                                cell_id: cell_identifier,
                            })?;
                            None
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
