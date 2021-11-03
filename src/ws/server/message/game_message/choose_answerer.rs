use actix::{ContextFutureSpawner, fut, Handler, Message, WrapFuture, ActorFuture};
use serde::{Deserialize, Serialize};

use crate::ws::game::message::{GameMessageError, GameMessageResponse};
use crate::ws::game::message::choose_answerer::ChooseAnswerer as GameChooseAnswerer;
use crate::ws::server::GameServer;

#[derive(Debug, Deserialize, Message)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct ChooseAnswerer {
    #[serde(default)]
    pub session_id: usize,
    game_id: usize,
    room: String,
    new_answerer_id: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChooseAnswererResponse {
    previous_answerer_id: usize,
    new_answerer_id: usize,
}

impl Handler<ChooseAnswerer> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ChooseAnswerer, ctx: &mut Self::Context) -> Self::Result {
        if let Some((game, _)) = self.games.get_mut(&msg.game_id) {
            game.send(GameChooseAnswerer {
                new_answerer_id: msg.new_answerer_id,
                chooser_id: msg.session_id,
            })
                .into_actor(self)
                .then(move |res, act, _ctx| {
                    match res.unwrap_or_else(|err| Err(GameMessageError::new(&format!("{:?}", err), msg.game_id))) {
                        Ok(res) => {
                            if let GameMessageResponse::AnswererChosen(new_answerer_id) = res {
                                act.send_message(&msg.room, &ChooseAnswererResponse {
                                    previous_answerer_id: msg.session_id,
                                    new_answerer_id,
                                }, 0);
                            } else {
                                act.fail_to(msg.session_id, &GameMessageError::new(&format!("expected GameMessageResult::AnswererChosen, received {:?}", res), msg.game_id));
                            }
                        }
                        Err(err) => act.fail_to(msg.session_id, &err)
                    }
                    fut::ready(())
                })
                .wait(ctx);
        } else {
            self.fail_to(msg.session_id, &GameMessageError::new("Invalid game_id", msg.game_id));
        }
    }
}