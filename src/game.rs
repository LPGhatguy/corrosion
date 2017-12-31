use std::collections::HashMap;

use id::Id;
use zone::Zone;
use entity::Entity;
use player::Player;

/// Represents the game's current phase
#[derive(Debug, Clone, PartialEq)]
pub enum GamePhase {
    Main,

    // TODO: Other game phases
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameStatus {
    Processing,
    NeedsPlayerAction,

    // TODO: GameEnded? Need to represent win/draw and potentially error
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    PassPriority,

    // TODO: Other possible player actions
}

/// Represents all of the important serializable information about a game.
///
/// `Game` cycles between two primary statuses:
/// * `GameStatus::Processing` -- the game is currently processing
/// * `GameStatus::NeedsPlayerAction` -- the game requires a response from the
///   player who has priority, defined by `priority_player`.
#[derive(Debug, Clone)]
pub struct Game {
    pub zones: HashMap<Id, Zone>,
    pub players: HashMap<Id, Player>,

    /// The base definition of each entity in the game, before being modified by
    /// the currently active effects.
    pub entities: HashMap<Id, Entity>,

    /// The order that players have their turns in.
    pub player_turn_order: Vec<Id>,

    /// The player whose turn it is right now.
    ///
    /// I'm not sure if there's a game state in which there is no active player,
    /// but it simplifies initialization of `Game`!
    pub active_player: Option<Id>,

    /// The player who currently has priority.
    ///
    /// In certain parts of the game, like resolution of state-based-actions and
    /// the untap step, no players have priority.
    pub priority_player: Option<Id>,

    /// The current phase of the game, as defined in the game rules.
    pub current_phase: GamePhase,

    /// The current game status, defining the core interaction loop.
    pub current_status: GameStatus,

    // TODO: A reference to an entity descriptor pool, like what cards are legal
    //       in this format.
    // TODO: The stack, a Vec<Entity>?
    // TODO: A log of player actions
}

impl Game {
    /// Process the given player action.
    pub fn do_player_action(&mut self, player_id: Id, action: &PlayerAction) {
        // We're busy, no players can act right now!
        if self.current_status == GameStatus::Processing {
            return;
        }

        // Players can only act when they have priority
        match self.priority_player {
            Some(priority_id) => {
                if player_id != priority_id {
                    return;
                }
            },
            None => return,
        }

        // We'll break the actual action handling into a private routine
        // eventually
        match *action {
            PlayerAction::PassPriority => {
                let current_priority_index = self.player_turn_order
                    .iter()
                    .position(|&id| id == player_id)
                    .expect("Player with priority is missing from player_turn_order!");

                let current_active_id = self.active_player.unwrap();
                let current_active_index = self.player_turn_order
                    .iter()
                    .position(|&id| id == current_active_id)
                    .expect("Player with priority is missing from player_turn_order!");

                let player_count = self.player_turn_order.len();

                let next_priority_index = (current_priority_index + 1) % player_count;
                let next_priority_id = self.player_turn_order[next_priority_index];

                // If priority would swing back around to the active player,
                // it's time to advance the turn cycle.
                if next_priority_id == current_active_id {
                    let next_active_index = (current_active_index + 1) % player_count;
                    let next_active_id = self.player_turn_order[next_active_index];

                    self.active_player = Some(next_active_id);
                    self.priority_player = Some(next_active_id);
                } else {
                    self.priority_player = Some(next_priority_id);
                }
            },
        }
    }

    /// Create a version of `Game` as viewed by the given player. This
    /// should collapse hidden zones and unknown information. Hopefully, it's
    /// also cheap, since the most naive way to implement client communication
    /// would be to send a full state on every change!
    pub fn view_as_player(&self, _player_id: Id) -> Game {
        // TODO: Hide entities that player has no knowledge of, like face-down
        //       permanents.

        // TODO: Hide zones that player has no knowledge of, like libraries, and
        //       face-down exile zones created by cards like Bomat Courier.

        self.clone()
    }

    /// Queries an entity in the game by ID.
    ///
    /// Entity representation may be unintuitive -- the `Entity` objects stored
    /// in the `Game` only contain their base state. When querying an
    /// entity's current state, we need to traverse a list of effects currently
    /// modifying that object in layer order.
    ///
    /// As an example, consider a Mountain enchanted by Spreading Seas:
    ///
    /// The definition of Mountain says that it has type `Basic Land - Mountain`
    /// and thus has `{T}: Add {R} to your mana pool`.
    ///
    /// Spreading Seas, when on the battlefield, has a replacement effect for
    /// that removes its target's abilities and changes its type line to `Basic
    /// Land - Island`.
    ///
    /// The mountain's current state (post-effect) thus says that it has type
    /// `Basic Land - Island` and taps for blue mana only!
    ///
    /// If we wanted to return the Mountain entity directly, we'd either have to
    /// modify the data whenever an effect enters/leaves the battlefield or just
    /// keep a list of active effects and calculate them on each observation.
    ///
    /// While the former technique is possible, I think that calculating effects
    /// on each individual observation event is simpler.
    pub fn view_entity(&self, entity_id: Id) -> Option<Entity> {
        let base_entity = self.entities.get(&entity_id)?;

        // TODO: Enumerate the set of effects that could affect this entity.

        Some(base_entity.clone())
    }
}
