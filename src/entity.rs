use id::Id;
use timestamp::Timestamp;

/// Represents the various kinds of entity that exist in the game rules.
///
/// I'm not sure if this is sufficient! I think all of the variants of this enum
/// will probably need associated data, like a card/ability descriptor ID.
#[derive(Debug, Clone)]
pub enum EntityKind {
    /// Any entity backed by a unique card
    Card,
    Token,
    Emblem,
    Ability,

    // TODO: Represent copy of another entity without Box or recursive types?
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
    pub timestamp: Timestamp,
    pub kind: EntityKind,
}
