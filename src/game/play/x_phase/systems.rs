use std::io;

use bevy::prelude::*;

use crate::game::resource::*;
use crate::game::{
    loading::component::{
        Card, FaceCondition, FirstEnergy, Location, Player1, Player2, SpinCondition,
    },
    play::PlayState,
};

use super::event::{ListFacedownCardsInXzone, XPhaseManagement, Xskill};

pub fn x_phase_start(
    mut list_facedown_cards_in_xzone_event_writer: EventWriter<ListFacedownCardsInXzone>,
) {
    list_facedown_cards_in_xzone_event_writer.send(ListFacedownCardsInXzone);
    println!("Xフェーズです");
}

pub fn see_facedown_cards_in_xzone(
    query: Query<(Entity, &Card, &Location, &FaceCondition), With<Player1>>,
    mut in_xzone_card: ResMut<FacedownXZoneCards>,
    mut list_facedown_cards_in_xzone_event_reader: EventReader<ListFacedownCardsInXzone>,
    mut x_phase_management_event_writer: EventWriter<XPhaseManagement>,
) {
    for event in list_facedown_cards_in_xzone_event_reader.read() {
        println!("Facedown cards in XZone are:");

        println!("{:?}", in_xzone_card.cards);
        x_phase_management_event_writer.send(XPhaseManagement);
    }
}

pub fn see_facedown_cards_in_xzone_opponent(
    query: Query<(Entity, &Card, &Location, &FaceCondition), With<Player2>>,
    mut in_xzone_card: ResMut<FacedownXZoneCardsOpponent>,
    mut list_facedown_cards_in_xzone_event_reader: EventReader<ListFacedownCardsInXzone>,
    mut x_phase_management_event_writer: EventWriter<XPhaseManagement>,
) {
    for event in list_facedown_cards_in_xzone_event_reader.read() {
        println!("Facedown cards in XZone are:");

        println!("{:?}", in_xzone_card.cards);        
        x_phase_management_event_writer.send(XPhaseManagement);
    }
}

pub fn x_phase_management(
    mut next_state: ResMut<NextState<PlayState>>,
    mut x_phase_management_event_reader: EventReader<XPhaseManagement>,
    in_xzone_card: ResMut<FacedownXZoneCards>,
    mut xskill_event_writher: EventWriter<Xskill>,
    query: Query<(Entity, &Card), (With<FirstEnergy>, With<Player1>)>,
) {
    for event in x_phase_management_event_reader.read() {
        for (entity, card) in query.iter() {
            println!("{}のXスキルを使いますか？", card.name);

            let choice = check_the_will_of_xskill();

            match choice {
                1 => {
                    if in_xzone_card.cards.is_empty() {
                        println!("Xゾーンに裏側のカードがありません。メインフェーズに移行します。");
                        next_state.set(PlayState::MainPhase);
                    } else {
                        xskill_event_writher.send(Xskill {
                            first_energy_id: entity,
                        });
                        println!("Xスキルを発動します。");
                    }
                }
                2 => {
                    println!("メインフェーズに移行します。");
                    next_state.set(PlayState::MainPhase);
                }

                _ => {}
            }
        }
    }
}

pub fn x_phase_management_opponent(
    mut next_state: ResMut<NextState<PlayState>>,
    mut x_phase_management_event_reader: EventReader<XPhaseManagement>,
    in_xzone_card: ResMut<FacedownXZoneCardsOpponent>,
    mut xskill_event_writher: EventWriter<Xskill>,
    query: Query<(Entity, &Card), (With<FirstEnergy>, With<Player2>)>,
) {
    for event in x_phase_management_event_reader.read() {
        for (entity, card) in query.iter() {
            println!("{}のXスキルを使いますか？", card.name);

            let choice = check_the_will_of_xskill();

            match choice {
                1 => {
                    if in_xzone_card.cards.is_empty() {
                        println!("Xゾーンに裏側のカードがありません。メインフェーズに移行します。");
                        next_state.set(PlayState::MainPhase);
                    } else {
                        xskill_event_writher.send(Xskill {
                            first_energy_id: entity,
                        });
                        println!("Xスキルを発動します。");
                    }
                }
                2 => {
                    println!("メインフェーズに移行します。");
                    next_state.set(PlayState::MainPhase);
                }

                _ => {}
            }
        }
    }
}

pub fn check_the_will_of_xskill() -> i32 {
    loop {
        println!("[1]:Xスキルを使う [2]:使わない");
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

pub fn xskill(
    mut xskill_event_reader: EventReader<Xskill>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<PlayState>>,
    query: Query<(Entity, &Card), (With<FirstEnergy>, With<Player1>)>,
) {
    for event in xskill_event_reader.read() {
        if let Ok((entity, card)) = query.get(event.first_energy_id) {
            println!("{}のXスキルは調整中です。", card.name)
        }
        next_state.set(PlayState::MainPhase);
    }
}

pub fn xskill_opponent(
    mut xskill_event_reader: EventReader<Xskill>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<PlayState>>,
    query: Query<(Entity, &Card), (With<FirstEnergy>, With<Player2>)>,
) {
    for event in xskill_event_reader.read() {
        if let Ok((entity, card)) = query.get(event.first_energy_id) {
            println!("{}のXスキルは調整中です。", card.name)
        }
        next_state.set(PlayState::MainPhase);
    }
}

pub fn select_a_facedown_card(in_xzone_card: ResMut<FacedownXZoneCards>) -> Entity {
    println!("Select a card by number (1-{}):", in_xzone_card.cards.len());

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(selected_index) = input.trim().parse::<usize>() {
            if selected_index > 0 && selected_index <= in_xzone_card.cards.len() {
                let selected_entity = in_xzone_card.cards[selected_index - 1];
                return selected_entity; // 有効な選択が行われた場合に返す
            } else {
                println!(
                    "Invalid selection. Please enter a number between 1 and {}.",
                    in_xzone_card.cards.len()
                );
            }
        } else {
            println!("Please enter a valid number.");
        }
    }
}
