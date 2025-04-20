use bevy::prelude::*;

use crate::game::{
    loading::component::Card, play::set_up_phase::event::GameOver, set_up::resource::{FirstHand, FirstHandOpponent},
};

pub fn game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    first_hand: Res<FirstHand>,
    first_hand_opponent: Res<FirstHandOpponent>,

    query: Query<&Card>,
) {
    for event in game_over_event_reader.read() {
        println!("ゲームの勝者は、プレイヤー{}です", event.winner_id);
        let cards_list:Vec<String>= first_hand
        .cards
        .iter()
        .filter_map(|entity| query.get(*entity).ok().map(|card| card.name.clone()))
        .collect();

        let cards_list_opponent:Vec<String>= first_hand_opponent
        .cards
        .iter()
        .filter_map(|entity| query.get(*entity).ok().map(|card| card.name.clone()))
        .collect();
        
        println!("プレイヤー1の初期の手札は: {:?}でした", cards_list);
        println!("プレイヤー2の初期の手札は: {:?}でした", cards_list_opponent);
    }
}