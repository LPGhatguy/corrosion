use id::Id;

/// Contains additional information about a zone, like what kind of zone it is.
#[derive(Debug, Clone)]
pub enum ZoneDetails {
    Battlefield,
    Hand {
        player_id: Id,
    },
}

/// Represents a single zone in the game.
#[derive(Debug, Clone)]
pub struct Zone {
    pub id: Id,
    pub details: ZoneDetails,

    // TODO: Zone order -- should that be handled in ZoneDetails or in a field
    // of Object?
}

impl Zone {
    /// Create a version of the zone that contains only information that the
    /// given player would have.
    pub fn view_as_player(&self, _player_id: Id) -> Zone {
        // In the future, more information might be needed about the player to
        // correctly view a zone from their perspective.

        // TODO
        self.clone()
    }
}
