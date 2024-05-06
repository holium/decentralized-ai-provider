use std::collections::HashMap;

use crate::chain;
use crate::types::{AdminRequest, OnChainState, State};
use kinode_process_lib::{println, Message};

pub fn handle_admin_request(
    message: &Message,
    state: &mut State,
) -> anyhow::Result<(), anyhow::Error> {
    match serde_json::from_slice::<AdminRequest>(message.body()) {
        Ok(AdminRequest::SyncChainState) => {
            println!("---> SyncChainState");
            let current_process: String = "diffusion:memedeck:memedeck.os".into();
            let apps = chain::get_applications();
            let processes = chain::get_processes();
            let brokers = chain::get_brokers(&current_process);
            let workers = chain::get_workers(&current_process);
            let mut chain_state = OnChainState {
                apps: apps
                    .into_iter()
                    .map(|app| (app.clone().name, app))
                    .collect(),
                processes: processes
                    .into_iter()
                    .map(|process| (process.clone().name, process))
                    .collect(),
                brokers: HashMap::new(),
                workers: HashMap::new(),
                queue_response_timeout_seconds: 10,
                serve_timeout_seconds: 10,
                max_outstanding_payments: 10,
                payment_period_hours: 1,
            };
            chain_state
                .brokers
                .insert(current_process.to_string(), brokers);
            chain_state
                .workers
                .insert(current_process.to_string(), workers);

            state.on_chain_state = chain_state;
            state.save().unwrap();
        }
        Ok(AdminRequest::SetIsReady { is_ready }) => {
            println!("---> SetIsReady {{ is_ready : {:?} }}", is_ready);
        }
        Ok(AdminRequest::SetContractAddress { address }) => {
            println!("---> SetContractAddress {{ address : {:?} }}", address);
        }
        Ok(AdminRequest::SetWorkerProcess { process_id }) => {
            println!("---> SetWorkerProcess {{ process_id : {:?} }}", process_id);
        }
        Ok(AdminRequest::GetState) => {
            println!("{}", serde_json::to_string_pretty(&state).unwrap());
        }
        _ => return Err(anyhow::anyhow!("Unknown admin request")),
    }
    Ok(())
}
