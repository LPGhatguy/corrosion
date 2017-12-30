extern crate corrosion;

use std::collections::HashMap;

use corrosion::{
    Entity,
    GamePhase,
    Player,
    PlayState,
    Zone,
    ZoneKind,
};

use corrosion::{
    get_id,
    get_timestamp,
};

/// A test method for quickly bootstrapping a valid `PlayState`.
fn new_play_state() -> PlayState {
    let mut state = PlayState {
        zones: HashMap::new(),
        entities: HashMap::new(),
        players: HashMap::new(),
        current_phase: GamePhase::Main,

        // We'll mutate these before we return
        active_player: None,
        player_priority: None,
    };

    let battlefield = Zone {
        id: get_id(),
        kind: ZoneKind::Battlefield,
    };
    state.zones.insert(battlefield.id, battlefield);

    let player1 = Player {
        id: get_id(),
    };
    state.active_player = Some(player1.id);
    state.player_priority = Some(player1.id);

    let player1_hand = Zone {
        id: get_id(),
        kind: ZoneKind::Hand {
            player_id: player1.id,
        },
    };

    let in_player1_hand = Entity {
        id: get_id(),
        zone: player1_hand.id,
        timestamp: get_timestamp(),
    };

    state.entities.insert(in_player1_hand.id, in_player1_hand);

    state.players.insert(player1.id, player1);
    state.zones.insert(player1_hand.id, player1_hand);

    let player2 = Player {
        id: get_id(),
    };

    let player2_hand = Zone {
        id: get_id(),
        kind: ZoneKind::Hand {
            player_id: player2.id,
        },
    };
    state.players.insert(player2.id, player2);
    state.zones.insert(player2_hand.id, player2_hand);

    state
}

fn main() {
    let state = new_play_state();

    println!("{:?}", state);
}
