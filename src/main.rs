extern crate corrosion;

use corrosion::types::*;
use corrosion::id::*;

fn main() {
	// Define the card pool for our new format, 'Standard 2018'
	let mut descriptor_set = EntityDescriptorSet::new();

	descriptor_set.add(EntityDescriptor {
		name: "Grizzly Bears".to_string(),
		cost: Cost::Mana(ManaCost {
			generic: 1,
			green: 1,
			..Default::default()
		}),
		id: get_id(),
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
	let mut state = PlayState::new_default(
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

	// Shove a forest into the user's hand
	let entity = Entity {
		kind: EntityKind::Card,
		descriptor_id: forest_id,
	};

	// Events can be replaced by effects
	state.handle_event(GameEvent::AddEntity {
		zone_id: hand_id,
		entity,
	});

	println!("{:?}", state);
}