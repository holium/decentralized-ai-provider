/// This file contains the logic for interacting with the AppRegistry contract.
use alloy_sol_types::{sol, SolCall, SolValue};
use kinode_process_lib::{
    eth::{Provider, TransactionInput, TransactionRequest, U64},
    println,
};
use serde::{Deserialize, Serialize};
use shared::CONTRACT_ADDRESS;

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct ApplicationRecord {
        string name;
        address governanceToken;
        address usageToken;
        address[] whitelist;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ProcessRecord {
        string name;
        string repoUrl;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Broker {
        string brokerKnsId;
        address brokerAddress;
        string reachableUrl;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Worker {
        string workerKnsId;
        address workerAddress;
    }

    function getApplications() external view returns (ApplicationRecord[] memory) {}
    function getProcesses() external view returns (ProcessRecord[] memory) {}
    function getProcessBrokers(string memory processId) external view returns (Broker[] memory) {}
    function getProcessWorkers(string memory processId) external view returns (Worker[] memory) {}
    function registerBroker(string calldata processId, string calldata brokerKnsId, string calldata reachableUrl) external;
}

pub fn register_broker(our: &str, process_id: String) -> Result<(),()> {
    let provider = Provider::new(31337, 5);

    let input = registerBrokerCall {
        processId: process_id,//: String::from("diffusion:ai_provider:meme-deck.os"),
        brokerKnsId: our.into(),
        reachableUrl: String::from("http://localhost:8082"),
    };
    let tx_input = TransactionInput {
        data: Some(input.abi_encode().into()),
        ..Default::default()
    };

    let tx_req = TransactionRequest {
        chain_id: Some(U64::from(31337)),
        to: Some(CONTRACT_ADDRESS),
        input: tx_input,
        ..Default::default()
    };

    match provider.call(tx_req, None) {
        Ok(r) => {
            println!("registerBroker response {:?}", r);
            Ok(())
        },
        Err(e) => {
            println!("error: {:?}", e);
            Err(())
        }
    }
}


/// Get the list of applications from the AppRegistry contract.
pub fn get_applications() -> Vec<ApplicationRecord> {
    let provider = Provider::new(31337, 5);

    let input = getApplicationsCall {};
    let tx_input = TransactionInput {
        data: Some(input.abi_encode().into()),
        ..Default::default()
    };

    let tx_req = TransactionRequest {
        chain_id: Some(U64::from(31337)),
        to: Some(CONTRACT_ADDRESS),
        input: tx_input,
        ..Default::default()
    };

    let apps = match provider.call(tx_req, None) {
        Ok(apps) => apps,
        Err(e) => {
            println!("error: {:?}", e);
            return vec![];
        }
    };
    let apps: Vec<ApplicationRecord> = match Vec::<ApplicationRecord>::abi_decode(&apps, false) {
        Ok(apps) => apps,
        Err(e) => {
            println!("error decoding ApplicationRecord array: {:?}", e);
            return vec![];
        }
    };
    apps
}

pub fn get_processes() -> Vec<ProcessRecord> {
    let provider = Provider::new(31337, 5);

    let input = getProcessesCall {};
    let tx_input = TransactionInput {
        data: Some(input.abi_encode().into()),
        ..Default::default()
    };

    let tx_req = TransactionRequest {
        chain_id: Some(U64::from(31337)),
        to: Some(CONTRACT_ADDRESS),
        input: tx_input,
        ..Default::default()
    };

    let processes = match provider.call(tx_req, None) {
        Ok(processes) => processes,
        Err(e) => {
            println!("error: {:?}", e);
            return vec![];
        }
    };
    let processes: Vec<ProcessRecord> = match Vec::<ProcessRecord>::abi_decode(&processes, false) {
        Ok(processes) => processes,
        Err(e) => {
            println!("error decoding ProcessRecord array: {:?}", e);
            return vec![];
        }
    };
    processes
}

pub fn get_brokers(process_id: &String) -> Vec<Broker> {
    let provider = Provider::new(31337, 5);

    let input = getProcessBrokersCall {
        processId: process_id.into(),
    };
    let tx_input = TransactionInput {
        data: Some(input.abi_encode().into()),
        ..Default::default()
    };

    let tx_req = TransactionRequest {
        chain_id: Some(U64::from(31337)),
        to: Some(CONTRACT_ADDRESS),
        input: tx_input,
        ..Default::default()
    };

    let apps = match provider.call(tx_req, None) {
        Ok(apps) => apps,
        Err(e) => {
            println!("error: {:?}", e);
            return vec![];
        }
    };
    let brokers: Vec<Broker> = match Vec::<Broker>::abi_decode(&apps, false) {
        Ok(apps) => apps,
        Err(e) => {
            println!("error decoding ApplicationRecord array: {:?}", e);
            return vec![];
        }
    };
    brokers
}

pub fn get_workers(process_id: &String) -> Vec<Worker> {
    let provider = Provider::new(31337, 5);

    let input = getProcessWorkersCall {
        processId: process_id.into(),
    };
    let tx_input = TransactionInput {
        data: Some(input.abi_encode().into()),
        ..Default::default()
    };

    let tx_req = TransactionRequest {
        chain_id: Some(U64::from(31337)),
        to: Some(CONTRACT_ADDRESS),
        input: tx_input,
        ..Default::default()
    };

    let workers = match provider.call(tx_req, None) {
        Ok(workers) => workers,
        Err(e) => {
            println!("error: {:?}", e);
            return vec![];
        }
    };
    let workers: Vec<Worker> = match Vec::<Worker>::abi_decode(&workers, false) {
        Ok(workers) => workers,
        Err(e) => {
            println!("error decoding ApplicationRecord array: {:?}", e);
            return vec![];
        }
    };
    workers
}
