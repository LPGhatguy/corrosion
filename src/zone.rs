use id::Id;

/// Contains additional information about a zone, like what kind of zone it is.
#[derive(Debug, Clone)]
pub enum ZoneKind {
    Battlefield,
    Hand {
        player_id: Id,
    },
}

/// Represents a single zone in the game.
#[derive(Debug, Clone)]
pub struct Zone {
    pub id: Id,
    pub kind: ZoneKind,

    // TODO: Zone order -- should that be handled in ZoneKind or in a field
    // of Entity?
}

impl Zone {
    /// Create a version of the zone that contains only information that the
    /// given player would have.
    pub fn view_as_player(&self, player_id: Id) -> Zone {
        // In the future, more information might be needed about the player to
        // correctly view a zone from their perspective.

        // TODO
        self.clone()
    }
}
