use id::get_id;
use std::collections::HashMap;
use std::ops::Deref;

pub type Id = usize;
pub type Number = i64;

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
pub struct ManaCost {
	pub generic: Number,
	pub white: Number,
	pub blue: Number,
	pub black: Number,
	pub red: Number,
	pub green: Number,
	pub colorless: Number,
}

impl Default for ManaCost {
	fn default() -> ManaCost {
		ManaCost {
			generic: 0,
			white: 0,
			blue: 0,
			black: 0,
			red: 0,
			green: 0,
			colorless: 0,
		}
	}
}

#[derive(Debug)]
pub enum Cost {
	Impossible,
	Mana(ManaCost),
}

#[derive(Debug)]
pub struct EntityDescriptor {
	pub name: String,
	pub id: Id,

	// TODO: switch to Vec<Cost>
	pub cost: Cost,
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
	// October 17, 2017 B&R announcement:
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
pub enum GameEvent {
	GainPriority(Id),
	LosePriority,
	AdvanceStep(PlayStep),
	AdvanceTurn(Id),
}

#[derive(Debug)]
pub enum PlayAction {
	PassPriority(Id),
	Concede(Id),
}

#[derive(Debug)]
pub struct Players {
	pub map: HashMap<Id, Player>,
	pub turn_order: Vec<Id>,
}

impl Players {
	pub fn new() -> Players {
		Players {
			map: HashMap::new(),
			turn_order: Vec::new(),
		}
	}

	pub fn add(&mut self, player: Player) {
		self.turn_order.push(player.id);
		self.map.insert(player.id, player);
	}
}

impl Deref for Players {
	type Target = HashMap<Id, Player>;

	fn deref(&self) -> &HashMap<Id, Player> {
		&self.map
	}
}

#[derive(Debug)]
pub struct PlayState<'a> {
	pub descriptor_set: &'a EntityDescriptorSet,
	pub zones: HashMap<Id, PlayZone>,
	pub players: Players,
	pub priority: Option<Id>,
}

impl<'a> PlayState<'a> {
	pub fn new(descriptor_set: &EntityDescriptorSet) -> PlayState {
		PlayState {
			descriptor_set,
			zones: HashMap::new(),
			players: Players::new(),
			priority: None,
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

			creating.players.add(player);
		}

		creating
	}
}