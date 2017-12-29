use std::collections::HashMap;

use id::{Id, get_id};
use zone::{Zone, ZoneKind};
use entity::Entity;
use player::Player;

/// Represents the game's current phase
#[derive(Debug, Clone)]
pub enum GamePhase {
    Main,

    // Surely there are other phases to this game, right?
    // Yeah, but they don't matter right now.
}

/// Represents all of the important serializable information about a game.
#[derive(Debug, Clone)]
pub struct PlayState {
    pub zones: HashMap<Id, Zone>,
    pub entities: HashMap<Id, Entity>,
    pub players: HashMap<Id, Player>,

    /// The player whose turn it is right now.
    ///
    /// I'm not sure if there's a game state in which there is no active player,
    /// but it simplifies initialization of `PlayState`!
    pub active_player: Option<Id>,

    /// The player who currently has priority.
    ///
    /// In certain parts of the game, like resolution of state-based-actions and
    /// the untap step, no players have priority.
    pub player_priority: Option<Id>,

    /// The current phase of the game.
    pub current_phase: GamePhase,

    // TODO: A reference to an entity descriptor pool, like what cards are legal
    // in this format.

    // TODO: Player turn order, probably Vec<Id>
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
