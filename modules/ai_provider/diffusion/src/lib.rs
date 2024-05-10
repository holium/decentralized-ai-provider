use kinode_process_lib::{await_message, call_init, println, Address, set_state, get_typed_state};
use serde::{Deserialize, Serialize};
use shared::WorkerToProcessRequests;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});
call_init!(init);

#[derive(Debug, Serialize, Deserialize)]
struct State {
    current_job: Option<u64>,
    comfyui_host: String,
    comfyui_port: u16,
    comfyui_client_id: u32,
}
impl State {
    fn is_ready(&self) -> bool {
        self.comfyui_host.len() > 0 && self.comfyui_port != 0 && self.comfyui_client_id != 0
    }

    fn save(&self) -> anyhow::Result<()> {
        set_state(&serde_json::to_vec(self)?);
        Ok(())
    }

    fn load() -> Self {
        match get_typed_state(|bytes| Ok(serde_json::from_slice::<State>(bytes)?)) {
            Some(rs) => rs,
            None => State::default(),
        }
    }
}
impl Default for State {
    fn default() -> Self {
        Self {
            current_job: None,
            comfyui_host: "".into(),
            comfyui_port: 0,
            comfyui_client_id: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum AdminRequest {
    GetState,
    SetComfyUI { host: String, port: u16, client_id: u32 },
}

fn init(our: Address) {
    println!("starting diffusion:ai_provider, which needs to be setup to talk to a comfyui instance");
    let mut state: State = State::load();

    loop {
        match await_message() {
            Ok(message) => {
                if message.is_request() {
                    if message.source().node == our.node {
                        // handle AdminRequest 
                        if let Ok(req) = serde_json::from_slice::<AdminRequest>(message.body()) {
                            match req {
                                AdminRequest::GetState => println!("{}", serde_json::to_string_pretty(&state).unwrap()),
                                AdminRequest::SetComfyUI { host, port, client_id } => {
                                    state.comfyui_host = host;
                                    state.comfyui_port = port;
                                    state.comfyui_client_id = client_id;
                                    println!("updated comfyui connection details");
                                }
                            }
                        }

                        if let Ok(req) = serde_json::from_slice::<WorkerToProcessRequests>(message.body()) {
                            match req {
                                WorkerToProcessRequests::StartTask(value) => {
                                    println!("got StartTask with value: {}", serde_json::to_string_pretty(&value).unwrap());
                                    if !state.is_ready() {
                                        println!("this process is not ready to handle tasks yet. please setup comfyui details with SetComfyUI command");
                                    }
                                },
                            }
                        }
                    } else {
                        println!("got a message from foreign node {}, which we will ignore.", message.source().node.to_string());
                    }
                }
            },
            Err(e) => println!("error: {:?}", e),
        };
        match state.save() {
            Ok(_) => (),
            Err(e) => println!("error saving state: {e:?}"),
        }
    }
}
