use std::collections::HashMap;

use id::Id;
use timestamp::Timestamp;

/// Temporary structure to encode entity details.
///
/// I'm still thinking about how entities should be structured, but I want to
/// tinker with other pieces of the codebase.
#[derive(Debug, Clone)]
pub enum EntityDetails {
    Forest {
        // This isn't relevant outside the battlefield
        tapped: bool,
    },
}

/// A super simplified view of abilities; this will need to be separated into
/// mana abilities and not, represent priority, and actually represent
/// descriptors of what these abilities do.
#[derive(Debug, Clone)]
pub enum Ability {
    AddGreen,
}

/// Describes an entity that exists anywhere in the game.
///
/// The game dictates that when entities move to different zones, they actually
/// become new entities (with new IDs!). That motivates the idea that the entity
/// itself should be lightweight and replacable, with most of the interesting
/// data living inside another, persistent object.
///
/// We'll need to represent several kinds of entity:
/// * Cards
///     * Permanents and non-permanents on the battlefield
///     * Cards in a graveyard, library, or hand
///     * Spells on the stack
/// * Tokens
///     * Unique tokens, like 2/2 zombies
///     * Copies of cards
/// * Emblems
/// * Abilities on the stack, never represented by a card
/// * Copies of spells on the stack
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: Id,
    pub zone: Id,

    /// A monotonically-increasing value describing when the entity entered the
    /// zone.
    pub timestamp: Timestamp,

    /// Temporary field denoting entity details
    pub details: EntityDetails,

    /// All of the abilities that this entity has available to activate
    pub abilities: HashMap<Id, Ability>,

    // TODO: Owner, a player ID
    // TODO: Controller, a player ID
}
