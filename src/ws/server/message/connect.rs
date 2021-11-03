use std::collections::HashSet;

use actix::{Actor, Context, Handler, Message, Recipient};
use actix::dev::{MessageResponse, ResponseChannel};
use serde::Serialize;

use crate::ws::server::{Connection, GameServer};
use crate::ws::session::message::SessionMessage;

/// New session is created
pub struct Connect {
    pub name: String,
    pub addr: Recipient<SessionMessage>,
}

impl Message for Connect {
    type Result = ConnectSessionResponse;
}

impl Handler<Connect> for GameServer {
    type Result = ConnectSessionResponse;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.session_counter += 1;
        self.sessions.insert(self.session_counter, Connection {
            addr: msg.addr,
            name: msg.name,
            id: self.session_counter,
            rooms: HashSet::new(),
        });
        ConnectSessionResponse {
            connected_id: self.session_counter
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectSessionResponse {
    pub connected_id: usize,
}

impl ConnectSessionResponse {
    pub const _MESSAGE_TYPE: &'static str = "connect";
}

impl<A, M> MessageResponse<A, M> for ConnectSessionResponse
    where
        A: Actor,
        M: Message<Result=ConnectSessionResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _ctx: &mut <A as Actor>::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
