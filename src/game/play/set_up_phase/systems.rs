use crate::game::GameState;
use crate::game::loading::component::*;
use crate::game::loading::resource::{InLibraryCards, InLibraryCardsOpponent};
use crate::game::play::PlayState;
use crate::game::play::resource::{TurnCount, TurnPlayer};
use crate::game::resource::Winner;
use bevy::prelude::*;
use bevy::reflect::Set;

use super::event::{GameOver, RespinFzone, RespinSzone, SetupDraw};
pub fn set_up_start(mut set_up_draw_event_writer: EventWriter<SetupDraw>) {
    set_up_draw_event_writer.send(SetupDraw);
    println!("準備フェーズ開始")
}

pub fn set_up_draw(
    query: Query<(Entity, &Card, &Location)>, // クエリとして受け取る
    mut in_library_cards: ResMut<InLibraryCards>, // ミュータブル参照として受け取る
    mut commands: Commands,                   // コマンド用
    turn_player: ResMut<TurnPlayer>,          // ResMutを使う
    mut winner: ResMut<Winner>,               // ResMutを使う
    mut next_state: ResMut<NextState<GameState>>, // ResMutを使う
    turn: Res<TurnCount>,                     // 不変の参照
    mut game_over_event_writer: EventWriter<GameOver>,
    mut set_up_draw_event_reader: EventReader<SetupDraw>,
    mut respin_fzone_event_writer: EventWriter<RespinFzone>,
) {
    for event in set_up_draw_event_reader.read() {
        if turn.value == 1 {
            println!("先行なのでドローなしで")
        } else {
            if let Some(&top_card) = in_library_cards.cards.first() {
                if let Ok((_, card, _)) = query.get(top_card) {
                    println!(
                        "あなたは {}, を引きました entity id: {:?},",
                        card.name, top_card
                    );
                    commands.entity(top_card).insert(Location::InHand);
                    in_library_cards.cards.remove(0); // 先頭のカードを削除
                } else {
                    println!("No cards available in the deck.");
                }
            } else {
                println!("デッキにカードがありません。");
                winner.winner_id = turn_player.non_turnplayer_id;
                game_over_event_writer.send(GameOver {
                    winner_id: winner.winner_id,
                });
                println!("{}, の勝利です", winner.winner_id,);
                next_state.set(GameState::GameOver)
            }
        }
        respin_fzone_event_writer.send(RespinFzone);
    }
}

pub fn set_up_draw_opponent(
    query: Query<(Entity, &Card, &Location)>, // クエリとして受け取る
    mut in_library_cards: ResMut<InLibraryCardsOpponent>, // ミュータブル参照として受け取る
    mut commands: Commands,                   // コマンド用
    turn_player: ResMut<TurnPlayer>,          // ResMutを使う
    mut winner: ResMut<Winner>,               // ResMutを使う
    mut next_state: ResMut<NextState<GameState>>, // ResMutを使う
    turn: Res<TurnCount>,                     // 不変の参照
    mut game_over_event_writer: EventWriter<GameOver>,
    mut set_up_draw_event_reader: EventReader<SetupDraw>,
    mut respin_fzone_event_writer: EventWriter<RespinFzone>,
) {
    for event in set_up_draw_event_reader.read() {
        if turn.value == 1 {
            println!("先行なのでドローなしで")
        } else {
            if let Some(&top_card) = in_library_cards.cards.first() {
                if let Ok((_, card, _)) = query.get(top_card) {
                    println!(
                        "相手は {}, を引きました entity id: {:?},",
                        card.name, top_card
                    );
                    commands.entity(top_card).insert(Location::InHand);
                    in_library_cards.cards.remove(0); // 先頭のカードを削除
                } else {
                    println!("No cards available in the deck.");
                }
            } else {
                println!("デッキにカードがありません。");
                winner.winner_id = turn_player.non_turnplayer_id;
                game_over_event_writer.send(GameOver {
                    winner_id: winner.winner_id,
                });
                println!("{}, の勝利です", winner.winner_id,);
                next_state.set(GameState::GameOver)
            }
        }
        respin_fzone_event_writer.send(RespinFzone);
    }
}

pub fn respin_fzone_cards(
    query: Query<
        (Entity, &Card, &Location, &SpinCondition, &Fighter),
        (With<Fighter>, With<Player1>),
    >,
    // プレイヤー1の&Fighter を取得
    mut commands: Commands,
    mut respin_fzone_event_reader: EventReader<RespinFzone>,
    mut respin_szone_event_writer: EventWriter<RespinSzone>,
) {
    for event in respin_fzone_event_reader.read() {
        for (entity, card, location, spincondition, fighter) in query.iter() {
            // mut ではなく、immutable で取得
            if let Location::FZone = location {
                if let SpinCondition::Spin = spincondition {
                    commands.entity(entity).insert(SpinCondition::ReSpin);
                    println!("{}, をリスピンしました", card.name);
                }

                // Fighter コンポーネントを新しい summoned_sick で更新
                commands.entity(entity).insert(Fighter {
                    summoned_sick: false,
                    ..*fighter
                }); // power はそのまま
                println!("{}, はアタックできます", card.name);
            }
        }
        respin_szone_event_writer.send(RespinSzone);
    }
}

pub fn respin_fzone_cards_opponent(
    query: Query<
        (Entity, &Card, &Location, &SpinCondition, &Fighter),
        (With<Fighter>, With<Player2>),
    >,
    // プレイヤー1の&Fighter を取得
    mut commands: Commands,
    mut respin_fzone_event_reader: EventReader<RespinFzone>,
    mut respin_szone_event_writer: EventWriter<RespinSzone>,
) {
    for event in respin_fzone_event_reader.read() {
        for (entity, card, location, spincondition, fighter) in query.iter() {
            // mut ではなく、immutable で取得
            if let Location::FZone = location {
                if let SpinCondition::Spin = spincondition {
                    commands.entity(entity).insert(SpinCondition::ReSpin);
                    println!("{}, をリスピンしました", card.name);
                }

                // Fighter コンポーネントを新しい summoned_sick で更新
                commands.entity(entity).insert(Fighter {
                    summoned_sick: false,
                    ..*fighter
                }); // power はそのまま
                println!("{}, はアタックできます", card.name);
            }
        }
        respin_szone_event_writer.send(RespinSzone);
    }
}

pub fn respin_szone_cards(
    query: Query<(Entity, &Card, &Location, &SpinCondition), With<Player1>>,
    mut commands: Commands,
    mut respin_szone_event_reader: EventReader<RespinSzone>,
    mut next_state: ResMut<NextState<PlayState>>,
) {
    for event in respin_szone_event_reader.read() {
        for (entity, card, location, spincondition) in query.iter() {
            if let Location::SZone = location {
                if let SpinCondition::Spin = spincondition {
                    commands.entity(entity).insert(SpinCondition::ReSpin);
                    println!("{}, をリスピンしました", card.name);
                }
            }
        }
        next_state.set(PlayState::WithdrawPhase);
        println!("撤退フェーズです")
    }
}

pub fn respin_szone_cards_opponent(
    query: Query<(Entity, &Card, &Location, &SpinCondition), With<Player2>>,
    mut commands: Commands,
    mut respin_szone_event_reader: EventReader<RespinSzone>,
    mut next_state: ResMut<NextState<PlayState>>,
) {
    for event in respin_szone_event_reader.read() {
        for (entity, card, location, spincondition) in query.iter() {
            if let Location::SZone = location {
                if let SpinCondition::Spin = spincondition {
                    commands.entity(entity).insert(SpinCondition::ReSpin);
                    println!("{}, をリスピンしました", card.name);
                }
            }
        }
        next_state.set(PlayState::WithdrawPhase);
    }
}
pub fn automatically_move_to_withdraw_state(mut next_state: ResMut<NextState<PlayState>>) {
    next_state.set(PlayState::WithdrawPhase);
}
