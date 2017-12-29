use id::Id;

#[derive(Debug)]
pub enum ZoneKind {
    Battlefield,
    Hand {
        player_id: Id,
    },
}

#[derive(Debug)]
pub struct Zone {
    pub id: Id,
    pub kind: ZoneKind,
}
