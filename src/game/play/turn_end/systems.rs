use bevy::prelude::*;

use crate::game::play::{self, Controller, PlayState, resource::TurnPlayer};

pub fn change_turn_player(
    turn_player: ResMut<TurnPlayer>,
    mut next_state: ResMut<NextState<PlayState>>,
    mut next_turn_player: ResMut<NextState<Controller>>,
    controller: Res<State<Controller>>,
) {
    println!("{}のターンが終了しました。", turn_player.turnplayer_id);
    match controller.get() {
        Controller::Opponent => {
            next_turn_player.set(Controller::Player);
        }
        Controller::Player => {
            next_turn_player.set(Controller::Opponent);
        }
    }
    next_state.set(PlayState::TurnStart);
}
