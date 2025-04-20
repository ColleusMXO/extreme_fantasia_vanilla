use bevy::prelude::*;


use crate::game::{play::resource::{TurnCount, TurnPlayer}, set_up::resource::PlayerSort};


pub fn update_turn_player(turn: Res<TurnCount>, playersort: Res<PlayerSort>, mut turnplayer: ResMut<TurnPlayer>) {
    if turn.is_changed() {
        if turn.value % 2 == 1 {
            turnplayer.turnplayer_id = playersort.firstplayer_id;
            turnplayer.non_turnplayer_id = playersort.secondplayer_id;
        } else {
            turnplayer.turnplayer_id = playersort.secondplayer_id;
            turnplayer.non_turnplayer_id = playersort.firstplayer_id;
        }
        println!("{}, のターンです", turnplayer.turnplayer_id,);
    }
}
