extern crate corrosion;

use corrosion::types::*;
use corrosion::id::*;

fn main() {
	// Define the card pool for our new format, 'Standard 2018'
	let mut descriptor_set = EntityDescriptorSet::new();

	let bears_id = get_id();
	descriptor_set.add(EntityDescriptor {
		name: "Grizzly Bears".to_string(),
		cost: Cost::Mana(ManaCost {
			generic: 1,
			green: 1,
			..Default::default()
		}),
		id: bears_id,
	});

	let forest_id = get_id();
	descriptor_set.add(EntityDescriptor {
		name: "Forest".to_string(),
		cost: Cost::Impossible,
		id: forest_id,
	});

	// Add a couple players
	let player1_id = get_id();
	let player1 = Player::new(player1_id);

	let player2_id = get_id();
	let player2 = Player::new(player2_id);

	// Build the actual game state.
	// new_default will construct default zones and add players to the game.
	let mut state = PlayState::new(
		vec![player1, player2],
		&descriptor_set
	);

	// Locate the handle associated with player1
	let result = state.find_zone_where(|_, zone| {
		match zone.kind {
			PlayZoneKind::Hand(id) => id == player1_id,
			_ => false,
		}
	});

	// We might've failed to find our zone!
	let hand_id = match result {
		Some(v) => v,
		None => {
			println!("Couldn't find hand zone for player 1, aborting.");
			return;
		},
	};

	// Shove two forests into player1's hand
	let forest1 = Entity {
		kind: EntityKind::Card,
		id: get_id(),
		descriptor_id: forest_id,
	};

	let forest2 = Entity {
		kind: EntityKind::Card,
		id: get_id(),
		descriptor_id: forest_id,
	};

	// Events can be replaced by effects
	state.mutate(GameMutation::AddEntity {
		zone_id: hand_id,
		entity: forest1,
	});

	state.mutate(GameMutation::AddEntity {
		zone_id: hand_id,
		entity: forest2,
	});

	// Put a Grizzly Bears into player1's hand
	let bears = Entity {
		kind: EntityKind::Card,
		id: get_id(),
		descriptor_id: bears_id,
	};

	state.mutate(GameMutation::AddEntity {
		zone_id: hand_id,
		entity: bears,
	});

	println!("{:?}", state);
}