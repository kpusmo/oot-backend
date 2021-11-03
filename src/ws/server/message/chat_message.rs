use actix::{Context, Handler, Message};
use serde::{Deserialize, Serialize};

use crate::ws::server::GameServer;

/// Send message to a specific room
#[derive(Message, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct ChatMessage {
    #[serde(default)]
    pub session_id: usize,
    room: String,
    game_id: usize,
    contents: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChatMessageResponse<'a> {
    sender_id: usize,
    message: &'a str,
    room: &'a str,
}

impl Handler<ChatMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!("{:?}", msg);
        if let Some(connection) = self.sessions.get(&msg.session_id) {
            if connection.rooms.contains(&msg.room) {
                self.send_message(&msg.room, &ChatMessageResponse {
                    sender_id: connection.id,
                    message: &msg.contents,
                    room: &msg.room,
                }, msg.session_id);
            }
        }
    }
}
