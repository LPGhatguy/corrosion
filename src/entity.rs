use id::Id;
use timestamp::Timestamp;

#[derive(Debug)]
pub struct Entity {
    pub id: Id,
    pub zone: Id,
    pub timestamp: Timestamp,
}
