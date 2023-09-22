use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitRequestBody {
    #[serde(rename = "type")] 
    req_type: String,
    msg_id: usize,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequestBody {
    #[serde(rename = "type")] 
    req_type: String,
    msg_id: usize,
    echo: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestBody {
    Init(InitRequestBody),
    Echo(EchoRequestBody)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    dest: String,
    src: String,
    body: RequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitResponseBody {
    #[serde(rename = "type")] 
    resp_type: String,
    msg_id: usize,
    in_reply_to: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponseBody {
    #[serde(rename = "type")] 
    resp_type: String,
    msg_id: usize,
    in_reply_to: usize,
    echo: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseBody {
    Init(InitResponseBody),
    Echo(EchoResponseBody),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    src: String, 
    dest: String,
    body: ResponseBody
}

impl Request {
    pub fn respond(&self) -> Response {

        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let msg_id: usize = COUNTER.fetch_add(1, Ordering::Relaxed);

        let body = &self.body;

        let response_body = match body {
            RequestBody::Init(init_request) => {
                let init_response = InitResponseBody{
                    resp_type: String::from("init_ok"), 
                    msg_id: msg_id, 
                    in_reply_to: init_request.msg_id
                };
                ResponseBody::Init(init_response)
            },
            RequestBody::Echo(echo_request) => {
                let echo_response = EchoResponseBody{
                    resp_type: String::from("echo_ok"), 
                    msg_id: msg_id,
                    in_reply_to: echo_request.msg_id,
                    echo: echo_request.echo.clone(),
                };
                ResponseBody::Echo(echo_response)
            }
        };

        let resp: Response = Response {src: self.dest.clone(), dest: self.src.clone(), body: response_body};
        eprintln!("{:?}", &resp);
        resp
    } 
}