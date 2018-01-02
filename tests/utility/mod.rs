use std::collections::HashMap;

use corrosion::{
    Game,
    GameStatus,
    GamePhase,
    Player,
    Zone,
    ZoneKind,
};

use corrosion::{
    get_id,
};

/// A test method for quickly bootstrapping a valid two-player `Game`.
pub fn new_two_player_game() -> Game {
    let mut game = Game {
        zones: HashMap::new(),
        entities: HashMap::new(),
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
        kind: ZoneKind::Battlefield,
    };
    game.zones.insert(battlefield.id, battlefield);

    let player1 = Player {
        id: get_id(),
    };
    game.active_player = Some(player1.id);
    game.priority_player = Some(player1.id);

    let player1_hand = Zone {
        id: get_id(),
        kind: ZoneKind::Hand {
            player_id: player1.id,
        },
    };

    game.player_turn_order.push(player1.id);
    game.players.insert(player1.id, player1);
    game.zones.insert(player1_hand.id, player1_hand);

    let player2 = Player {
        id: get_id(),
    };

    let player2_hand = Zone {
        id: get_id(),
        kind: ZoneKind::Hand {
            player_id: player2.id,
        },
    };

    game.player_turn_order.push(player2.id);
    game.players.insert(player2.id, player2);
    game.zones.insert(player2_hand.id, player2_hand);

    game
}
