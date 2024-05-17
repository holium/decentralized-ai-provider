mod admin;
mod broker_handler;
pub mod chain;
mod types;

use broker_handler::handle_broker_request;
use kinode_process_lib::{await_message, call_init, println, Address, Message, Request};
use crate::{admin::handle_admin_request, types::WorkerState};
use shared::ProcessToWorkerRequests;
use shared::WorkerToBrokerRequests;

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
                        match handle_process_to_us_request(&message, &mut state) {
                            Ok(()) => (),
                            Err(e) => {
                                println!("process request error: {:?}", e);
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

fn handle_process_to_us_request(
    message: &Message,
    state: &mut types::WorkerState,
) -> anyhow::Result<()> {
    match serde_json::from_slice::<ProcessToWorkerRequests>(message.body()) {
        Ok(ProcessToWorkerRequests::TaskUpdate { task_id }) => {
            println!("worker procees got an update from the diffusion process for {task_id}");
            state.save()
        }
        Ok(ProcessToWorkerRequests::TaskComplete { task_id, process_id, broker }) => {
            // wait for the active_process to tell us that the task is finished before we
            // tell the broker TaskComplete, which will then automatically add us back to the
            // waiting workers queue
            println!("worker knows it's done with {task_id}, telling the broker now");
            Request::new()
                .target(broker)
                .body(serde_json::to_vec(&WorkerToBrokerRequests::TaskComplete {
                    process_id,
                    task_id,
                })?)
                .send()?;
            state.save()
        }
        _ => {
            Err(anyhow::anyhow!("Unknown process_to_worker request"))
        }
    }
}

