use std::io;

use bevy::prelude::*;
use crate::game::play::main_phase::battle::event::AttackStart;
use crate::game::play::main_phase::cardplay::event::PlayStart;
use crate::game::play::main_phase::resource::*;
use crate::game::play::main_phase::event::*;
use crate::game::loading::component::*;
use crate::game::play::set_up_phase::event::GameOver;
use crate::game::play::PlayState;
use crate::game::GameState;
use super::event::*;

pub fn situation_log(
    mut hand: ResMut<Player1Hand>,
    mut szone: ResMut<Player1SZone>,
    mut fzone: ResMut<Player1FZone>,
    mut xzone: ResMut<Player1XZone>,
    mut szone_opponent: ResMut<Player2SZone>,
    mut fzone_opponent: ResMut<Player2FZone>,
    mut xzone_opponent: ResMut<Player2XZone>,
    query: Query<(Entity, &Card, &Location), With<Player1>>,
    query_opponent: Query<(Entity, &Card, &Location), With<Player2>>,
    query_first_energy: Query<(Entity, &Card), (With<Player1>, With<FirstEnergy>)>,
    query_first_energy_opponent: Query<(Entity, &Card), (With<Player2>, With<FirstEnergy>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut check_event_reader: EventReader<SituationLog>,
    mut continue_main_phase_event_writer: EventWriter<ContinueMainPhase>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut commands: Commands,
) {
    for _event in check_event_reader.read() {
        // プレイヤー1の手札をチェック
        hand.cards.clear();
        fzone.cards.clear();
        szone.cards.clear();
        xzone.cards.clear();

        for (entity, card, location) in query.iter() {
            if let Location::InHand = location {
                hand.cards.push(entity);
            }
            if let Location::SZone = location {
                szone.cards.push(entity);
            }
            if let Location::FZone = location {
                fzone.cards.push(entity);
            }
            if let Location::XZone = location {
                xzone.cards.push(entity);
            }
        }

        // プレイヤー2のSゾーンとFゾーンをチェック
        szone_opponent.cards.clear();
        fzone_opponent.cards.clear();
        xzone_opponent.cards.clear();

        for (entity, card, location) in query_opponent.iter() {
            if let Location::SZone = location {
                szone_opponent.cards.push(entity);
            }
            if let Location::FZone = location {
                fzone_opponent.cards.push(entity);
            }
            if let Location::XZone = location {
                xzone_opponent.cards.push(entity);
            }
        }
        println!("--- フィールド情報 ---");
        println!("プレイヤー1の手札: {:?}", hand.cards);
        println!("プレイヤー1のSゾーン: {:?}", szone.cards);
        println!("プレイヤー1のFゾーン: {:?}", fzone.cards);
        println!("プレイヤー1のXゾーン: {:?}", xzone.cards);

        println!("プレイヤー2のSゾーン: {:?}", szone_opponent.cards);
        println!("プレイヤー2のFゾーン: {:?}", fzone_opponent.cards);
        println!("プレイヤー2のXゾーン: {:?}", xzone_opponent.cards);
        if xzone.cards.len() >= 6 {
            println!("あなたのXゾーンには6枚以上のカードがあります。");

            for (entity, card) in query_first_energy.iter() {
                commands.entity(entity).insert(Location::XZone);
                println!("あなたのの{}がXゾーンに置かれました。", card.name);
            }
            game_over_event_writer.send(GameOver {
                winner_id: 2,
            });
            next_state.set(GameState::GameOver);
        }

        if xzone_opponent.cards.len() >= 6 {
            println!("相手ののXゾーンには6枚以上のカードがあります。");

            for (entity, card) in query_first_energy_opponent.iter() {
                commands.entity(entity).insert(Location::XZone);
                println!("相手の{}がXゾーンに置かれました。", card.name);
            }
            game_over_event_writer.send(GameOver {
                winner_id: 1,
            });
            next_state.set(GameState::GameOver);
        }
        println!("フィールド情報の更新をしました");
        continue_main_phase_event_writer.send(ContinueMainPhase);
    }
}


pub fn situation_log_opponent(
    mut hand: ResMut<Player2Hand>,
    mut szone: ResMut<Player2SZone>,
    mut fzone: ResMut<Player2FZone>,
    mut xzone: ResMut<Player2XZone>,
    mut szone_opponent: ResMut<Player1SZone>,
    mut fzone_opponent: ResMut<Player1FZone>,
    mut xzone_opponent: ResMut<Player1XZone>,
    query: Query<(Entity, &Card, &Location), With<Player2>>,
    query_opponent: Query<(Entity, &Card, &Location), With<Player1>>,
    query_first_energy: Query<(Entity, &Card), (With<Player2>, With<FirstEnergy>)>,
    query_first_energy_opponent: Query<(Entity, &Card), (With<Player1>, With<FirstEnergy>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut check_event_reader: EventReader<SituationLog>,
    mut continue_main_phase_event_writer: EventWriter<ContinueMainPhase>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut commands: Commands,
) {
    for _event in check_event_reader.read() {
        // プレイヤー1の手札をチェック
        hand.cards.clear();
        fzone.cards.clear();
        szone.cards.clear();
        xzone.cards.clear();

        for (entity, card, location) in query.iter() {
            if let Location::InHand = location {
                hand.cards.push(entity);
            }
            if let Location::SZone = location {
                szone.cards.push(entity);
            }
            if let Location::FZone = location {
                fzone.cards.push(entity);
            }
            if let Location::XZone = location {
                xzone.cards.push(entity);
            }
        }

        // プレイヤー2のSゾーンとFゾーンをチェック
        szone_opponent.cards.clear();
        fzone_opponent.cards.clear();
        xzone_opponent.cards.clear();

        for (entity, card, location) in query_opponent.iter() {
            if let Location::SZone = location {
                szone_opponent.cards.push(entity);
            }
            if let Location::FZone = location {
                fzone_opponent.cards.push(entity);
            }
            if let Location::XZone = location {
                xzone_opponent.cards.push(entity);
            }
        }
        println!("--- フィールド情報 ---");
        println!("プレイヤー2の手札: {:?}", hand.cards);
        println!("プレイヤー2のSゾーン: {:?}", szone.cards);
        println!("プレイヤー2のFゾーン: {:?}", fzone.cards);
        println!("プレイヤー2のXゾーン: {:?}", xzone.cards);

        println!("プレイヤー1のSゾーン: {:?}", szone_opponent.cards);
        println!("プレイヤー1のFゾーン: {:?}", fzone_opponent.cards);
        println!("プレイヤー1のXゾーン: {:?}", xzone_opponent.cards);
        if xzone.cards.len() >= 6 {
            println!("あなたのXゾーンには6枚以上のカードがあります。");

            for (entity, card) in query_first_energy.iter() {
                commands.entity(entity).insert(Location::XZone);
                println!("あなたのの{}がXゾーンに置かれました。", card.name);
            }
            game_over_event_writer.send(GameOver {
                winner_id: 1,
            });
            next_state.set(GameState::GameOver);
        }

        if xzone_opponent.cards.len() >= 6 {
            println!("相手ののXゾーンには6枚以上のカードがあります。");

            for (entity, card) in query_first_energy_opponent.iter() {
                commands.entity(entity).insert(Location::XZone);
                println!("相手の{}がXゾーンに置かれました。", card.name);
            }
            game_over_event_writer.send(GameOver {
                winner_id: 2,
            });
            next_state.set(GameState::GameOver);
        }
        println!("フィールド情報の更新をしました");
        continue_main_phase_event_writer.send(ContinueMainPhase);
    }
}

pub fn check_the_will_of_turn_end() -> i32 {
    loop {
        println!("[1]:メインフェーズを続ける [0]:ターンエンドする");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // 入力を整数に変換し、結果を返す
        match input.trim().parse::<i32>() {
            Ok(choice) if choice == 1 || choice == 0 => return choice,
            _ => println!("無効な選択です。もう一度入力してください。"),
        }
    }
}

pub fn continue_main_phase(
    mut next_state: ResMut<NextState<PlayState>>,
    mut continue_main_phase_event_reader: EventReader<ContinueMainPhase>,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
) {
    for event in continue_main_phase_event_reader.read() {
        let choice = check_the_will_of_turn_end();

        match choice {
            1 => {
                main_phase_management_event_writer.send(MainPhaseManagement);
            }
            0 => {
                println!("ターンエンドしました");
                next_state.set(PlayState::TurnEnd);
            }

            _ => {}
        }
    }
}

pub fn check_the_will_of_battle_or_play() -> i32 {
    loop {
        println!("[1]:ファイターで攻撃する [2]:カードをプレイする [0]:メインフェーズ終了する");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // 入力を整数に変換し、結果を返す
        match input.trim().parse::<i32>() {
            Ok(choice) if choice == 1 || choice == 0 || choice == 2 => return choice,
            _ => println!("無効な選択です。もう一度入力してください。"),
        }
    }
}

pub fn main_phase_management(
    mut next_state: ResMut<NextState<PlayState>>,
    mut main_phase_management_event_reader: EventReader<MainPhaseManagement>,
    mut attack_start_event_writer: EventWriter<AttackStart>,
    mut play_start_event_writer: EventWriter<PlayStart>,
) {
    for event in main_phase_management_event_reader.read() {
        let choice = check_the_will_of_battle_or_play();

        match choice {
            1 => {
                attack_start_event_writer.send(AttackStart);
            }

            2 => {
                play_start_event_writer.send(PlayStart);
            }
            0 => {
                println!("メインフェーズ終了しました");
                next_state.set(PlayState::TurnEnd);
            }

            _ => {}
        }
    }
}

