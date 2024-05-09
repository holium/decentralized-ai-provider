/// This file contains the logic for interacting with the AppRegistry contract.
use alloy_sol_types::{sol, SolCall, SolValue};
use kinode_process_lib::{
    eth::{Address as EthAddress, Provider, TransactionInput, TransactionRequest, U64},
    println,
};
use serde::{Deserialize, Serialize};

// 0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0
static CONTRACT_ADDRESS: EthAddress = EthAddress::new([
    0xa5,0x1c, 0x1f, 0xc2, 0xf0, 0xd1, 0xa1, 0xb8, 0x49, 0x4e, 0xd1, 0xfe, 0x31, 0x2d, 0x7c, 0x3a, 0x78, 0xed, 0x91, 0xc0, 
]);

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct ApplicationRecord {
        string name;
        address governanceToken;
        address usageToken;
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
    function registerBroker(string calldata processId, string calldata brokerKnsId) external;
}

pub fn register_broker(our: &str, process_id: String) -> Result<(),()> {
    let provider = Provider::new(31337, 5);

    let input = registerBrokerCall {
        processId: process_id,//: String::from("diffusion:memedeck:memedeck.os"),
        brokerKnsId: our.into(),//"memedeck:memedeck.os".into(),
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
