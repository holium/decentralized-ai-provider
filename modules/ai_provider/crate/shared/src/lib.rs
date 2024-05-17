use kinode_process_lib::ProcessId;
use kinode_process_lib::Address;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProcessToWorkerRequests {
    TaskUpdate { task_id: String }, // comes with blob_bytes
    TaskComplete {
        task_id: String,
        process_id: String,
        broker: Address
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WorkerToProcessRequests {
    // the message that this worker:ai_provider process sends to whatever
    // active_process is currently assigned. (diffusion:ai_provider for now)
    StartTask {
        task_id: TaskId,
        params: Value,
        process_id: String,
        broker: Address,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WorkerToBrokerRequests {
    ClaimNextTask,
    TaskStarted { process_id: String, task_id: String },
    TaskComplete { process_id: String, task_id: String },
}

// -- Task definitions
pub type TaskId = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub task_id: String,
    pub status: TaskStatus,
    pub parameters: TaskParameters,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskParameters {
    pub process_id: ProcessId,  // diffusion.memedeck.os
    pub task_id: String,        // uuid - unique id of the task
    pub from_broker: String,    // the address of the broker
    pub from_user: String,      // the id of the user who requested the task
    pub task_parameters: Value, // parameters of the task, json
}

impl TaskParameters {
    pub fn test_task_parameters() -> Self {
        Self {
            process_id: ProcessId {
                process_name: "diffusion".to_string(),
                package_name: "ai_provider".to_string(),
                publisher_node: "meme-deck.os".to_string(),
            },
            task_id: "123".into(),
            from_broker: "memedeck-broker-1.dev".into(),
            from_user: "memedeck-client-1.dev".into(),
            task_parameters: json!({
                "nodes": {}
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    #[serde(rename = "claimed")]
    Claimed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

