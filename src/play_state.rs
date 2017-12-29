use std::collections::HashMap;

use id::{Id, get_id};
use zone::{Zone, ZoneKind};
use entity::Entity;
use player::Player;

#[derive(Debug)]
pub struct PlayState {
    pub zones: HashMap<Id, Zone>,
    pub entities: HashMap<Id, Entity>,
    pub players: HashMap<Id, Player>,
}

impl PlayState {
    pub fn new() -> PlayState {
        PlayState {
            zones: HashMap::new(),
            entities: HashMap::new(),
            players: HashMap::new(),
        }
    }

    pub fn default() -> PlayState {
        let mut state = PlayState::new();

        let battlefield = Zone {
            id: get_id(),
            kind: ZoneKind::Battlefield,
        };
        state.zones.insert(battlefield.id, battlefield);

        let player1 = Player {
            id: get_id(),
        };
        state.players.insert(player1.id, player1);

        let player2 = Player {
            id: get_id(),
        };
        state.players.insert(player2.id, player2);

        state
    }
}
