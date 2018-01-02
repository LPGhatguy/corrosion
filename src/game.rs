use std::collections::HashMap;

use entity::{Entity, EntityDetails};
use id::{Id, get_id};
use player::Player;
use timestamp::get_timestamp;
use zone::{Zone, ZoneKind};

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

/// Defines all of the actions that a player can take when they have
#[derive(Debug, Clone)]
pub enum PlayerAction {
    PassPriority,
    PlayLand {
        entity_id: Id,
    },

    // TODO: Other possible player actions

    // Some potentially interesting actions:
    // * Concede (doesn't need priority)
    // * Flip a morph card (doesn't need priority)
}

/// Will be used to define mutations to the game state. Mutations are defined as
/// objects so that effects can respond to and replace them.
#[derive(Debug, Clone)]
pub enum GameMutation {
}

/// Represents all of the important serializable information about a game.
///
/// `Game` cycles between two primary statuses:
/// * `GameStatus::Processing` -- the game is currently processing
/// * `GameStatus::NeedsPlayerAction` -- the game requires a response from the
///   player who has priority, defined by `priority_player`.
///
/// All mutation to the game will come from `PlayerAction` and `GameMutation`
/// objects, which will be filtered by effects created by the game's rules.
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
    // TODO: A list of currently active effects and their durations
}

impl Game {
    /// Process the given player action.
    pub fn do_player_action(&mut self, acting_player_id: Id, action: &PlayerAction) {
        // We're busy, no players can act right now!
        if self.current_status == GameStatus::Processing {
            return;
        }

        // Players can only act (right now) when they have priority
        match self.priority_player {
            Some(priority_id) => {
                if acting_player_id != priority_id {
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
                    .position(|&id| id == acting_player_id)
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
            PlayerAction::PlayLand { entity_id } => {
                // TODO: Players can only play lands when it's their turn
                // TODO: Players can only play lands during a main phase
                // TODO: Players can only play lands when the stack is empty

                let player_hand_id = self
                    .find_zone_id(|zone| {
                        match zone.kind {
                            ZoneKind::Hand { player_id } => player_id == acting_player_id,
                            _ => false,
                        }
                    })
                    .expect("Unable to locate player's hand!");

                let battlefield_id = self
                    .find_zone_id(|zone| {
                        match zone.kind {
                            ZoneKind::Battlefield => true,
                            _ => false,
                        }
                    })
                    .expect("Unable to locate battlefield!");

                // We need to make sure we have a land to play!
                match self.entities.get(&entity_id) {
                    Some(entity) => {
                        // Make sure it's in our player's hand
                        if entity.zone != player_hand_id {
                            return;
                        }

                        // Make sure it's a land
                        match entity.details {
                            EntityDetails::Forest => {},
                            // TODO: Other entity details, when they're introduced
                        }
                    }
                    None => return,
                }

                // We just checked to make sure this existed above!
                let entity = self.entities.remove(&entity_id).unwrap();

                // For now, let's just generate a new entity and put it on the
                // battlefield.
                let new_entity = Entity {
                    id: get_id(),
                    zone: battlefield_id,
                    timestamp: get_timestamp(),
                    details: entity.details,
                };

                // TODO: Use GameMutation instead?
                self.entities.insert(new_entity.id, new_entity);
            },
        }
    }

    /// Finds the zone that passes the given condition, if it exists.
    pub fn find_zone_id<F>(&self, predicate: F) -> Option<Id>
    where
        F: Fn(&Zone) -> bool
    {
        self.zones.values()
            .find(|zone| predicate(zone))
            .and_then(|zone| Some(zone.id))
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
