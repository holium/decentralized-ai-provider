/// This file contains the logic for interacting with the AppRegistry contract.
use alloy_sol_types::{sol, SolCall, SolValue};
use kinode_process_lib::{
    eth::{Address as EthAddress, Provider, TransactionInput, TransactionRequest, U64},
    println,
};
use serde::{Deserialize, Serialize};

// 0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0
static CONTRACT_ADDRESS: EthAddress = EthAddress::new([
//    0xa5,0x1c, 0x1f, 0xc2, 0xf0, 0xd1, 0xa1, 0xb8, 0x49, 0x4e, 0xd1, 0xfe, 0x31, 0x2d, 0x7c, 0x3a, 0x78, 0xed, 0x91, 0xc0, 
    0x22, 0x79, 0xb7, 0xa0, 0xa6, 0x7d, 0xb3, 0x72, 0x99, 0x6a, 0x5f, 0xab, 0x50, 0xd9, 0x1e, 0xaa, 0x73, 0xd2, 0xeb, 0xe6,
]);

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
