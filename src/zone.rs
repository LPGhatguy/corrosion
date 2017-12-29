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
