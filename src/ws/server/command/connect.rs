use actix::{Context, Handler, Message, Recipient, Actor};
use serde::Serialize;

use crate::ws::server::connection::Connection;
use crate::ws::server::GameServer;
use crate::ws::server::message::ServerMessage;
use crate::ws::session::message::SessionMessage;
use actix::dev::{MessageResponse, ResponseChannel};

/// New session is created
#[derive(Message)]
#[rtype(result = "ConnectResponse")]
pub struct Connect {
    pub name: String,
    pub addr: Recipient<SessionMessage>,
}

impl Handler<Connect> for GameServer {
    type Result = ConnectResponse;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.counter += 1;
        self.sessions.insert(self.counter, Connection {
            addr: msg.addr,
            name: msg.name,
            id: self.counter,
        });
        ConnectResponse {
            connected_id: self.counter
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all= "camelCase")]
pub struct ConnectResponse {
    pub connected_id: usize,
}

impl ConnectResponse {
    pub const MESSAGE_TYPE: &'static str = "connect";
}

impl<A, M> MessageResponse<A, M> for ConnectResponse
    where
        A: Actor,
        M: Message<Result = ConnectResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}