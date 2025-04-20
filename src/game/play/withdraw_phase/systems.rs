use std::io;

use crate::game::{
    loading::component::{Card, Location, Player1, Player2},
    play::PlayState,
};
use bevy::prelude::*;

use super::event::*;
use super::resource::{FieldCondition, FieldConditionOpponent};

pub fn check_fzone_condition(
    query: Query<(&Card, &Location), With<Player1>>,
    mut field_conditon: ResMut<FieldCondition>,
) {
    // クエリで取得したカードをチェック
    for (card, location) in query.iter() {
        if let Location::FZone = location {
            field_conditon.has_fzone_card = true;
            println!("Fゾーンにカードが見つかりました"); // FZoneのカードが見つかった場合
            break;
        }
    }
}

pub fn check_xzone_condition(
    query: Query<(&Card, &Location), With<Player1>>,
    mut field_conditon: ResMut<FieldCondition>,
) {
    // クエリで取得したカードをチェック
    for (card, location) in query.iter() {
        if let Location::XZone = location {
            field_conditon.has_xzone_card = true;
            println!("Xゾーンにカードが見つかりました"); // FZoneのカードが見つかった場合
            break;
        }
    }
}

pub fn write_think_event(mut rethink_event_writer: EventWriter<Rethink>) {
    rethink_event_writer.send(Rethink);
}

pub fn withdraw_start(mut check_condition_event_writer: EventWriter<CheckCondition>) {
    check_condition_event_writer.send(CheckCondition);
    println!("撤退フェーズです。")
}
pub fn check_condition(
    query: Query<(&Card, &Location), With<Player1>>,
    mut field_conditon: ResMut<FieldCondition>,
    mut check_condition_event_reader: EventReader<CheckCondition>,
    mut withdraw_management_event_writer: EventWriter<WithdrawManagement>,
) {
    for event in check_condition_event_reader.read() {
        for (card, location) in query.iter() {
            if let Location::FZone = location {
                field_conditon.has_fzone_card = true;
                println!("Fゾーンにカードが見つかりました"); // FZoneのカードが見つかった場合
                break;
            }
        }
        for (card, location) in query.iter() {
            if let Location::XZone = location {
                field_conditon.has_xzone_card = true;
                println!("Xゾーンにカードが見つかりました"); // FZoneのカードが見つかった場合
                break;
            }
        }
        withdraw_management_event_writer.send(WithdrawManagement);
    }
}

pub fn check_condition_opponent(
    query: Query<(&Card, &Location), With<Player2>>,
    mut field_conditon: ResMut<FieldConditionOpponent>,
    mut check_condition_event_reader: EventReader<CheckCondition>,
    mut withdraw_management_event_writer: EventWriter<WithdrawManagement>,
) {
    for event in check_condition_event_reader.read() {
        for (card, location) in query.iter() {
            if let Location::FZone = location {
                field_conditon.has_fzone_card = true;
                println!("Fゾーンにカードが見つかりました"); // FZoneのカードが見つかった場合
                break;
            }
        }
        for (card, location) in query.iter() {
            if let Location::XZone = location {
                field_conditon.has_xzone_card = true;
                println!("Xゾーンにカードが見つかりました"); // FZoneのカードが見つかった場合
                break;
            }
        }
        withdraw_management_event_writer.send(WithdrawManagement);
    }
}

pub fn check_the_will_of_withdraw() -> i32 {
    loop {
        println!("[1]:撤退をする [2]:撤退をしない");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // 入力を整数に変換し、結果を返す
        match input.trim().parse::<i32>() {
            Ok(choice) if choice == 1 || choice == 2 => return choice,
            _ => println!("無効な選択です。もう一度入力してください。"),
        }
    }
}

pub fn withdraw_management(
    mut next_state: ResMut<NextState<PlayState>>,
    mut pick_fighter_to_withdraw_event_writer: EventWriter<PickFighterToWithdraw>,
    field_conditon: ResMut<FieldCondition>,
    mut withdraw_management_event_reader: EventReader<WithdrawManagement>,
) {
    for event in withdraw_management_event_reader.read() {
        let choice = check_the_will_of_withdraw();

        match choice {
            1 => {
                if field_conditon.has_fzone_card {
                    println!("撤退できます");
                    pick_fighter_to_withdraw_event_writer.send(PickFighterToWithdraw);
                } else {
                    if field_conditon.has_xzone_card {
                        println!("Fゾーンにカードが無いのでXフェーズに移行します");
                        next_state.set(PlayState::XPhase)
                    } else {
                        println!(
                            "Xゾーンにカードが無いのでＸフェーズをスキップしてメインフェーズに移行します"
                        );
                        next_state.set(PlayState::XPhase)
                    }
                }
            }
            2 => {
                if field_conditon.has_xzone_card {
                    println!("Xフェーズに移行します");
                    next_state.set(PlayState::XPhase);
                } else {
                    println!(
                        "Xゾーンにカードが無いのでＸフェーズをスキップしてメインフェーズに移行します"
                    );
                    next_state.set(PlayState::MainPhase);
                }
            }

            _ => {}
        }
    }
}

pub fn withdraw_management_opponent(
    mut next_state: ResMut<NextState<PlayState>>,
    mut pick_fighter_to_withdraw_event_writer: EventWriter<PickFighterToWithdraw>,
    field_conditon: ResMut<FieldConditionOpponent>,
    mut withdraw_management_event_reader: EventReader<WithdrawManagement>,
) {
    for event in withdraw_management_event_reader.read() {
        let choice = check_the_will_of_withdraw();

        match choice {
            1 => {
                if field_conditon.has_fzone_card {
                    println!("撤退できます");
                    pick_fighter_to_withdraw_event_writer.send(PickFighterToWithdraw);
                } else {
                    if field_conditon.has_xzone_card {
                        println!("Fゾーンにカードが無いのでXフェーズに移行します");
                        next_state.set(PlayState::XPhase)
                    } else {
                        println!(
                            "Xゾーンにカードが無いのでＸフェーズをスキップしてメインフェーズに移行します"
                        );
                        next_state.set(PlayState::XPhase)
                    }
                }
            }
            2 => {
                if field_conditon.has_xzone_card {
                    println!("Xフェーズに移行します");
                    next_state.set(PlayState::XPhase);
                } else {
                    println!(
                        "Xゾーンにカードが無いのでＸフェーズをスキップしてメインフェーズに移行します"
                    );
                    next_state.set(PlayState::MainPhase);
                }
            }

            _ => {}
        }
    }
}

pub fn pick_fighter_to_withdraw(
    query: Query<(Entity, &Card, &Location), With<Player1>>,
    mut withdraw_event_writer: EventWriter<Withdraw>,
    mut pick_fighter_to_withdraw_event_reader: EventReader<PickFighterToWithdraw>,
    mut withdraw_management_event_writer: EventWriter<WithdrawManagement>,
) {
    for event in pick_fighter_to_withdraw_event_reader.read() {
        let mut cards_in_fzone = Vec::new();
        println!("Fゾーンにあるあなたのカードは:");
        for (entity, card, location) in query.iter() {
            if let Location::FZone = location {
                cards_in_fzone.push(entity); // エンティティIDを保存
                println!("{}: {}[{}]", cards_in_fzone.len(), card.name, card.energy); // 番号を振って表示
            }
        }

        if cards_in_fzone.is_empty() {
            println!("No cards in fzone.");
            return;
        }

        println!("Select a card by number (1-{}):", cards_in_fzone.len());

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // Trim whitespace and attempt to parse the input
            match input.trim().parse::<usize>() {
                Ok(selected_index) => {
                    if selected_index == 0 {
                        // 0が選択された場合
                        withdraw_management_event_writer.send(WithdrawManagement);
                        println!("Withdraw management initiated.");
                        break; // ループを抜ける
                    }
                    if selected_index > 0 && selected_index <= cards_in_fzone.len() {
                        let selected_entity = cards_in_fzone[selected_index - 1];

                        if let Ok((_, card, _)) = query.get(selected_entity) {
                            withdraw_event_writer.send(Withdraw {
                                id: selected_entity,
                            });
                            println!("{} を撤退するファイターとして選択しました", card.name); // カード名を表示
                        }
                        println!("You selected card with Entity ID: {:?}", selected_entity);
                        break; // 有効な選択が行われたらループを抜ける
                    } else {
                        println!(
                            "Invalid selection. Please enter a number between 1 and {}.",
                            cards_in_fzone.len()
                        );
                    }
                }
                Err(_) => {
                    println!("Please enter a valid number.");
                }
            }
        }
    }
}

pub fn pick_fighter_to_withdraw_opponent(
    query: Query<(Entity, &Card, &Location), With<Player2>>,
    mut withdraw_event_writer: EventWriter<Withdraw>,
    mut pick_fighter_to_withdraw_event_reader: EventReader<PickFighterToWithdraw>,
    mut withdraw_management_event_writer: EventWriter<WithdrawManagement>,
) {
    for event in pick_fighter_to_withdraw_event_reader.read() {
        let mut cards_in_fzone = Vec::new();
        println!("Fゾーンにあるあなたのカードは:");
        for (entity, card, location) in query.iter() {
            if let Location::FZone = location {
                cards_in_fzone.push(entity); // エンティティIDを保存
                println!("{}: {}[{}]", cards_in_fzone.len(), card.name, card.energy); // 番号を振って表示
            }
        }

        if cards_in_fzone.is_empty() {
            println!("No cards in hand.");
            return;
        }

        println!("Select a card by number (1-{}):", cards_in_fzone.len());

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // Trim whitespace and attempt to parse the input
            match input.trim().parse::<usize>() {
                Ok(selected_index) => {
                    if selected_index == 0 {
                        // 0が選択された場合
                        withdraw_management_event_writer.send(WithdrawManagement);
                        println!("Withdraw management initiated.");
                        break; // ループを抜ける
                    }
                    if selected_index > 0 && selected_index <= cards_in_fzone.len() {
                        let selected_entity = cards_in_fzone[selected_index - 1];

                        if let Ok((_, card, _)) = query.get(selected_entity) {
                            withdraw_event_writer.send(Withdraw {
                                id: selected_entity,
                            });
                            println!("{} を撤退するファイターとして選択しました", card.name); // カード名を表示
                        }
                        println!("You selected card with Entity ID: {:?}", selected_entity);
                        break; // 有効な選択が行われたらループを抜ける
                    } else {
                        println!(
                            "Invalid selection. Please enter a number between 1 and {}.",
                            cards_in_fzone.len()
                        );
                    }
                }
                Err(_) => {
                    println!("Please enter a valid number.");
                }
            }
        }
    }
}

pub fn withdraw(
    mut withdraw_event_reader: EventReader<Withdraw>,
    mut withdraw_management_event_writer: EventWriter<WithdrawManagement>,
    query: Query<(&Card, &Location), With<Player1>>,
    mut commands: Commands,
    field_conditon: ResMut<FieldCondition>,
    mut next_state: ResMut<NextState<PlayState>>,
    mut withdraw_finish_event_writer: EventWriter<WithdrawFinish>,
) {
    for event in withdraw_event_reader.read() {
        let mut sum_of_energy = 0;
        for (card, location) in query.iter() {
            if let Location::SZone = location {
                sum_of_energy += card.energy;
                println!("{} {}", card.name, sum_of_energy);
            }
        }

        println!(
            "あなたのＳゾーンのエナジーの合計は、{}です。",
            sum_of_energy
        );

        if let Ok((card, location)) = query.get(event.id) {
            sum_of_energy += card.energy;
            if sum_of_energy > 10 {
                println!(
                    "このカードはエナジーが大きすぎて、撤退できません。もう一度選びなおしてください"
                );
                withdraw_management_event_writer.send(WithdrawManagement);
            } else {
                println!("{}を撤退させました", card.name);
                commands.entity(event.id).insert(Location::SZone);
                withdraw_finish_event_writer.send(WithdrawFinish { id: event.id });
                if field_conditon.has_xzone_card {
                    println!("Xフェーズです");
                    next_state.set(PlayState::XPhase);
                } else {
                    println!("Xゾーンにカードが無いので、スキップします。");
                    println!("メインフェーズです");
                    next_state.set(PlayState::MainPhase);
                }
            }
        } else {
            // エンティティが見つからない場合の処理
            println!(
                "Entity ID: {:?} has no components or does not exist.",
                event.id
            );
        }
    }
}

pub fn withdraw_opponent(
    mut withdraw_event_reader: EventReader<Withdraw>,
    mut withdraw_management_event_writer: EventWriter<WithdrawManagement>,
    query: Query<(&Card, &Location), With<Player2>>,
    mut commands: Commands,
    field_conditon: ResMut<FieldConditionOpponent>,
    mut next_state: ResMut<NextState<PlayState>>,
    mut withdraw_finish_event_writer: EventWriter<WithdrawFinish>,
) {
    for event in withdraw_event_reader.read() {
        let mut sum_of_energy = 0;
        for (card, location) in query.iter() {
            if let Location::SZone = location {
                sum_of_energy += card.energy;
                println!("{} {}", card.name, sum_of_energy);
            }
        }

        println!(
            "あなたのＳゾーンのエナジーの合計は、{}です。",
            sum_of_energy
        );

        if let Ok((card, location)) = query.get(event.id) {
            sum_of_energy += card.energy;
            if sum_of_energy > 10 {
                println!(
                    "このカードはエナジーが大きすぎて、撤退できません。もう一度選びなおしてください"
                );
                withdraw_management_event_writer.send(WithdrawManagement);
            } else {
                println!("{}を撤退させました", card.name);
                commands.entity(event.id).insert(Location::SZone);
                withdraw_finish_event_writer.send(WithdrawFinish { id: event.id });
                if field_conditon.has_xzone_card {
                    next_state.set(PlayState::XPhase);
                } else {
                    println!("Xゾーンにカードが無いので、Xフェーズはスキップします。");
                    next_state.set(PlayState::MainPhase);
                }
            }
        } else {
            // エンティティが見つからない場合の処理
            println!(
                "Entity ID: {:?} has no components or does not exist.",
                event.id
            );
        }
    }
}
