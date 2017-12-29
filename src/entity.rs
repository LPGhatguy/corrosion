use id::Id;
use timestamp::Timestamp;

/// Describes an entity that exists anywhere in the game.
///
/// The game dictates that when entities move to different zones, they actually
/// become new entities (with new IDs!). That motivates the idea that the entity
/// itself should be lightweight and replacable, with most of the interesting
/// data living inside another, persistent object.
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: Id,
    pub zone: Id,
    pub timestamp: Timestamp,
}
