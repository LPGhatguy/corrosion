extern crate corrosion;

use corrosion::{PlayerAction, GameStatus};

use corrosion::utility::*;

/// Tests conceding.
#[test]
fn test_concede() {
    let mut game = new_two_player_game();

    let player1_id = game.player_turn_order[0];

    game.do_player_action(player1_id, &PlayerAction::Concede).unwrap();

    assert_eq!(game.current_status, GameStatus::Ended);
}
