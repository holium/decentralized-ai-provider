use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use kinode_process_lib::{get_typed_state, println, set_state, Address, ProcessId};
use crate::chain::{ApplicationRecord, Broker, ProcessRecord, Worker};
use shared::{TaskId, TaskParameters, Task};

// --- State of the broker ---
// ---------------------------
type Timestamp = i64;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub task_queue: HashMap<TaskId, (Timestamp, Address, TaskParameters)>, // source, task_id, parameters
    // make waiting_workers a unique set first in first out
    pub waiting_workers: std::collections::VecDeque<Address>, // Queue of waiting workers
    waiting_workers_set: HashSet<Address>, // Set to ensure uniqueness of the waiting workers
    pub ongoing_tasks: HashMap<String, (Address, Task)>, // task_id -> source, parameters
    pub on_chain_state: OnChainState,
}

impl State {
    pub fn save(&self) -> Result<()> {
        // Attempt to serialize the current state to a JSON vector
        let serialized = match serde_json::to_vec(self).context("Failed to serialize state to JSON")
        {
            Ok(serialized) => serialized,
            Err(e) => {
                println!("Error serializing state: {:?}", e);
                return Err(anyhow::Error::from(e));
            }
        };

        // Attempt to set the state, handling potential errors
        set_state(&serialized);
        Ok(())
    }
    pub fn load() -> Result<Self> {
        // Attempt to get the typed state and handle the outcome
        match get_typed_state(|bytes| {
            // Deserialize bytes and handle deserialization errors
            bincode::deserialize::<State>(bytes)
                .map_err(|e| anyhow::Error::from(e))
                .context("Failed to deserialize state from bytes")
        }) {
            // Successfully retrieved and deserialized state
            Some(result) => Ok(result),
            // No state was found; default to a new state, but differentiate if this should be an error
            None => {
                // Depending on the application logic, consider if this should return an error
                eprintln!("No state found; using default state.");
                Ok(State::default())
            }
        }
    }

    pub fn default() -> Self {
        Self {
            task_queue: HashMap::new(),
            waiting_workers: std::collections::VecDeque::new(),
            waiting_workers_set: HashSet::new(),
            ongoing_tasks: HashMap::new(),
            on_chain_state: OnChainState::default(),
        }
    }

    pub fn set_chain_state(&mut self, state: OnChainState) {
        self.on_chain_state = state;
        self.save().unwrap();
    }

    pub fn add_task(&mut self, task_id: TaskId, source: Address, parameters: TaskParameters) {
        // create timestamp
        let timestamp = self.task_queue.len() as i64;
        self.task_queue
            .entry(task_id)
            .or_insert((timestamp, source, parameters));
        self.save().unwrap();
    }

    pub fn fetch_task(&mut self) -> Option<(String, Address, TaskParameters)> {
        if let Some((task_id, _)) = self
            .task_queue
            .iter()
            .next()
            .map(|(task_id, _)| (task_id.clone(), ()))
        {
            if let Some((_timestamp, source, parameters)) = self.task_queue.remove(&task_id) {
                self.save().unwrap();
                return Some((task_id, source, parameters));
            }
        }
        None
    }

    pub fn remove_task(&mut self, task_id: &TaskId) {
        self.task_queue.remove(task_id);
        self.save().unwrap();
    }

    pub fn start_task(&mut self, task: Task, source: Address) {
        self.ongoing_tasks
            .insert(task.clone().task_id, (source, task.clone()));
        self.save().unwrap();
    }

    // Adds a worker to the queue if it's not already present
    pub fn add_waiting_worker(&mut self, worker: &Address) {
        if !self.waiting_workers_set.contains(&worker) {
            self.waiting_workers.push_back(worker.clone());
            self.waiting_workers_set.insert(worker.clone());
        }
        self.save().unwrap();
    }

    // Removes the first worker from the queue and returns it, if any
    pub fn remove_waiting_worker(&mut self) -> Option<Address> {
        if let Some(worker) = self.waiting_workers.pop_front() {
            self.waiting_workers_set.remove(&worker);
            self.save().unwrap();
            Some(worker)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnChainState {
    pub brokers: HashMap<String, Vec<Broker>>, // length 1 for now
    pub workers: HashMap<String, Vec<Worker>>,
    pub apps: HashMap<String, ApplicationRecord>,
    pub processes: HashMap<String, ProcessRecord>,
    pub queue_response_timeout_seconds: u8,
    pub serve_timeout_seconds: u16, // TODO
    pub max_outstanding_payments: u8,
    pub payment_period_hours: u8,
}

impl OnChainState {
    pub fn set_apps(&mut self, apps: Vec<ApplicationRecord>) {
        self.apps = apps
            .into_iter()
            .map(|app| (app.clone().name, app))
            .collect();
    }

    pub fn set_processes(&mut self, processes: Vec<ProcessRecord>) {
        self.processes = processes
            .into_iter()
            .map(|process| (process.clone().name, process))
            .collect();
    }

    pub fn set_brokers(&mut self, process_id: &String, brokers: Vec<Broker>) {
        self.brokers.insert(process_id.to_string(), brokers);
    }

    pub fn set_workers(&mut self, process_id: &String, workers: Vec<Worker>) {
        self.workers.insert(process_id.to_string(), workers);
    }

    fn default() -> Self {
        Self {
            brokers: HashMap::new(),
            workers: HashMap::new(),
            apps: HashMap::new(),
            processes: HashMap::new(),
            queue_response_timeout_seconds: 10,
            serve_timeout_seconds: 10,
            max_outstanding_payments: 10,
            payment_period_hours: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateRequest {
    pub signature: String,
    pub task: Value,
    pub process_id: String,
}

// --------------------------
// ------ User Req/Res ------
// --------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRequests {
    // broker:ai_provider:meme-deck.os { "RequestTask": { "process_id": "diffusion:ai_provider:meme-deck.os", "task_parameters": { "workflow": "basic" } } }
    RequestTask {
        process_id: String,
        task_parameters: Value,
    },
    CancelTask {
        process_id: ProcessId,
        task_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserResponses {
    TaskRequested {
        process_id: ProcessId,
        task_id: String,
    },
    TaskClaimed {
        process_id: ProcessId,
        task_id: String,
        worker_id: String,
    },
    TaskUpdated {
        process_id: ProcessId,
        task_id: String,
        worker_id: String,
        data: Value,
    },
    TaskCompleted {
        process_id: ProcessId,
        task_id: String,
        worker_id: String,
        result: Value,
    },
    TaskFailed {
        process_id: ProcessId,
        task_id: String,
        error: String,
    },
    TaskCancelled {
        process_id: ProcessId,
        task_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserErrors {
    InvalidTaskParameters(String),
}

// --------------------------
// ----- Worker Req/Res -----
// --------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WorkerResponses {
    WorkerList(Vec<String>),
    Worker(String),
    Tasks(Vec<Task>),
    Task {
        process_id: String,
        task_id: String,
    },
    TaskAssigned {
        worker_id: String,
        process_id: String,
        task: Task,
    },
    TaskList {
        process_id: String,
    },
    AlreadyWaiting,
}

// ----- Admin Req/Res ------
// --------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AdminRequest {
    // broker:ai_provider:meme-deck.os {"SetWorkerProcess": { "process_id": "diffusion:ai_provider:meme-deck.os" } }
    SetWorkerProcess { process_id: String },
    // broker:ai_provider:meme-deck.os {"SetContractAddress": { "address": "0x5fbdb2315678afecb367f032d93f642f64180aa3" } }
    SetContractAddress { address: String },
    // broker:ai_provider:meme-deck.os {"SetIsReady": { "is_ready": true } }
    SetIsReady { is_ready: bool },
    // broker:ai_provider:meme-deck.os "GetState"
    GetState,
    // broker:ai_provider:meme-deck.os "SyncChainState"
    SyncChainState,
    // m our@broker:ai_provider:meme-deck.os {"RegisterBroker": {"process_id": "diffusion:ai_provider:meme-deck.os" }}
    RegisterBroker { process_id: String },
}

pub fn string_to_process_id(s: &str) -> ProcessId {
    let parts = s.split(':').collect::<Vec<&str>>();
    ProcessId {
        process_name: parts[0].to_string(),
        package_name: parts[1].to_string(),
        publisher_node: parts[2].to_string(),
    }
}

