use std::error::Error;
use std::time::{Duration, Instant};

use actix::{Actor, Addr, ContextFutureSpawner, fut, Running, StreamHandler, WrapFuture, AsyncContext, ActorContext, ActorFuture};
use actix_web_actors::ws;

use crate::ws::error::WsError;
use crate::ws::server::GameServer;
use crate::ws::server::message::connect::Connect;
use crate::ws::server::message::disconnect::Disconnect;
use crate::ws::server::message::ServerMessage;
use crate::ws::session::message::SessionMessage;
use crate::ws::server::message::chat_message::ChatMessage;
use crate::ws::server::message::join::JoinRoom;
use crate::ws::server::message::game_message::create_game::CreateGame;
use crate::ws::server::message::game_message::ask_question::AskQuestion;
use crate::ws::server::message::game_message::answer::AnswerQuestion;
use crate::ws::server::message::game_message::validate_answer::ValidateAnswer;
use crate::ws::server::message::game_message::choose_answerer::ChooseAnswerer;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub mod message;

pub struct WsSession {
    pub id: usize,
    pub name: String,
    pub server: Addr<GameServer>,
    pub hb: Instant,
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) <= CLIENT_TIMEOUT {
                ctx.ping(b"");
                return;
            }
            // heartbeat timed out
            println!("Websocket Client heartbeat failed, disconnecting!");

            // notify server
            act.server.do_send(Disconnect { session_id: act.id });

            // stop actor
            ctx.stop();
        });
    }

    fn handle_client_message(&self, encoded_msg: String) -> Result<(), Box<dyn Error>> {
        let chunks: Vec<&str> = encoded_msg.splitn(2, ' ').collect();
        match chunks[0] {
            "message" => {
                let mut msg = serde_json::from_str::<ChatMessage>(chunks[1])?;
                msg.session_id = self.id;
                self.server.do_send(msg);
            },
            "join" => {
                let mut msg = serde_json::from_str::<JoinRoom>(chunks[1])?;
                msg.session_id = self.id;
                self.server.do_send(msg);
            },
            "create_game" => {
                let mut msg = serde_json::from_str::<CreateGame>(chunks[1])?;
                msg.session_id = self.id;
                self.server.do_send(msg);
            },
            "ask_question" => {
                let mut msg = serde_json::from_str::<AskQuestion>(chunks[1])?;
                msg.session_id = self.id;
                self.server.do_send(msg);
            },
            "answer" => {
                let mut msg = serde_json::from_str::<AnswerQuestion>(chunks[1])?;
                msg.session_id = self.id;
                self.server.do_send(msg);
            },
            "validate_answer" => {
                let mut msg = serde_json::from_str::<ValidateAnswer>(chunks[1])?;
                msg.session_id = self.id;
                self.server.do_send(msg);
            },
            "choose_answerer" => {
                let mut msg = serde_json::from_str::<ChooseAnswerer>(chunks[1])?;
                msg.session_id = self.id;
                self.server.do_send(msg);
            },
            _ => return Err(WsError::boxed(&format!("communication error - unknown command {}", chunks[0]))),
        }
        Ok(())
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with GameServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        let addr = ctx.address().recipient::<SessionMessage>();
        self.server
            .send(Connect {
                addr,
                name: self.name.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(response) => {
                        act.id = response.connected_id;
                        ctx.text(serde_json::to_string(&ServerMessage::success(response)).unwrap());
                    }
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify server
        self.server.do_send(Disconnect { session_id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(msg)) => {
                println!("received {}", msg);
                if let Err(e) = self.handle_client_message(msg) {
                    ctx.text(format!("error: {}", e));
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}