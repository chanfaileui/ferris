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
                        } => todo!(),
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
