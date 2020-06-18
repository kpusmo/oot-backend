use actix::{Context, Handler, Message};
use serde::{Deserialize, Serialize};

use crate::ws::server::GameServer;
use crate::ws::session::default_id;

/// Join room, create it if it does not exist
#[derive(Message, Deserialize)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct JoinRoom {
    #[serde(default = "default_id")]
    pub id: usize,
    room: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SomeoneJoinedResponse<'a> {
    joined_id: usize,
    room: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RoomJoinedResponse<'a> {
    room: &'a str,
}

impl Handler<JoinRoom> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) -> Self::Result {
        if let Some(connection) = self.sessions.get(&msg.id) {
            self.send_message(&msg.room, &SomeoneJoinedResponse {
                joined_id: msg.id,
                room: &msg.room,
            }, 0);
            self.add_to_room(&msg.room, msg.id);
            self.send_message_to(msg.id, &RoomJoinedResponse {
                room: &msg.room,
            });
        }
    }
}
