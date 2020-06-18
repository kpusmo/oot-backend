use actix::{Context, Handler, Message};
use serde::{Deserialize, Serialize};
use crate::ws::server::game::event::{AnswererChosen, TickResult, GameTick};
use crate::ws::server::GameServer;
use crate::ws::session::default_id;

#[derive(Debug, Deserialize, Message)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct ChooseAnswerer {
    #[serde(default = "default_id")]
    pub id: usize,
    game_id: usize,
    room: String,
    new_answerer_id: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChooseAnswererResponse {
    old_answerer_id: usize,
    new_answerer_id: usize,
}

impl Handler<ChooseAnswerer> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ChooseAnswerer, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(game) = self.games.get_mut(&msg.game_id) {
            if let Some(answerer_id) = game.get_answerer_id() {
                if msg.id != answerer_id {
                    todo!("error")
                }
            } else {
                todo!("error")
            }
            match game.tick(AnswererChosen {
                answerer_id: msg.new_answerer_id,
            }) {
                TickResult::AnswererChosen(new_answerer_id) => {
                    self.send_message(&msg.room, &ChooseAnswererResponse {
                        old_answerer_id: msg.id,
                        new_answerer_id,
                    }, 0);
                },
                _ => unimplemented!(),
            }
        }
    }
}