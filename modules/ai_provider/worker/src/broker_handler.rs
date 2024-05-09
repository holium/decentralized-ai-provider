use crate::types::{BrokerRequests, BrokerResponses, WorkerRequests, WorkerState};

use anyhow::Error;
use kinode_process_lib::{println, Address, Message, Request};

pub fn handle_broker_request(
    our: &Address,
    message: &Message,
    state: &mut WorkerState,
) -> Result<(), Error> {
    match serde_json::from_slice::<BrokerRequests>(message.body())? {
        BrokerRequests::NewTask(params) => {
            let response = BrokerResponses::TaskStarted {
                process_id: params.process_id.to_string(),
                task_id: params.task_id,
            };

            let response_body = serde_json::to_string(&response)
                .map_err(|e| anyhow::Error::msg(e))?
                .into_bytes();

            let _ = match Request::to(message.source())
                .body(response_body)
                .send_and_await_response(5)
            {
                Ok(result) => match result {
                    Ok(result) => Ok(result),
                    Err(e) => Err(anyhow::anyhow!("error: {:?}", e)),
                },
                Err(e) => Err(anyhow::anyhow!("error: {:?}", e)),
            };

            Ok(())
        }
        BrokerRequests::TaskAssigned {
            worker_id,
            process_id,
            task,
        } => {
            println!(
                "---> TaskClaimed: worker {:?} process {:?}",
                worker_id, process_id
            );

            if state.active_process_id.is_none() {
                return Ok(());
            }
            if worker_id == our.node()
                && state.active_process_id.as_ref().unwrap().to_string() == process_id
            {
                // TODO: Start the task
                println!("---> Starting the task {:?}", task);
                state.active_task = Some(task.clone());

                println!("we set state.active_task");
                //TODO: this request is "missing fields" whatever that means
                Request::new()
                    .target(message.source())
                    .body(serde_json::to_vec(&WorkerRequests::TaskStarted {
                        process_id: process_id.to_string(),
                        task_id: task.clone().task_id,
                    })?)
                    .send()?;
                println!("we send a request");

                // wait for 5 seconds
                std::thread::sleep(std::time::Duration::from_secs(5));
                println!("we waited");

                Request::new()
                    .target(message.source())
                    .body(serde_json::to_vec(&WorkerRequests::TaskComplete {
                        process_id: process_id.to_string(),
                        task_id: task.clone().task_id,
                    })?)
                    .send()?;
                println!("we sent another request");
            }

            Ok(())
        }
    }
}
