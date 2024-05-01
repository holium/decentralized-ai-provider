use broker_handler::handle_broker_request;
use kinode_process_lib::{await_message, call_init, println, Address};

use crate::{admin::handle_admin_request, types::WorkerState};

mod admin;
mod broker_handler;
pub mod chain;
mod types;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

fn handle_message(our: &Address, state: &mut WorkerState) -> anyhow::Result<(), anyhow::Error> {
    let message = match await_message() {
        Ok(message) => message,
        Err(e) => {
            println!("error: {:?}", e);
            return Err(anyhow::anyhow!("error: {:?}", e));
        }
    };

    if message.is_request() {
        if message.source().node == our.node {
            return match handle_admin_request(&message, state) {
                Ok(()) => Ok(()),
                Err(e) => {
                    println!("---> error: {:?}", e);
                    Err(e)
                }
            };
        }
        if handle_broker_request(our, &message, state).is_ok() {
            return Ok(());
        }
    }

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    let mut state: WorkerState = WorkerState::load();

    loop {
        match handle_message(&our, &mut state) {
            Ok(()) => {}
            Err(e) => {
                println!("error: {:?}", e);
            }
        };
    }
}
