use actix::{Context, Handler, Message};
use serde::{Deserialize, Serialize};

use crate::ws::server::{GameServer, Room};

/// Join room, create it if it does not exist
#[derive(Message, Deserialize)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct JoinRoom {
    #[serde(default)]
    pub session_id: usize,
    room: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RoomJoinedResponse<'a> {
    joined_id: usize,
    room: &'a str,
}

impl Handler<JoinRoom> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) -> Self::Result {
        if let Some(connection) = self.sessions.get_mut(&msg.session_id) {
            if connection.rooms.insert(msg.room.clone()) {
                self.add_to_room(&msg.room, msg.session_id);
                self.send_message(&msg.room, &RoomJoinedResponse {
                    joined_id: msg.session_id,
                    room: &msg.room,
                }, 0);
            }
        }
    }
}

impl GameServer {
    /// Adds given id to room, creates room if it does not exist
    fn add_to_room(&mut self, room: &str, id: usize) -> bool {
        self.rooms
            .entry(room.to_owned())
            .or_insert(Room::default())
            .members
            .insert(id)
    }
}