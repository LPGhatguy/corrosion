use id::Id;

/// Describes a player actively engaged in a duel.
#[derive(Debug, Clone)]
pub struct Player {
    pub id: Id,

    // TODO: Reference to some descriptor containing name?
    // TODO: Life total
    // TODO: Counters, like energy and poison
}
