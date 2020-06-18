use actix::Message;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all= "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
#[serde(rename_all= "snake_case")]
pub struct ServerMessage<T>
    where
        T: Serialize {
    pub status: ResponseStatus,
    pub data: T,
}

impl<T: Serialize> ServerMessage<T> {
    pub fn success(data: T) -> Self {
        ServerMessage {
            data,
            status: ResponseStatus::Success,
        }
    }

    pub fn failure(data: T) -> Self {
        ServerMessage {
            data,
            status: ResponseStatus::Failure,
        }
    }
}