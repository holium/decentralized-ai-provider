use kinode_process_lib::{println, Address, Message, ProcessId, Request};
use crate::types::{State, WorkerResponses,};
use shared::{Task, TaskId, TaskParameters, TaskStatus, WorkerToBrokerRequests};

pub fn handle_worker_request(
    _our: &Address,
    message: &Message,
    state: &mut State,
) -> anyhow::Result<()> {
    let source = message.source();
    match serde_json::from_slice::<WorkerToBrokerRequests>(message.body())? {
        WorkerToBrokerRequests::ClaimNextTask => {
            println!("---> ClaimNextTask: {:?}", message.source().node());
            println!("---> waiting_workers: {:?}", state.waiting_workers.len());
            println!("---> task_queue: {:?}", state.task_queue.len());
            if let Some(task) = state.fetch_task() {
                let process_workers = match state
                    .on_chain_state
                    .workers
                    .get(&task.2.process_id.to_string())
                {
                    Some(workers) => workers.to_owned(),
                    None => vec![],
                };

                for worker in process_workers.iter() {
                    match send_task_to_worker(&task, worker.workerKnsId.clone()) {
                        Ok(_) => {
                            println!(
                                "---> TaskAssigned: worker {:?} process {:?} task {:?}",
                                worker.workerKnsId,
                                task.2.clone().process_id,
                                task.1
                            );
                            continue;
                        }
                        Err(e) => {
                            println!("---> Error: {:?}", e);
                            continue;
                        }
                    }
                }
                let task_id: TaskId = task.0.clone();
                // remove task from task queue
                state.remove_task(&task_id);

                // set as ongoing
                state.start_task(
                    Task {
                        task_id: task_id.clone(),
                        status: TaskStatus::Claimed,
                        parameters: task.2,
                    },
                    source.clone(),
                );
            } else {
                // No tasks available
                println!("No tasks available, add to waiting workers");
                if state.waiting_workers.contains(&source) {
                    Request::to(message.source())
                        .body(serde_json::to_vec(&WorkerResponses::AlreadyWaiting)?)
                        .send()?;
                } else {
                    state.add_waiting_worker(&source);
                }
            }
        }
        WorkerToBrokerRequests::TaskStarted { task_id, .. } => {
            println!("---> TaskStarted: {:?}", task_id);
            // update task status to running
            if let Some((_, task)) = state.ongoing_tasks.get_mut(&task_id) {
                task.status = TaskStatus::Running;
            }
        }
        WorkerToBrokerRequests::TaskComplete { task_id, .. } => {
            println!("---> TaskComplete: {:?}", task_id);
            state.ongoing_tasks.remove(&task_id);
        }
    }
    Ok(())
}

pub fn assign_tasks_to_waiting_workers(state: &mut State) -> anyhow::Result<()> {
    if state.task_queue.is_empty() || state.waiting_workers.is_empty() {
        println!("warning: tried to assign_tasks_to_waiting_workers but queue is empty or there are no waiting workers");
        return Ok(());
    }

    while let Some(task) = state.fetch_task() {
        if let Some(worker) = state.remove_waiting_worker() {
            match send_task_to_worker(&task, worker.node().to_string()) {
                Ok(_) => {
                    println!(
                        "---> TaskAssigned: worker {:?} process {:?} task {:?}",
                        worker.node(),
                        task.2.clone().process_id,
                        task.1
                    );
                }
                Err(e) => {
                    println!("---> Error: {:?}", e);
                }
            }
        }
    }
    Ok(())
}

pub fn send_task_to_worker(
    task: &(String, Address, TaskParameters),
    worker_id: String,
) -> anyhow::Result<()> {
    let worker_address = Address {
        node: worker_id.clone(),
        process: ProcessId {
            process_name: "worker".into(),
            package_name: "ai_provider".into(),
            publisher_node: "meme-deck.os".into(),
        },
    };
    Request::to(&worker_address)
        .body(serde_json::to_vec(&WorkerResponses::TaskAssigned {
            worker_id: worker_id.to_string(),
            process_id: task.2.clone().process_id.to_string(),
            task: Task {
                task_id: task.0.clone(),
                status: TaskStatus::Claimed,
                parameters: task.2.clone(),
            },
        })?)
        .send()
}
