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
    counter: usize,
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
            counter: 0,
            game_counter: 0,
            games: HashMap::new(),
        }
    }
}

impl GameServer {
    /// Send message to all users in the room
    fn send_message<T: Serialize>(&self, room: &str, message: &T, skip_id: usize) {
        if let Some(room) = self.rooms.get(room) {
            for id in &room.members {
                if *id != skip_id {
                    if let Some(connection) = self.sessions.get(id) {
                        let msg = SessionMessage::from(serde_json::to_string(&ServerMessage::success(message)).unwrap());
                        let _ = connection.addr.do_send(msg);
                    }
                }
            }
        }
    }

    fn send_message_to<T: Serialize>(&self, id: usize, message: &T) {
        if let Some(connection) = self.sessions.get(&id) {
            let msg = SessionMessage::from(serde_json::to_string(&ServerMessage::success(message)).unwrap());
            let _ = connection.addr.do_send(msg);
        }
    }

    /// Adds given id to room, creates room if it does not exist
    fn add_to_room(&mut self, room: &str, id: usize) {
        self.rooms
            .entry(room.to_owned())
            .or_insert(Room::default())
            .members
            .insert(id);
    }
}