use std::collections::HashMap;

use id::Id;
use zone::Zone;
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
    pub players: HashMap<Id, Player>,

    /// The base definition of each entity in the game, before being modified by
    /// the currently active effects.
    pub entities: HashMap<Id, Entity>,

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

    // TODO: The stack, a Vec<Entity>?
}

impl PlayState {
    /// Create a version of `PlayState` as viewed by the given player. This
    /// should collapse hidden zones and unknown information. Hopefully, it's
    /// also cheap, since the most naive way to implement client communication
    /// would be to send a full state on every change!
    pub fn view_as_player(&self, _player_id: Id) -> PlayState {
        // TODO: Hide entities that player has no knowledge of, like face-down
        // permanents.

        // TODO: Hide zones that player has no knowledge of, like libraries, and
        // face-down exile zones created by cards like Bomat Courier.

        self.clone()
    }

    /// Queries an entity in the game by ID.
    ///
    /// Entity representation may be unintuitive -- the `Entity` objects stored
    /// in the `PlayState` only contain their base state. When querying an
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
