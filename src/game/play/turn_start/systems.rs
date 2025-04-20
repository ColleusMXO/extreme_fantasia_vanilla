use bevy::prelude::*;
use crate::game::play::resource::*;
use crate::game::set_up::resource::*;
use crate::game::play::PlayState;



pub fn turn_start(mut turn_count: ResMut<TurnCount>) {
    turn_count.value += 1;
    println!("ターンを開始しました");
    println!("現在{}ターン目です", turn_count.value);

}
pub fn automatically_move_to_setup_phase(mut next_state: ResMut<NextState<PlayState>>) {
    next_state.set(PlayState::SetupPhase);
}
