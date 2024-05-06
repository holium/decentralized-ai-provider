use admin::handle_admin_request;
use kinode_process_lib::{await_message, call_init, println, Address};

use types::State;

mod admin;
mod chain;
pub mod types;
mod user;
mod worker;

use user::handle_user_request;
use worker::handle_worker_request;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

fn handle_message(our: &Address, state: &mut State) -> anyhow::Result<(), anyhow::Error> {
    let message = match await_message() {
        Ok(message) => message,
        Err(e) => {
            println!("error: {:?}", e);
            return Err(anyhow::anyhow!("error: {:?}", e));
        }
    };

    if message.is_request() {
        if message.source().node == our.node {
            if handle_admin_request(&message, state).is_ok() {
                return Ok(());
            }
        }
        if handle_worker_request(our, &message, state).is_ok() {
            return Ok(());
        }
        if handle_user_request(our, &message, state).is_ok() {
            return Ok(());
        }
    }

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    let mut state: State = match State::load() {
        Ok(s) => s,
        Err(e) => {
            println!("error loading state: {:?}", e);
            State::default()
        }
    };

    loop {
        if let Err(e) = handle_message(&our, &mut state) {
            println!("error: {:?}", e);
        }
        state.save().unwrap();
    }
}

pub fn print_state(state: &State) {
    println!("{}", serde_json::to_string_pretty(&state).unwrap());
}
