use crate::chain;
use crate::types::{AdminRequest, WorkerState};
use kinode_process_lib::{println, Address, Message, ProcessId, Request};
use shared::WorkerToBrokerRequests;

pub fn handle_admin_request(
    message: &Message,
    state: &mut WorkerState,
) -> anyhow::Result<(), anyhow::Error> {
    match serde_json::from_slice::<AdminRequest>(message.body()) {
        Ok(AdminRequest::GetState) => {
            println!("{}", serde_json::to_string_pretty(&state).unwrap());
            Ok(())
        },
        Ok(AdminRequest::SetIsReady { is_ready }) => {
            println!("---> SetIsReady {{ is_ready : {:?} }}", is_ready);
            if state.brokers.is_empty() && state.active_process_id.is_some() {
                let process_id = state.active_process_id.as_ref().unwrap();
                state.brokers = chain::get_brokers(&process_id.to_string());
            }

            println!("---> brokers: {:?}", state.brokers);

            // fetch broker from chain
            let first_broker = state.brokers.iter().next().unwrap();
            let address = Address::new(
                first_broker.brokerKnsId.clone(),
                ProcessId {
                    process_name: "broker".into(),
                    package_name: "ai_provider".into(),
                    publisher_node: "meme-deck.os".into(),
                },
            );
            match Request::to(address)
                .body(serde_json::to_vec(&WorkerToBrokerRequests::ClaimNextTask)?)
                .send()
            {
                Ok(_) => state.save(),
                Err(e) => {
                    println!("Error sending request: {:?}", e);
                    Err(anyhow::anyhow!("Error sending request: {:?}", e)) // This path already returns an Err
                }
            }
        }
        Ok(AdminRequest::SetContractAddress { address }) => {
            println!("---> SetContractAddress {{ address : {:?} }}", address);
            state.contract_address = address;
            // Get brokers via the new contract address
            if state.active_process_id.is_some() {
                let process_id = state.active_process_id.as_ref().unwrap();
                state.brokers = chain::get_brokers(&process_id.to_string());
            }
            state.save()
        }
        Ok(AdminRequest::SetWorkerProcess { process_id }) => {
            println!("---> SetWorkerProcess {{ process_id : {:?} }}", process_id);
            let split = process_id.split(":").collect::<Vec<&str>>();
            let process_name = split[0];
            let package_name = split[1];
            let publisher_node = split[2];
            state.active_process_id = Some(ProcessId::new(
                Some(process_name),
                package_name,
                publisher_node,
            ));
            // Get brokers for the new process
            if state.active_process_id.is_some() {
                let process_id = state.active_process_id.as_ref().unwrap();
                state.brokers = chain::get_brokers(&process_id.to_string());
            }
            state.save()
        }
        _ => {
            println!("Unknown admin request");
            // Handle unknown requests by returning an appropriate error
            Err(anyhow::anyhow!("Unknown admin request"))
        }
    }
}
