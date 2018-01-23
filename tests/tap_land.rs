extern crate corrosion;

use std::collections::HashMap;

use corrosion::{
    Ability,
    Object,
    ObjectDetails,
    PlayerAction,

    get_id,
    get_timestamp,
};

use corrosion::utility::*;

#[test]
fn test_success() {
    let mut game = new_two_player_game();

    let battlefield_id = get_battlefield_id(&game);
    let forest_ability_id = get_id();

    let forest_abilities = {
        let mut abilities = HashMap::new();
        abilities.insert(forest_ability_id, Ability::AddGreen);

        abilities
    };

    let forest_id = get_id();
    {
        let forest = Object {
            id: forest_id,
            zone: battlefield_id,
            timestamp: get_timestamp(),
            details: ObjectDetails::Forest {
                tapped: false,
            },
            abilities: forest_abilities,
        };
        game.objects.insert(forest_id, forest);
    }

    assert_eq!(game.objects.len(), 1);

    let player1_id = game.player_turn_order[0];

    game.do_player_action(player1_id, &PlayerAction::ActivateAbility {
        object_id: forest_id,
        ability_id: forest_ability_id,
    }).unwrap();

    // Did the land tap?
    let forest = game.objects.get(&forest_id).unwrap();

    match forest.details {
        ObjectDetails::Forest { tapped } => assert!(tapped),
    }

    // Did we get that mana we paid for?
    let mana = *game.mana_pools.get(&player1_id).unwrap();
    assert_eq!(mana, 1);
}
