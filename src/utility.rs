/// These utilities are intended for use by the test suite.

use std::collections::HashMap;

use game::{Game, GamePhase, GameStatus};
use id::{Id, get_id};
use player::Player;
use zone::{Zone, ZoneDetails};

/// A test method for quickly bootstrapping a valid two-player `Game`.
pub fn new_two_player_game() -> Game {
    let mut game = Game {
        zones: HashMap::new(),
        objects: HashMap::new(),
        mana_pools: HashMap::new(),
        players: HashMap::new(),
        player_turn_order: Vec::new(),
        current_phase: GamePhase::Main,
        current_status: GameStatus::NeedsPlayerAction,

        // We'll mutate these before we return
        active_player: None,
        priority_player: None,
    };

    let battlefield = Zone {
        id: get_id(),
        details: ZoneDetails::Battlefield,
    };
    game.zones.insert(battlefield.id, battlefield);

    let player1 = Player {
        id: get_id(),
    };
    game.active_player = Some(player1.id);
    game.priority_player = Some(player1.id);
    game.mana_pools.insert(player1.id, 0);

    let player1_hand = Zone {
        id: get_id(),
        details: ZoneDetails::Hand {
            player_id: player1.id,
        },
    };

    game.player_turn_order.push(player1.id);
    game.players.insert(player1.id, player1);
    game.zones.insert(player1_hand.id, player1_hand);

    let player2 = Player {
        id: get_id(),
    };
    game.mana_pools.insert(player2.id, 0);

    let player2_hand = Zone {
        id: get_id(),
        details: ZoneDetails::Hand {
            player_id: player2.id,
        },
    };

    game.player_turn_order.push(player2.id);
    game.players.insert(player2.id, player2);
    game.zones.insert(player2_hand.id, player2_hand);

    game
}

pub fn get_hand_id(game: &Game, target_player_id: Id) -> Id {
    game.find_zone_id(|zone| {
            match zone.details {
                ZoneDetails::Hand { player_id } => player_id == target_player_id,
                _ => false,
            }
        })
        .unwrap()
}

pub fn get_battlefield_id(game: &Game) -> Id {
    game.find_zone_id(|zone| {
            match zone.details {
                ZoneDetails::Battlefield => true,
                _ => false,
            }
        })
        .unwrap()
}
