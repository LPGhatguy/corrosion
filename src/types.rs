use id::get_id;
use std::collections::HashMap;
use std::ops::Deref;

pub type Id = usize;

#[derive(Debug)]
pub enum PlayZoneKind {
	Battlefield,
	Hand(Id)
}

#[derive(Debug)]
pub enum EntityKind {
	Card,
}

#[derive(Debug)]
pub struct EntityDescriptor {
	pub name: String,
	pub id: Id,
}

#[derive(Debug)]
pub struct EntityDescriptorSet {
	pub map: HashMap<Id, EntityDescriptor>,
}

impl EntityDescriptorSet {
	pub fn new() -> EntityDescriptorSet {
		EntityDescriptorSet {
			map: HashMap::new(),
		}
	}

	pub fn add(&mut self, descriptor: EntityDescriptor) {
		self.map.insert(descriptor.id, descriptor);
	}
}

impl Deref for EntityDescriptorSet {
	type Target = HashMap<Id, EntityDescriptor>;

	fn deref(&self) -> &HashMap<Id, EntityDescriptor> {
		&self.map
	}
}

#[derive(Debug)]
pub struct Entity {
	pub kind: EntityKind,
	pub descriptorId: Id,
}

#[derive(Debug)]
pub struct PlayZone {
	pub kind: PlayZoneKind,
	pub id: Id,
	pub entities: Vec<Entity>,
}

impl PlayZone {
	pub fn new(id: Id, kind: PlayZoneKind) -> PlayZone {
		PlayZone {
			id,
			kind,
			entities: Vec::new(),
		}
	}
}

#[derive(Debug)]
pub struct ManaPool {
	// November 4, 2019 B&R announcement:
	// Standard:
	//  * Island is banned.
	//  * Mountain is banned.
	//  * Swamp is banned.
	//  * Plains is banned.
	pub green: usize,
}

impl ManaPool {
	pub fn new() -> ManaPool {
		ManaPool {
			green: 0,
		}
	}
}

#[derive(Debug)]
pub struct Player {
	pub id: Id,
	pub mana_pool: ManaPool,
}

impl Player {
	pub fn new(id: Id) -> Player {
		Player {
			id,
			mana_pool: ManaPool::new(),
		}
	}
}

#[derive(Debug)]
pub enum PlayStep {
	Untap,
	Main1,
}

#[derive(Debug)]
pub enum PlayAction {
	GainPriority(Id),
	PassPriority(Id),
	StartResolution,
}

impl PlayAction {
	pub fn can_undo(&self) -> bool {
		match *self {
			_ => false,
		}
	}
}

#[derive(Debug)]
pub struct PlayState<'a> {
	pub descriptor_set: &'a EntityDescriptorSet,
	pub zones: HashMap<Id, PlayZone>,
	pub players: HashMap<Id, Player>,
	pub history: Vec<PlayAction>,
}

impl<'a> PlayState<'a> {
	pub fn new(descriptor_set: &EntityDescriptorSet) -> PlayState {
		PlayState {
			descriptor_set,
			zones: HashMap::new(),
			players: HashMap::new(),
			history: Vec::new(),
		}
	}

	pub fn new_default(players: Vec<Player>, descriptor_set: &'a EntityDescriptorSet) -> PlayState<'a> {
		let mut creating = PlayState::new(descriptor_set);

		let battlefield_id = get_id();
		creating.zones.insert(battlefield_id, PlayZone::new(battlefield_id, PlayZoneKind::Battlefield));

		for player in players.into_iter() {
			let hand_id = get_id();
			let hand = PlayZone::new(hand_id, PlayZoneKind::Hand(player.id));
			creating.zones.insert(hand_id, hand);

			creating.players.insert(player.id, player);
		}

		creating
	}

	pub fn undo(&mut self) -> bool {
		match self.history.last() {
			Some(action) => {
				if !action.can_undo() {
					return false;
				}
			},
			None => return false,
		}

		// TODO: actually perform undo

		return true;
	}
}