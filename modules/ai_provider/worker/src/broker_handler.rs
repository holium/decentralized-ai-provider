use crate::types::{BrokerRequests, BrokerResponses, WorkerState};
use shared::WorkerToProcessRequests;
use shared::WorkerToBrokerRequests;
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

                Request::new()
                    .target(message.source())
                    .body(serde_json::to_vec(&WorkerToBrokerRequests::TaskStarted {
                        process_id: process_id.to_string(),
                        task_id: task.clone().task_id,
                    })?)
                    .send()?;
                println!("we told the broker that we started the task");

                // TODO: tell the "active_process" running on our own node, to do the task that we
                // just recieved from the broker
                Request::new()
                    .target((our.node(), state.active_process_id.clone().unwrap()))
                    .body(serde_json::to_vec(&WorkerToProcessRequests::StartTask(task.parameters.task_parameters.clone()))?)
                    .send()?;

                Request::new()
                    .target(message.source())
                    .body(serde_json::to_vec(&WorkerToBrokerRequests::TaskComplete {
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
