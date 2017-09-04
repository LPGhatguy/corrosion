use id::get_id;
use std::collections::HashMap;
use std::ops::Deref;

pub type Id = usize;
pub type Number = i64;

#[derive(Debug)]
pub enum PlayZoneKind {
	Battlefield,
	Hand(Id),
	Library(Id),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Entity {
	pub kind: EntityKind,
	pub id: Id,
	pub descriptor_id: Id,
}

impl Entity {
	pub fn clone_new(&self) -> Self {
		let mut cloned = self.clone();

		cloned.id = get_id();

		cloned
	}
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
	End,
}

#[derive(Debug)]
pub enum GameMutation {
	GainPriority(Id),
	LosePriority,
	AdvanceStep(PlayStep),
	AdvanceTurn(Id),
	AddEntity {
		zone_id: Id,
		entity: Entity,
	},
	MoveEntity {
		current_zone_id: Id,
		entity_id: Id,
		new_zone_id: Id,
	},
}

#[derive(Debug)]
pub enum PlayerAction {
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

/// Represents the entire game's state.
#[derive(Debug)]
pub struct PlayState<'a> {
	pub descriptor_set: &'a EntityDescriptorSet,
	pub zones: HashMap<Id, PlayZone>,
	pub players: Players,
	pub priority: Option<Id>,
	pub step: PlayStep,
	pub turn: Id,
}

impl<'a> PlayState<'a> {
	pub fn new(players: Vec<Player>, descriptor_set: &'a EntityDescriptorSet) -> PlayState<'a> {
		assert!(!players.is_empty());

		let first_id = players[0].id;

		let mut creating = PlayState {
			descriptor_set,
			zones: HashMap::new(),
			players: Players::new(),
			priority: None,
			step: PlayStep::Untap,
			turn: first_id,
		};

		let battlefield_id = get_id();
		creating.zones.insert(battlefield_id, PlayZone::new(battlefield_id, PlayZoneKind::Battlefield));

		for player in players.into_iter() {
			let hand = PlayZone::new(get_id(), PlayZoneKind::Hand(player.id));
			creating.zones.insert(hand.id, hand);

			let library = PlayZone::new(get_id(), PlayZoneKind::Library(player.id));
			creating.zones.insert(library.id, library);

			creating.players.add(player);
		}

		creating
	}

	pub fn do_action(&mut self, action: PlayerAction) {

	}

	pub fn mutate(&mut self, event: GameMutation) {
		match event {
			GameMutation::AddEntity { zone_id, entity } => {
				let zone = match self.zones.get_mut(&zone_id) {
					Some(zone) => zone,
					None => {
						println!("Couldn't find zone with ID {}", zone_id);
						return;
					}
				};

				zone.entities.push(entity);
			},
			GameMutation::MoveEntity { current_zone_id, entity_id, new_zone_id } => {
				let entity = {
					let zone = match self.zones.get_mut(&current_zone_id) {
						Some(zone) => zone,
						None => {
							println!("Couldn't find zone with ID {}", current_zone_id);
							return;
						}
					};

					let entity_index = zone.entities.iter().position(|ref entity| {
						entity.id == entity_id
					});

					let entity_index = match entity_index {
						Some(v) => v,
						None => {
							println!("Couldn't find entity {} in zone {}", entity_id, current_zone_id);
							return;
						},
					};

					zone.entities.remove(entity_index).clone_new()
				};

				let new_zone = match self.zones.get_mut(&new_zone_id) {
					Some(zone) => zone,
					None => {
						println!("Couldn't find zone with ID {}", new_zone_id);
						return;
					}
				};

				new_zone.entities.push(entity);
			},
			GameMutation::GainPriority(player_id) => {
				self.priority = Some(player_id);
			},
			GameMutation::LosePriority => {
				self.priority = None;
			},
			GameMutation::AdvanceStep(next_step) => {
				self.step = next_step;
			},
			GameMutation::AdvanceTurn(player_id) => {
				self.turn = player_id;
			},
			_ => {}
		}
	}

	pub fn find_zone_where<T: Fn(Id, &PlayZone) -> bool>(&self, condition: T) -> Option<Id> {
		for (key, value) in self.zones.iter() {
			if condition(*key, value) {
				return Some(*key);
			}
		}

		None
	}
}