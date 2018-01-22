extern crate corrosion;

use corrosion::PlayerAction;

use corrosion::utility::*;

/// Tests passing of priority and turn cycling. Currently assumes only one
/// phase.
#[test]
fn test_priority() {
    let mut game = new_two_player_game();

    let player1_id = game.player_turn_order[0];
    let player2_id = game.player_turn_order[1];

    assert_eq!(game.active_player, Some(player1_id));
    assert_eq!(game.priority_player, Some(player1_id));

    game.do_player_action(player1_id, &PlayerAction::PassPriority).unwrap();

    assert_eq!(game.active_player, Some(player1_id));
    assert_eq!(game.priority_player, Some(player2_id));

    game.do_player_action(player2_id, &PlayerAction::PassPriority).unwrap();

    assert_eq!(game.active_player, Some(player2_id));
    assert_eq!(game.priority_player, Some(player2_id));

    game.do_player_action(player2_id, &PlayerAction::PassPriority).unwrap();

    assert_eq!(game.active_player, Some(player2_id));
    assert_eq!(game.priority_player, Some(player1_id));

    game.do_player_action(player1_id, &PlayerAction::PassPriority).unwrap();

    assert_eq!(game.active_player, Some(player1_id));
    assert_eq!(game.priority_player, Some(player1_id));
}
