use kinode_process_lib::{get_typed_state, set_state, ProcessId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::chain::Broker;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkerState {
    pub is_ready: bool,
    pub brokers: Vec<Broker>,
    pub contract_address: String,
    pub active_process_id: Option<ProcessId>,
    pub active_task: Option<Task>,
}

impl WorkerState {
    pub fn save(&self) -> anyhow::Result<()> {
        set_state(&serde_json::to_vec(self)?);
        Ok(())
    }

    pub fn load() -> Self {
        match get_typed_state(|bytes| Ok(serde_json::from_slice::<WorkerState>(bytes)?)) {
            Some(rs) => rs,
            None => WorkerState::default(),
        }
    }
    fn default() -> Self {
        Self {
            is_ready: false,
            brokers: vec![],
            contract_address: "0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0".to_string(),
            active_process_id: Some(ProcessId {
                process_name: "diffusion".to_string(),
                package_name: "memedeck".to_string(),
                publisher_node: "memedeck.os".to_string(),
            }),
            active_task: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    #[serde(rename = "claimed")]
    Claimed,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskParameters {
    pub process_id: String,     // diffusion.memedeck.os
    pub task_id: String,        // uuid - unique id of the task
    pub from_broker: String,    // the address of the broker
    pub from_user: String,      // the id of the user who requested the task
    pub task_parameters: Value, // parameters of the task, json
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub task_id: String,
    pub status: TaskStatus,
    pub parameters: TaskParameters,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BrokerRequests {
    NewTask(TaskParameters),
    TaskAssigned {
        worker_id: String,
        process_id: String,
        task: Task,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WorkerRequests {
    ClaimNextTask,
    TaskStarted { process_id: String, task_id: String },
    TaskComplete { process_id: String, task_id: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BrokerResponses {
    TaskStarted { process_id: String, task_id: String },
    TaskComplete { process_id: String, task_id: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserResponses {
    TaskUpdate {
        process_id: String,
        task_id: String,
        status: String,
        signature: Result<u64, String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AdminRequest {
    // worker:ai_provider:meme-deck.os {"SetWorkerProcess": { "process_id": "diffusion:memedeck:meme-deck.os" } }
    SetWorkerProcess { process_id: String },
    // worker:ai_provider:meme-deck.os {"SetContractAddress": { "address": "0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0" } }
    SetContractAddress { address: String },
    // worker:ai_provider:meme-deck.os {"SetIsReady": { "is_ready": true } }
    SetIsReady { is_ready: bool },
    GetState,
}
