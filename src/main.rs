extern crate corrosion;

use corrosion::types::*;
use corrosion::id::*;

fn main() {
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

	descriptor_set.add(EntityDescriptor {
		name: "Forest".to_string(),
		cost: Cost::Impossible,
		id: get_id(),
	});

	let player1 = Player::new(get_id());
	let player2 = Player::new(get_id());

	let mut state = PlayState::new_default(
		vec![player1, player2],
		&descriptor_set
	);

	println!("{:?}", state);
}