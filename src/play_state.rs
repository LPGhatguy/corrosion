use std::collections::HashMap;

use id::{Id, get_id};
use zone::{Zone, ZoneKind};
use entity::Entity;
use player::Player;

/// Represents all of the important serializable information about a game.
#[derive(Debug, Clone)]
pub struct PlayState {
    pub zones: HashMap<Id, Zone>,
    pub entities: HashMap<Id, Entity>,
    pub players: HashMap<Id, Player>,

    // TODO: A reference to an entity descriptor pool, like what cards are legal
    // in this format.

    // TODO: Player turn order, probably Vec<Id>

    // TODO: Current game phase/step

    // TODO: Current priority, probably Option<Id>?
}

impl PlayState {
    /// A test method for quickly bootstrapping a valid `PlayState`.
    ///
    /// In the future, this will be replaced with a game descriptor struct of
    /// some sort that describes the format of a game.
    pub fn default() -> PlayState {
        let mut state = PlayState {
            zones: HashMap::new(),
            entities: HashMap::new(),
            players: HashMap::new(),
        };

        let battlefield = Zone {
            id: get_id(),
            kind: ZoneKind::Battlefield,
        };
        state.zones.insert(battlefield.id, battlefield);

        let player1 = Player {
            id: get_id(),
        };

        let player1_hand = Zone {
            id: get_id(),
            kind: ZoneKind::Hand {
                player_id: player1.id,
            },
        };
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

    /// Create a version of `PlayState` as viewed by the given player.
    /// This should collapse hidden zones and unknown information.
    /// Hopefully, it's also cheap, since the most naive way to implement client
    /// communication would be to send a full state on every change!
    pub fn view_as_player(&self, _player_id: Id) -> PlayState {
        // TODO
        self.clone()
    }
}
