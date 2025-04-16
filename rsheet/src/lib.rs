use cell::Cell;
use env_logger::fmt::Timestamp;
use rsheet_lib::cell_expr::CellExpr;
use rsheet_lib::command::Command;
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;
use spreadsheet::Spreadsheet;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;

use log::info;

mod cell;
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
                let reply = match msg.parse::<Command>() {
                    Ok(command) => match command {
                        Command::Get { cell_identifier } => {
                            let sheet = spreadsheet.read().unwrap();
                            let val = sheet.get(&cell_identifier);
                            match val {
                                Some(val) => Reply::Value(format!("{:?}", cell_identifier), val.to_string()),
                                None => Reply::Error("Cell not found".into()),
                            }
                        },
                        Command::Set {
                            cell_identifier,
                            cell_expr,
                        } => {
                            let sheet = spreadsheet.write().unwrap();
                            let cell_expr = CellExpr::new(&cell_expr).evaluate();
 
                            let cell = Cell {
                                expr: match cell_expr {
                                    Ok(val) => Some(val.to_string()),
                                    Err(_) => None,
                                },
                                value: match cell_expr {
                                    Ok(val) => val,
                                    Err(_) => return Err(Box::new(std::io::Error::new(
                                        std::io::ErrorKind::InvalidData,
                                        "Failed to evaluate cell expression",
                                    ))),
                                },
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            };
                            sheet.set(cell_identifier.clone(), cell);
                            return Ok(());
                        },
                    },
                    Err(e) => Reply::Error(e),
                };

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
