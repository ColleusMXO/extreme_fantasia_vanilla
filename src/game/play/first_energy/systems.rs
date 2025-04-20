use std::io;

use bevy::{prelude::*, state::commands};

use crate::game::{
    loading::component::{Card, FaceCondition}, play::x_phase::event::Xskill, resource::FacedownXZoneCards,
};

use super::event::Open;

fn yuukannoenazi(
    facedown_cards: &Res<FacedownXZoneCards>,
    mut x_skill_event_reader: EventReader<Xskill>,
    mut open_event_writer: EventWriter<Open>, 
    query: Query<&Card>,
    mut commands: Commands,
) {
    for event in x_skill_event_reader.read() {
        if let Ok(card) = query.get(event.first_energy_id) {
            match card.name.as_str() {
                "勇敢のエナジー" => {}
                "情熱のエナジー" => {
                    let chosen_entities = choose_entities_from_facedown_xzone(facedown_cards);
                    let x = chosen_entities.len();
                    for entity in chosen_entities{
                        commands.entity(entity).insert(FaceCondition::Open);
                        open_event_writer.send(Open { open_cards_id: entity });
                    }
                    for n in 0..x{

                    }
                }
                _ => {}
            }
        } else {
            println!(
                "指定されたEntity ID {:?} はカードではありません。または存在しません。",
                event.first_energy_id
            );
        }
    }
}

fn choose_entities_from_facedown_xzone(facedown_cards: &Res<FacedownXZoneCards>) -> Vec<Entity> {
    let mut chosen_entities: Vec<Entity> = Vec::new(); // 選択されたEntityの配列を作成

    println!("以下のFacedown XZoneカードを選択してください:");

    loop {
        // 選択肢を表示
        for (index, entity) in facedown_cards.cards.iter().enumerate() {
            println!("{}: {:?}", index + 1, entity);
        }
        println!("0: 選択を終了します。");

        // ユーザーからの入力を受け付ける（数値を取得する方法）
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("入力エラー");
        let trimmed = buffer.trim();

        // 数値に変換を試みる
        match trimmed.parse::<usize>() {
            Ok(choice_index) => {
                // 0が選ばれた場合は選択を終了できない
                if choice_index == 0 {
                    if chosen_entities.is_empty() {
                        println!("少なくとも一つのカードを選択してください。"); // 選択されていない場合の警告
                        continue; // 再選択を促す
                    } else {
                        break; // 選択が1つ以上ある場合はループを終了
                    }
                }

                // 入力が有効な範囲かどうか確認
                if choice_index > 0 && choice_index <= facedown_cards.cards.len() {
                    let chosen_entity = facedown_cards.cards[choice_index - 1]; // 選択されたEntity ID

                    // 確認プロンプトを表示
                    println!("{:?} を選択しました。これでよろしいですか？", chosen_entity);
                    if check_the_will_of_open_cards() == 1 {
                        chosen_entities.push(chosen_entity); // 選択確定として配列に追加
                    } else {
                        println!("再度選択してください。");
                    }
                } else {
                    println!("無効な数字です。再度選択してください。");
                }
            }
            Err(_) => {
                println!("無効な入力です。再度選択してください。");
            }
        }
    }

    chosen_entities // 最終的に選ばれたEntityの配列を返す
}

fn choose_entity_from_facedown_xzone(facedown_cards: Res<FacedownXZoneCards>) -> Entity {
    // 選択肢を表示
    println!("以下のFacedown XZoneカードを選択してください:");

    for (index, entity) in facedown_cards.cards.iter().enumerate() {
        println!("{}: {:?}", index + 1, entity);
    }

    loop {
        // ユーザーからの入力を受け付ける（数値を取得する方法）
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("入力エラー");
        let trimmed = buffer.trim();

        // 数値に変換を試みる
        match trimmed.parse::<usize>() {
            Ok(choice_index) => {
                // 入力が有効な範囲かどうか確認
                if choice_index > 0 && choice_index <= facedown_cards.cards.len() {
                    let chosen_entity = facedown_cards.cards[choice_index - 1]; // 選択されたEntity ID

                    // 確認プロンプトを表示
                    println!("{:?} を選択しました。これでよろしいですか？", chosen_entity);
                    if check_the_will_of_open_cards() == 1 {
                        return chosen_entity; // 選択確定
                    } else {
                        println!("再度選択してください。");
                    }
                } else {
                    println!("無効な数字です。再度選択してください。");
                }
            }
            Err(_) => {
                println!("無効な入力です。再度選択してください。");
            }
        }
    }
}

pub fn check_the_will_of_open_cards() -> i32 {
    loop {
        println!("[1]:決定する [2]:やりなおす ");
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
