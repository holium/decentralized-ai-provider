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
    struct Broker {
        string brokerKnsId;
        address brokerAddress;
    }

    function getProcessBrokers(string memory processId) external view returns (Broker[] memory) {}
}

/// Get the list of applications from the AppRegistry contract.
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

    let response_bytes = match provider.call(tx_req, None) {
        Ok(response) => response,
        Err(e) => {
            println!("error: {:?}", e);
            return vec![];
        }
    };

    let brokers: Vec<Broker> = match Vec::<Broker>::abi_decode(&response_bytes, false) {
        Ok(brokers) => brokers,
        Err(e) => {
            println!("error decoding ApplicationRecord array: {:?}", e);
            return vec![];
        }
    };
    brokers
}
