mod admin;
mod chain;
pub mod types;
mod user;
mod worker;

use admin::handle_admin_request;
use kinode_process_lib::{await_message, call_init, println, Address, Message, get_blob};
use kinode_process_lib::http::{bind_http_path, HttpServerRequest, send_response, StatusCode};
use types::{GenerateRequest, State, string_to_process_id};
use user::handle_user_request;
use worker::handle_worker_request;
use crate::worker::assign_tasks_to_waiting_workers;
use std::collections::HashMap;
use web3::signing::{keccak256, recover};
use shared::TaskParameters;
use uuid::Uuid;

wit_bindgen::generate!({
    path: "target/wit",
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
            if handle_admin_request(&our, &message, state).is_ok() {
                return Ok(());
            }
        }
        if handle_worker_request(our, &message, state).is_ok() {
            return Ok(());
        }
        if handle_user_request(our, &message, state).is_ok() {
            return Ok(());
        }
        if message.source().process == "http_server:distro:sys" {
            return handle_http_request(our, &message, state);
        }
    }

    Ok(())
}

fn handle_http_request(our: &Address, message: &Message, state: &mut State) -> anyhow::Result<()> {
    let server_request = serde_json::from_slice::<HttpServerRequest>(message.body()).map_err(|e| {
        println!("Failed to parse server request: {:?}", e);
        e
    })?;

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    if let HttpServerRequest::Http(request) = server_request {
        let r_path = request.path()?;
        let r_path = r_path.as_str();
        let b_path = request.bound_path(Some(&our.process.to_string()));
        if request.method()?.as_str() == "POST" && b_path == "/generate_image" {
            // we validate that they actually have the right to generate an image by checking the `signature` property they sent against
            // the addresses that are whitelisted onchain
            // we expect the signature to be the whole minified json body of the task request so the overall request format is like:
            // {signature: "...", task: {...}}
            // if the signature matches some whitelisted address on chain, then it's all good and we can StartTask just like if a user
            // requested through the kinode console directly
            let Some(blob) = get_blob() else {
                return Ok(send_response(StatusCode::BAD_REQUEST, None, vec![]));
            };
            let parsed_request = serde_json::from_slice::<GenerateRequest>(&blob.bytes)?;
            for app in state.on_chain_state.apps.values() {
                for address in &app.whitelist {
                    println!("checking address: {}", address);
                    let account = address.to_string();
                    let formatted_message = parsed_request.task.to_string();
                    let formatted_message = eth_message(formatted_message);
                    let signature = hex::decode(&parsed_request.signature)?;
                    let recovery_id = signature[64] as i32 - 27;
                    let pubkey = recover(&formatted_message, &signature[..64], recovery_id)?;
                    let pubkey = format!("{:02X?}", pubkey);

                    if pubkey == account {
                        // we have finally found a whitelisted account
                        // that matches the signature on the request
                        let task_id = Uuid::new_v4().to_string();
                        let process_id = string_to_process_id(&parsed_request.process_id);
                        let task = TaskParameters {
                            process_id,
                            task_id: task_id.clone(),
                            from_broker: our.to_string(),
                            from_user: account,
                            task_parameters: parsed_request.task.clone(),
                        };

                        state.add_task(task_id.clone(), our.clone(), task);
                        println!(
                            "---> task: {}",
                            serde_json::to_string_pretty(&state.task_queue).unwrap()
                        );
                        return match assign_tasks_to_waiting_workers(state) {
                            Ok(_) => Ok(send_response(StatusCode::OK, Some(headers.clone()), serde_json::to_vec(&serde_json::json!({"result":"task_accepted"}))?)),
                            Err(e) => Err(e),
                        }
                    }
                }
            }
            return Err(anyhow::anyhow!("not a matching whitelist address"));
        } else {
            return Err(anyhow::anyhow!("not a matching url path"));
        }
    } else {
        return Err(anyhow::anyhow!("not an HttpServerRequest"));
    }
}

call_init!(init);
fn init(our: Address) {
    println!("starting ai_provider:broker");
    let mut state: State = match State::load() {
        Ok(s) => s,
        Err(e) => {
            println!("error loading state: {:?}", e);
            State::default()
        }
    };

    let _ = bind_http_path("/generate_image", false, false);

    loop {
        if let Err(e) = handle_message(&our, &mut state) {
            println!("error: {:?}", e);
        }
        state.save().unwrap();
    }
}

pub fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}

