use kinode_process_lib::ProcessId;
use kinode_process_lib::Address;
use kinode_process_lib::eth::Address as EthAddress;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;

// 0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0
pub static CONTRACT_ADDRESS: EthAddress = EthAddress::new([
    0xa5,0x1c, 0x1f, 0xc2, 0xf0, 0xd1, 0xa1, 0xb8, 0x49, 0x4e, 0xd1, 0xfe, 0x31, 0x2d, 0x7c, 0x3a, 0x78, 0xed, 0x91, 0xc0, 
//    0x22, 0x79, 0xb7, 0xa0, 0xa6, 0x7d, 0xb3, 0x72, 0x99, 0x6a, 0x5f, 0xab, 0x50, 0xd9, 0x1e, 0xaa, 0x73, 0xd2, 0xeb, 0xe6,
]);

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

