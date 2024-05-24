use kinode_process_lib::{
    await_message, call_init, println, set_state, get_typed_state, get_blob,
    Address, Message, Request, ProcessId, //Response, 
    http//, vfs,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use shared::WorkerToProcessRequests;
use shared::ProcessToWorkerRequests;
use serde_json::Value;
use http_auth_basic::Credentials;

wit_bindgen::generate!({
    path: "target/wit",
    world: "process",
});
call_init!(init);

#[derive(Debug, Serialize, Deserialize)]
struct State {
    current_job: Option<u64>,
    comfyui_scheme: String,
    comfyui_host: String,
    comfyui_port: u16,
    comfyui_client_id: u32,
    is_working: bool,
}
impl State {
    fn is_ready(&self) -> bool {
        !self.is_working && self.comfyui_host.len() > 0 && self.comfyui_port != 0 && self.comfyui_client_id != 0
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
            comfyui_scheme: "https".into(),
            comfyui_host: "".into(),
            comfyui_port: 0,
            comfyui_client_id: 0,
            is_working: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum AdminRequest {
    GetState,
    SetComfyUI { host: String, port: u16, client_id: u32, scheme: Option<String> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    inputs: HashMap<String, Value>,
    class_type: String,
    _meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerateImageTask {
    client_id: String,
    nodes: HashMap<String, Node>,
    ws_id: String, // the id of the websocket actor for callbacks
    final_node_id: String,
    user_config: Value,
    dimensions: (i64, i64),
    prompt: String,
    meme_id: String,
    panel_id: String,
    callback_url: Option<String>, // url to send blurry-partially-generated images back along
}

fn init(our: Address) {
    println!("starting diffusion:ai_provider, which needs to be setup to talk to a comfyui instance");
    let mut state: State = State::load();

    loop {
        let message = await_message();
        let Ok(message) = message else {
            println!("{}: error: {:?}", our.process(), message);
            continue;
        };
        match handle_message(&our, &mut state, &message) {
            Err(e) => println!("{e}"),
            Ok(_) => (),
        };
        match state.save() {
            Ok(_) => (),
            Err(e) => println!("error saving state: {e:?}"),
        }
    }
}

fn handle_message(
    our: &Address,
    state: &mut State,
    message: &Message,
) -> anyhow::Result<()> {
    if message.is_request() {
        if message.source().node == our.node {
            // handle AdminRequest 
            if let Ok(req) = serde_json::from_slice::<AdminRequest>(message.body()) {
                match req {
                    AdminRequest::GetState => println!("{}", serde_json::to_string_pretty(&state).unwrap()),
                    AdminRequest::SetComfyUI { host, port, client_id, scheme } => {
                        match scheme {
                            None => (),
                            Some(s) => state.comfyui_scheme = s,
                        };
                        state.comfyui_host = host;
                        state.comfyui_port = port;
                        if client_id == 0 {
                            println!("warning: setting client_id to 0 is invalid");
                        }
                        state.comfyui_client_id = client_id;
                        println!("updated comfyui connection details");
                    }
                }
                return Ok(());
            }

            // handle WorkerToProcessRequest
            if let Ok(req) = serde_json::from_slice::<WorkerToProcessRequests>(message.body()) {
                return match req {
                    WorkerToProcessRequests::StartTask { task_id, params, process_id, broker, } => {
                        println!("got StartTask with value: {}", serde_json::to_string(&params).unwrap());
                        if !state.is_ready() {
                            println!("this process is not ready to handle tasks yet. please setup comfyui details with SetComfyUI command");
                            Err(anyhow::anyhow!("missing comfyui setup"))
                        } else {
                            let task_details: GenerateImageTask = serde_json::from_value(params).unwrap();
                            serve_job(
                                our,
                                &message,
                                state,
                                message.source().process.clone(),
                                task_details,
                                task_id,
                                &process_id,
                                broker,
                            )
                        }
                    },
                }
            }

            Err(anyhow::anyhow!("unkown message"))
        } else {
            println!("got a message from foreign node {}, which we will ignore.", message.source().node.to_string());
            Err(anyhow::anyhow!("unknown message"))
        }
    } else {
        Err(anyhow::anyhow!("not a request"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ComfyUpdate {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ComfyUpdateExecuting {
    data: ComfyUpdateExecutingData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ComfyUpdateExecutingData {
    prompt_id: Option<String>,
    node: Option<String>,
    output: Option<Value>,
}

fn serve_job(
    our: &Address,
    _message: &Message,
    state: &mut State,
    source_process: ProcessId,
    task_details: GenerateImageTask,
    task_id: String,
    process_id: &str,
    broker: Address,
) -> anyhow::Result<()> {
    state.is_working = true;

    // connect to comfyui via WS
    let url = format!(
        "{}://{}/ws?clientId={}",
        if state.comfyui_scheme == "https" { "wss" } else { "ws" },
        state.comfyui_host,
        state.comfyui_client_id
    );
    let parsed_url = url::Url::parse(&url)?;
    let auth_header = Credentials::new(parsed_url.username(), parsed_url.password().unwrap());
    let auth_header = auth_header.as_http_header();
    let mut ws_headers = HashMap::new();
    ws_headers.insert("Authorization".to_string(), auth_header);
    http::open_ws_connection(
        url,
        Some(ws_headers),
        state.comfyui_client_id.clone(),
    )?;

    // queue prompt
    let url = format!("{}://{}:{}/prompt", state.comfyui_scheme, state.comfyui_host, state.comfyui_port);
    let url = url::Url::parse(&url)?;
    let prompt = serde_json::json!({
        "prompt": task_details.nodes,
        "client_id": format!("{}", state.comfyui_client_id),
    });
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    let queue_response = http::send_request_await_response(
        http::Method::POST,
        url,
        Some(headers),
        5,
        serde_json::to_vec(&prompt)?,
    )?;
    if !queue_response.status().is_success() {
        if let Ok(s) = String::from_utf8(queue_response.body().clone()) {
            return Err(anyhow::anyhow!("couldn't queue: {s}"));
        };
        return Err(anyhow::anyhow!("couldn't queue"));
    }
    let queue_response: Value = serde_json::from_slice(&queue_response.body())?;
    let Value::String(prompt_id) = queue_response["prompt_id"].clone() else {
        panic!("got unexpected response from comfyui");
    };

    let address = Address::new(our.node.clone(), source_process);
    let mut current_node = String::new();
    let mut progress = 0;
    loop {
        let message = await_message()?;
        let result = handle_message(&our, state, &message);
        let source = message.source();
        if result.is_ok()
        || !message.is_request()
        || source != &Address::new(our.node(), ("http_client", "distro", "sys")) {
            continue;
        }
        match serde_json::from_slice(message.body())? {
            http::HttpClientRequest::WebSocketPush { channel_id: _, message_type } => {
                if message_type == http::WsMessageType::Text {
                    let blob_bytes = &get_blob().unwrap().bytes;
                    let update: ComfyUpdate = serde_json::from_slice(&blob_bytes)?;
                    if update.type_ == "executing" {
                        println!("executing: {}", String::from_utf8_lossy(&blob_bytes));
                        let update: ComfyUpdateExecuting = serde_json::from_slice(&blob_bytes)?;
                        if update.data.prompt_id.unwrap_or("".into()) == prompt_id {
                            if update.data.node.is_none() {
                                break;
                            } else {
                                current_node = update.data.node.unwrap();
                            }
                        }
                    } else if update.type_ == "status" {
                        println!("status: {}", String::from_utf8_lossy(&blob_bytes));
                    } else {
                        println!("unknown type: {}", String::from_utf8_lossy(&blob_bytes));
                    }
                } else if message_type == http::WsMessageType::Binary {
                    println!("progress: {progress}");
                    progress += 1;

                    let is_final = current_node == task_details.final_node_id;
                    let blob_bytes = &get_blob().unwrap().bytes;

                    // if the original caller provided a callback url, send them the updates as
                    // they are generated
                    if let Some(url) = task_details.callback_url.clone() {
                        let url = url::Url::parse(&url)?;
                        let progress_payload = serde_json::json!({
                            "ws_id": task_details.ws_id.clone(),
                            "kind": if is_final {
                                "image_generated"
                            } else {
                                "image_generating"
                            },
                            "data": blob_bytes[8..].to_vec(),
                            "prompt_config": task_details.user_config.clone(),
                            "progress": if is_final {
                                100
                            } else {
                                progress
                            },
                            "dimensions": task_details.dimensions,
                            "prompt": task_details.prompt,
                            "meme_id": task_details.meme_id,
                            "panel_id": task_details.panel_id,
                        });
                        let mut headers = HashMap::new();
                        headers.insert("Content-Type".to_string(), "application/json".to_string());
                        http::send_request(
                            http::Method::POST,
                            url,
                            Some(headers),
                            None,
                            serde_json::to_vec(&progress_payload)?,
                        );
                    }
                    // tell the worker process that we have made progress
                    Request::to(address.clone())
                        .body(serde_json::to_vec(&ProcessToWorkerRequests::TaskUpdate { task_id: task_id.clone() })?)
                        .blob_bytes(blob_bytes[8..].to_vec())
                        .send()?;
                    if is_final {
                        break;
                    }
                }
            }

            http::HttpClientRequest::WebSocketClose { channel_id: _ } => {
                println!("got ws close");
            }
        }
    }

    http::close_ws_connection(state.comfyui_client_id.clone())?;

    Request::to(address)
        .body(serde_json::to_vec(&ProcessToWorkerRequests::TaskComplete {
            task_id,
            process_id: process_id.into(),
            broker
        })?)
        .send()?;
    println!("sending done to worker process");
    state.is_working = false;
    Ok(())
}

