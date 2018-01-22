extern crate corrosion;

use corrosion::{
    Entity,
    EntityDetails,
    Game,
    Id,
    PlayerAction,

    get_id,
    get_timestamp,
};

use corrosion::utility::*;

fn setup() -> (Game, Id) {
    let mut game = new_two_player_game();

    let player1_id = game.player_turn_order[0];

    let hand_id = get_hand_id(&game, player1_id);

    let forest_id = get_id();
    let forest = Entity {
        id: forest_id,
        zone: hand_id,
        timestamp: get_timestamp(),
        details: EntityDetails::Forest,
        abilities: Vec::new(),
    };
    game.entities.insert(forest_id, forest);

    assert_eq!(game.entities.len(), 1);

    (game, forest_id)
}

#[test]
fn test_success() {
    let (mut game, forest_id) = setup();

    let player1_id = game.player_turn_order[0];
    let battlefield_id = get_battlefield_id(&game);

    game.do_player_action(player1_id, &PlayerAction::PlayLand {
        entity_id: forest_id,
    }).unwrap();

    // Entities change identity when they change zones
    assert!(game.entities.get(&forest_id).is_none());
    assert_eq!(game.entities.len(), 1);

    let new_forest = game.entities.values().next().unwrap();

    assert_eq!(new_forest.zone, battlefield_id);
}

#[test]
fn test_fail_wrong_player() {
    let (mut game, forest_id) = setup();

    let player2_id = game.player_turn_order[1];

    let result = game.do_player_action(player2_id, &PlayerAction::PlayLand {
        entity_id: forest_id,
    });

    assert!(result.is_err());

    // The entity didn't move!
    assert!(game.entities.get(&forest_id).is_some());
    assert_eq!(game.entities.len(), 1);
}
