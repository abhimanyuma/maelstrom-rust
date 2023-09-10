use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(rename = "type")] 
    req_type: String,
    msg_id: usize,
    node_id: Option<String>,
    node_ids: Option<Vec<String>>,
    echo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    #[serde(rename = "type")] 
    resp_type: String,
    msg_id: usize,
    in_reply_to: usize,
    echo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    dest: String,
    src: String,
    body: RequestBody,
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

        let response_body = if body.req_type == "init" {
            ResponseBody {resp_type: String::from("init_ok"), echo: None, msg_id: msg_id, in_reply_to: body.msg_id}
        } else if body.req_type == "echo" && body.echo.is_some() {
            ResponseBody {resp_type: String::from("echo_ok"), echo: body.echo.clone(), msg_id: msg_id, in_reply_to: self.body.msg_id} 
        } else {
            ResponseBody {resp_type: String::from("error"), echo: None, msg_id: msg_id, in_reply_to: self.body.msg_id}
        };

        let resp: Response = Response {src: self.dest.clone(), dest: self.src.clone(), body: response_body};
        eprintln!("{:?}", &resp);
        resp
    } 
}