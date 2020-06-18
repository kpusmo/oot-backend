use actix::{Context, Handler, Message};
use serde::{Deserialize, Serialize};

use crate::ws::session::default_id;
use crate::ws::server::GameServer;

/// Send message to specific room
#[derive(Message, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct ClientMessage {
    #[serde(default = "default_id")]
    pub id: usize,
    room: String,
    game_id: usize,
    contents: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientMessageResponse<'a> {
    sender_id: usize,
    message: &'a str,
    room: &'a str,
}

impl Handler<ClientMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!("{:?}", msg);
        if let Some(room) = self.rooms.get(&msg.room) {
            if let Some(_) = room.members.get(&msg.id) {
                let connection = self.sessions.get(&msg.id).unwrap();
                self.send_message(&msg.room, &ClientMessageResponse {
                    sender_id: connection.id,
                    message: &msg.contents,
                    room: &msg.room,
                }, msg.id);
            }
        }
    }
}
