use std::collections::HashMap;

use actix::prelude::*;
use serde::Serialize;

pub use crate::ws::server::connection::Connection;
use crate::ws::server::game::Game;
use crate::ws::server::message::ServerMessage;
use crate::ws::server::room::Room;
use crate::ws::session::message::SessionMessage;

mod connection;
pub mod command;
mod room;
pub mod game;
pub mod message;

#[derive(Debug)]
pub struct GameServer {
    rooms: HashMap<String, Room>,
    sessions: HashMap<usize, Connection>,
    session_counter: usize,
    game_counter: usize,
    games: HashMap<usize, Game>,
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Default for GameServer {
    fn default() -> Self {
        GameServer {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
            session_counter: 0,
            game_counter: 0,
            games: HashMap::new(),
        }
    }
}

impl GameServer {
    /// Send a message to all users in the room
    /// # Arguments
    /// * `room` - name of the room to send the message to
    /// * `skip_id` - can be used to filter out a room member (e.g. when they're the one who sends the message); 0 can be used in order not to filter anyone out
    fn send_message<T: Serialize>(&self, room: &str, message: &T, skip_id: usize) {
        if let Some(room) = self.rooms.get(room) {
            let msg = SessionMessage::from(serde_json::to_string(&ServerMessage::success(message)).unwrap());
            for id in &room.members {
                if *id != skip_id {
                    if let Some(connection) = self.sessions.get(id) {
                        // todo handle result
                        let _ = connection.addr.do_send(msg.clone());
                    }
                }
            }
        }
    }

    /// Send a message to given user
    /// # Arguments
    /// * `receiver_id` - id of the session to send message to
    /// * `message` - a serializable message
    fn send_message_to<T: Serialize>(&self, receiver_id: usize, message: &T) {
        if let Some(connection) = self.sessions.get(&receiver_id) {
            let msg = SessionMessage::from(serde_json::to_string(&ServerMessage::success(message)).unwrap());
            let _ = connection.addr.do_send(msg);
        }
    }
}