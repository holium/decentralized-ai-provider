use crate::types::{string_to_process_id, State, UserRequests, UserResponses};
use super::worker::assign_tasks_to_waiting_workers;
use kinode_process_lib::{println, Address, Message};
use shared::TaskParameters;
use uuid::Uuid;

pub fn handle_user_request(
    our: &Address,
    message: &Message,
    state: &mut State,
) -> anyhow::Result<UserResponses> {
    match serde_json::from_slice::<UserRequests>(message.body())? {
        UserRequests::RequestTask {
            process_id,
            task_parameters,
        } => {
            println!(
                "---> RequestTask: from {:?}, process {:?}",
                message.source().node(),
                process_id
            );
            let task_id = Uuid::new_v4().to_string();
            let process_id = string_to_process_id(&process_id);
            let task = TaskParameters {
                process_id: process_id.clone(),
                task_id: task_id.clone(),
                from_broker: our.to_string(),
                from_user: message.source().node().to_string(),
                task_parameters,
            };

            state.add_task(task_id.clone(), our.clone(), task);
            println!(
                "---> task: {}",
                serde_json::to_string_pretty(&state.task_queue).unwrap()
            );
            // println!("---> waiting_workers: {:?}", state.waiting_workers.len());
            // println!("---> task_queue: {:?}", state.task_queue.len());
            match assign_tasks_to_waiting_workers(state) {
                Ok(_) => Ok(UserResponses::TaskRequested {
                    process_id,
                    task_id,
                }),
                Err(e) => Err(e),
            }
        }
        UserRequests::CancelTask {
            process_id,
            task_id,
        } => {
            state.remove_task(&task_id);
            Ok(UserResponses::TaskCancelled {
                process_id,
                task_id,
            })
        }
    }
}
