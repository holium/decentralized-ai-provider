mod admin;
mod broker_handler;
pub mod chain;
mod types;

use broker_handler::handle_broker_request;
use kinode_process_lib::{await_message, call_init, println, Address};
use crate::{admin::handle_admin_request, types::WorkerState};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

call_init!(init);
fn init(our: Address) {
    println!("starting ai_provider:worker");
    let mut state: WorkerState = WorkerState::load();

    loop {
        match await_message() {
            Ok(message) => {
                if message.is_request() {
                    if message.source().node == our.node {
                        match handle_admin_request(&message, &mut state) {
                            Ok(()) => (),
                            Err(e) => {
                                println!("admin request error: {:?}", e);
                            }
                        };
                    } else {
                        match handle_broker_request(&our, &message, &mut state) {
                            Ok(()) => (),
                            Err(e) => {
                                println!("broker request error: {:?}", e);
                            }
                        }
                    }
                }
            },
            Err(e) => {
                println!("error: {:?}", e);
            }
        };
    }
}
