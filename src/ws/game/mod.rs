use std::collections::{HashMap, HashSet};

use actix::{Actor, Addr, Context, SpawnHandle};
use core::time::Duration;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::ws::game::message::GameMessageError;
use crate::ws::game::state::{AnswerState, AskQuestionState, ChooseAnswererState, GameState};
use crate::ws::server::GameServer;

pub mod state;
pub mod message;

#[derive(Debug)]
pub struct Game {
    id: usize,
    players: HashMap<usize, Player>,
    host: Host,
    config: GameConfig,
    state: GameState,
    server: Addr<GameServer>,
    room: String,
    timeout_spawn_handle: Option<SpawnHandle>,
}

impl Actor for Game {
    type Context = Context<Self>;
}

#[derive(Debug)]
pub struct Host {
    pub id: usize,
}

#[derive(Debug, Serialize)]
pub struct Player {
    pub session_id: usize,
    pub points: usize,
    pub chances: usize,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
struct GameConfig {
    /// answer timeout
    timeout: Duration,
    /// how many chances players have
    init_chances: usize,
    /// how many points the answerer gets for valid answer
    valid_answer_reward: usize,
}

impl GameConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::new(5, 0),
            init_chances: 3,
            valid_answer_reward: 10,
        }
    }
}

impl Game {
    pub fn new(id: usize, room_name: &str, server: Addr<GameServer>, player_ids: &HashSet<usize>, host_id: usize) -> Self {
        let config = GameConfig::default(); // todo
        let players = Game::create_players(player_ids, &config);
        Self {
            id,
            config,
            server,
            players,
            state: GameState::Starting,
            room: room_name.to_owned(),
            timeout_spawn_handle: None,
            host: Host {
                id: host_id,
            },
        }
    }

    fn create_players(player_ids: &HashSet<usize>, config: &GameConfig) -> HashMap<usize, Player> {
        let mut players = HashMap::new();
        for id in player_ids {
            players.insert(*id, Player {
                session_id: *id,
                points: 0,
                chances: config.init_chances,
                is_active: true,
            });
        }
        players
    }

    pub fn get_answerer_id(&self) -> Option<usize> {
        match &self.state {
            GameState::AskQuestion(AskQuestionState { previous_answerer_id: _, answerer_id })
            | GameState::Answer(AnswerState { previous_answerer_id: _, answerer_id, question: _ })
            | GameState::ChooseAnswerer(ChooseAnswererState { previous_answerer_id: _, chooser_id: answerer_id }) => Some(*answerer_id),
            _ => None,
        }
    }

    fn get_random_player_id(&self) -> usize {
        let player_ids: Vec<&usize> = self.players.keys().collect();
        let answerer_id = **player_ids.choose(&mut rand::thread_rng()).unwrap();
        answerer_id
    }

    fn verify_host_id(&self, host_id: usize, message_type: &str) -> Result<(), GameMessageError> {
        if self.host.id == host_id {
            Ok(())
        } else {
            Err(GameMessageError::with_state(&format!("{} message error: given user ({}) is not the host ({})", message_type, host_id, self.host.id), self.id, self.state.clone()))
        }
    }

    fn verify_answerer_id(&self, answerer_id: usize, message_type: &str) -> Result<(), GameMessageError> {
        if let Some(actual_answerer_id) = self.get_answerer_id() {
            if actual_answerer_id == answerer_id {
                Ok(())
            } else {
                Err(GameMessageError::with_state(&format!("{} message error: given user ({}) is not the answerer", message_type, answerer_id), self.id, self.state.clone()))
            }
        } else {
            Err(GameMessageError::with_state(&format!("{} message error: there is no answerer set in the game", message_type), self.id, self.state.clone()))
        }
    }
}