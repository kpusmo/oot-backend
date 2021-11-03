use actix::{Handler, Message};

use crate::ws::session::WsSession;
use serde_json::Value;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct SessionMessage {
    pub response: String,
}

impl From<String> for SessionMessage {
    fn from(response: String) -> Self {
        SessionMessage {
            response,
        }
    }
}

impl Handler<SessionMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: SessionMessage, ctx: &mut Self::Context) -> Self::Result {
        let v: Value = serde_json::from_str(&msg.response).unwrap();
        // v["type"] = Value::String("typ".to_owned());
        // println!("{:?}", v);
        // println!("{}", serde_json::to_string(&v).unwrap());
        ctx.text(serde_json::to_string(&v).unwrap());
    }
}
