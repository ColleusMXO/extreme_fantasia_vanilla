use std::io;

use bevy::{prelude::*, state::commands, text::cosmic_text::rustybuzz::Face};

use super::event::*;
use crate::game::{
    loading::component::*,
    play::{
        self,
        main_phase::{
            event::*,
            free::event::{MainPhaseManagement, SituationLog},
            resource::*,
        },
    },
};
use crate::game::resource::*;

pub fn play_start(
    mut play_start_event_reader: EventReader<PlayStart>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
) {
    for event in play_start_event_reader.read() {
        println!("カードプレイに移行します");
        select_hand_event_writer.send(SelectHand);
    }
}

pub fn select_hand(
    mut select_hand_event_reader: EventReader<SelectHand>,
    hand: ResMut<Player1Hand>,
    szone: ResMut<Player1SZone>,
    mut check_played_card_type_event_writer: EventWriter<CheckPlayedCardType>,
    query: Query<(Entity, &Card, &SpinCondition), With<Player1>>,
    query_option: Query<(Entity, &Card, Option<&Fighter>, Option<&Tactics>), With<Player1>>,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
) {
    for event in select_hand_event_reader.read() {
        let total_energy_in_szone_respin: u32 = szone
            .cards
            .iter()
            .filter_map(|entity| {
                if let Ok((_, card, spincondition)) = query.get(*entity) {
                    if let SpinCondition::ReSpin = spincondition {
                        Some(card.energy) // リスピン中のカードのエナジーを取得
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum();
        if hand.cards.is_empty() {
            println!("手札にカードがありません");
            main_phase_management_event_writer.send(MainPhaseManagement);
            return;
        }
        // 合計エナジーを表示

        println!(
            "現在のSゾーンのリスピン状態のカードの合計エナジー: {}",
            total_energy_in_szone_respin
        );

        // 手札のカードの情報を番号付きで表示
        println!("手札のカード:[エナジー(タクティクスの必要エナジーは表示されません)]");
        for (index, &entity) in hand.cards.iter().enumerate() {
            if let Ok((_, card, _)) = query.get(entity) {
                println!("{}: {} [{}]", index + 1, card.name, card.energy);
            }
        }

        // ユーザーからの入力を受け取る
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(selected_index) = input.trim().parse::<usize>() {
            if selected_index > 0 && selected_index <= hand.cards.len() {
                let selected_entity = hand.cards[selected_index - 1];

                // 選択されたカードのエナジーが合計エナジー以下であることを確認
                if let Ok((_, card, fighter_option, tactics_option)) =
                    query_option.get(selected_entity)
                {
                    if let Some(tactics) = tactics_option {
                        println!("{} はタクティクスです。", card.name);

                        // タクティクスの場合、エナジーの検討
                        if tactics.required_energy <= total_energy_in_szone_respin {
                            println!("{} を選択しました。", card.name);
                            check_played_card_type_event_writer.send(CheckPlayedCardType {
                                played_card_id: selected_entity,
                            });
                        } else {
                            println!(
                                "選択したタクティクスの必要エナジーが合計エナジーを超えています。再度選択してください。"
                            );
                            main_phase_management_event_writer.send(MainPhaseManagement);
                        }
                    } else {
                        // ファイターの場合の処理
                        if card.energy <= total_energy_in_szone_respin {
                            println!("{} を選択しました。", card.name);
                            check_played_card_type_event_writer.send(CheckPlayedCardType {
                                played_card_id: selected_entity,
                            });
                        } else {
                            println!(
                                "選択したファイターのエナジーが合計エナジーを超えています。再度選択してください。"
                            );
                            main_phase_management_event_writer.send(MainPhaseManagement);
                        }
                    }
                } else {
                    println!("無効な選択です。");
                    main_phase_management_event_writer.send(MainPhaseManagement);
                }
            } else {
                println!("無効な選択です。");
                main_phase_management_event_writer.send(MainPhaseManagement);
            }
        } else {
            println!("有効な番号を入力してください。");
        }
    }
}

pub fn select_hand_opponent(
    mut select_hand_event_reader: EventReader<SelectHand>,
    hand: ResMut<Player2Hand>,
    szone: ResMut<Player2SZone>,
    mut check_played_card_type_event_writer: EventWriter<CheckPlayedCardType>,
    query: Query<(Entity, &Card, &SpinCondition), With<Player2>>,
    query_option: Query<(Entity, &Card, Option<&Fighter>, Option<&Tactics>), With<Player2>>,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
) {
    for event in select_hand_event_reader.read() {
        let total_energy_in_szone_respin: u32 = szone
            .cards
            .iter()
            .filter_map(|entity| {
                if let Ok((_, card, spincondition)) = query.get(*entity) {
                    if let SpinCondition::ReSpin = spincondition {
                        Some(card.energy) // リスピン中のカードのエナジーを取得
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum();
        if hand.cards.is_empty() {
            println!("手札にカードがありません");
            main_phase_management_event_writer.send(MainPhaseManagement);
            return;
        }
        // 合計エナジーを表示
        println!(
            "現在のSゾーンのリスピン状態のカードの合計エナジー: {}",
            total_energy_in_szone_respin
        );

        // 手札のカードの情報を番号付きで表示
        println!("手札のカード:[エナジー(タクティクスの必要エナジーは表示されません)]");
        for (index, &entity) in hand.cards.iter().enumerate() {
            if let Ok((_, card, _)) = query.get(entity) {
                println!("{}: {} [{}]", index + 1, card.name, card.energy);
            }
        }

        // ユーザーからの入力を受け取る
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(selected_index) = input.trim().parse::<usize>() {
            if selected_index > 0 && selected_index <= hand.cards.len() {
                let selected_entity = hand.cards[selected_index - 1];

                // 選択されたカードのエナジーが合計エナジー以下であることを確認
                if let Ok((_, card, fighter_option, tactics_option)) =
                    query_option.get(selected_entity)
                {
                    if let Some(tactics) = tactics_option {
                        println!("{} はタクティクスです。", card.name);

                        // タクティクスの場合、エナジーの検討
                        if tactics.required_energy <= total_energy_in_szone_respin {
                            println!("{} を選択しました。", card.name);
                            check_played_card_type_event_writer.send(CheckPlayedCardType {
                                played_card_id: selected_entity,
                            });
                        } else {
                            println!(
                                "選択したタクティクスの必要エナジーが合計エナジーを超えています。再度選択してください。"
                            );
                            main_phase_management_event_writer.send(MainPhaseManagement);
                        }
                    } else {
                        // ファイターの場合の処理
                        if card.energy <= total_energy_in_szone_respin {
                            println!("{} を選択しました。", card.name);
                            check_played_card_type_event_writer.send(CheckPlayedCardType {
                                played_card_id: selected_entity,
                            });
                        } else {
                            println!(
                                "選択したファイターのエナジーが合計エナジーを超えています。再度選択してください。"
                            );
                            main_phase_management_event_writer.send(MainPhaseManagement);
                        }
                    }
                } else {
                    println!("無効な選択です。");
                    main_phase_management_event_writer.send(MainPhaseManagement);
                }
            } else {
                println!("無効な選択です。");
                main_phase_management_event_writer.send(MainPhaseManagement);
            }
        } else {
            println!("有効な番号を入力してください。");
        }
    }
}

pub fn check_played_card_type(
    mut check_played_card_type_event_reader: EventReader<CheckPlayedCardType>,
    mut play_fighter_spin_szone_event_writer: EventWriter<PlayFighterSpinSzone>,
    mut play_zero_energy_fighter_event_writer: EventWriter<PlayZeroEnergyFighterFromHand>,
    mut play_tactics_spin_szone_event_writer: EventWriter<PlayTacticsSpinSzone>,
    query: Query<(Entity, &Card, Option<&Fighter>, Option<&Tactics>)>,
) {
    for event in check_played_card_type_event_reader.read() {
        if let Ok((_, card, fighter_option, tactics_option)) = query.get(event.played_card_id) {
            if let Some(fighter) = fighter_option {
                println!(
                    "{} はファイターでエナジーは{}です。",
                    card.name, card.energy
                );
                if card.energy == 0 {
                    play_zero_energy_fighter_event_writer.send(PlayZeroEnergyFighterFromHand {
                        played_card_id: event.played_card_id,
                    });
                } else {
                    play_fighter_spin_szone_event_writer.send(PlayFighterSpinSzone {
                        played_card_id: event.played_card_id,
                    });
                }

                // ファイターに対する処理をここに追加
            } else if let Some(tactics) = tactics_option {
                println!(
                    "{} はタクティクスで必要エナジーは{}です。",
                    card.name, tactics.required_energy
                );
                play_tactics_spin_szone_event_writer.send(PlayTacticsSpinSzone {
                    played_card_id: event.played_card_id,
                });
                // タクティクスに対する処理をここに追加
            } else {
                println!(
                    "{:?} はファイターでもタクティクスでもありません。",
                    event.played_card_id
                );
            }
        } else {
            println!("{:?} は無効なエンティティです。", event.played_card_id);
        }
    }
}

pub fn play_fighter_spin_szone(
    szone: ResMut<Player1SZone>,
    mut play_fighter_spin_szone_event_reader: EventReader<PlayFighterSpinSzone>,
    query: Query<(Entity, &Card, &SpinCondition), With<Player1>>,
    mut commands: Commands,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
    mut play_fighter_from_hand_event_writer: EventWriter<PlayFighterFromHand>,
) {
    for event in play_fighter_spin_szone_event_reader.read() {
        let mut available_cards = Vec::new();
        let mut spin_targets: Vec<Entity> = Vec::new();
        let mut used_energy_for_fighter = 0;

        // played_cardのエナジーを取得
        let energy: u32 = if let Ok((_, card, _)) = query.get(event.played_card_id) {
            card.energy
        } else {
            // played_cardが無効な場合の処理
            println!("無効なplayed_cardです。");
            continue; // 次のイベントに進む
        };

        // Sゾーンのカードをリストアップ
        for &entity in &szone.cards {
            if let Ok((_, card, spincondition)) = query.get(entity) {
                if let SpinCondition::ReSpin = spincondition {
                    available_cards.push(entity);
                    println!(
                        "{}: {} (Sゾーン)[{}]",
                        available_cards.len(),
                        card.name,
                        card.energy
                    );
                }
            }
        }

        // played_cardのエナジーが0の場合の処理

        // 選択ループ
        while used_energy_for_fighter < energy {
            println!(
                "スピンさせるカードを選択してください (1-{}) 0:やりなおす",
                available_cards.len()
            );

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(selected_index) = input.trim().parse::<usize>() {
                if selected_index == 0 {
                    println!("選択をキャンセルし、メインフェーズに戻ります。");
                    main_phase_management_event_writer.send(MainPhaseManagement);
                    return; // ループを抜ける
                }
                if selected_index > 0 && selected_index <= available_cards.len() {
                    let selected_entity = available_cards[selected_index - 1];

                    // すでに選択されたカードか確認
                    if spin_targets.contains(&selected_entity) {
                        println!("このカードはすでに選択されています。");
                        continue; // 再選択を促す
                    }

                    if let Ok((_, card, _)) = query.get(selected_entity) {
                        // スピンさせるカードのリストを作る
                        spin_targets.push(selected_entity);
                        used_energy_for_fighter += card.energy;
                        println!(
                            "{} をスピンさせる対象として追加しました (現在の合計エナジー: {})",
                            card.name, used_energy_for_fighter
                        );
                    }
                } else {
                    println!("無効な選択です。");
                }
            } else {
                println!("有効な番号を入力してください。");
            }
        }

        println!(
            "合計エナジーが達成されました。スピン対象: {:?}",
            spin_targets
        );
        for target in &spin_targets {
            if let Ok((_, card, _)) = query.get(*target) {
                println!("- {} [エナジー: {}]", card.name, card.energy);
            }
        }

        let choice = check_the_will_of_spinning_cards();
        match choice {
            1 => {
                //エレメントについての言及は全くしていないことに注意！！

                println!("スピンさせました。");
                for target in &spin_targets {
                    commands.entity(*target).insert(SpinCondition::Spin);
                }
                play_fighter_from_hand_event_writer.send(PlayFighterFromHand {
                    played_card_id: event.played_card_id,
                    spin_cards_list: spin_targets,
                });
            }

            2 => {
                println!("やり直します");
                select_hand_event_writer.send(SelectHand);
            }
            _ => {}
        }

        // 今後の処理をここに追加（例: スピン実行など）
    }
}

pub fn play_fighter_spin_szone_opponent(
    szone: ResMut<Player2SZone>,
    mut play_fighter_spin_szone_event_reader: EventReader<PlayFighterSpinSzone>,
    query: Query<(Entity, &Card, &SpinCondition), With<Player2>>,
    mut commands: Commands,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
    mut play_fighter_from_hand_event_writer: EventWriter<PlayFighterFromHand>,
) {
    for event in play_fighter_spin_szone_event_reader.read() {
        let mut available_cards = Vec::new();
        let mut spin_targets: Vec<Entity> = Vec::new();
        let mut used_energy_for_fighter = 0;

        // played_cardのエナジーを取得
        let energy: u32 = if let Ok((_, card, _)) = query.get(event.played_card_id) {
            card.energy
        } else {
            // played_cardが無効な場合の処理
            println!("無効なplayed_cardです。");
            continue; // 次のイベントに進む
        };

        // Sゾーンのカードをリストアップ
        for &entity in &szone.cards {
            if let Ok((_, card, spincondition)) = query.get(entity) {
                if let SpinCondition::ReSpin = spincondition {
                    available_cards.push(entity);
                    println!(
                        "{}: {} (Sゾーン)[{}]",
                        available_cards.len(),
                        card.name,
                        card.energy
                    );
                }
            }
        }

        // played_cardのエナジーが0の場合の処理

        // 選択ループ
        while used_energy_for_fighter < energy {
            println!(
                "スピンさせるカードを選択してください (1-{}) 0:やりなおす",
                available_cards.len()
            );

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(selected_index) = input.trim().parse::<usize>() {
                if selected_index == 0 {
                    println!("選択をキャンセルし、メインフェーズに戻ります。");
                    main_phase_management_event_writer.send(MainPhaseManagement);
                    return; // ループを抜ける
                }
                if selected_index > 0 && selected_index <= available_cards.len() {
                    let selected_entity = available_cards[selected_index - 1];

                    // すでに選択されたカードか確認
                    if spin_targets.contains(&selected_entity) {
                        println!("このカードはすでに選択されています。");
                        continue; // 再選択を促す
                    }

                    if let Ok((_, card, _)) = query.get(selected_entity) {
                        // スピンさせるカードのリストを作る
                        spin_targets.push(selected_entity);
                        used_energy_for_fighter += card.energy;
                        println!(
                            "{} をスピンさせる対象として追加しました (現在の合計エナジー: {})",
                            card.name, used_energy_for_fighter
                        );
                    }
                } else {
                    println!("無効な選択です。");
                }
            } else {
                println!("有効な番号を入力してください。");
            }
        }

        println!(
            "合計エナジーが達成されました。スピン対象: {:?}",
            spin_targets
        );
        for target in &spin_targets {
            if let Ok((_, card, _)) = query.get(*target) {
                println!("- {} [エナジー: {}]", card.name, card.energy);
            }
        }

        let choice = check_the_will_of_spinning_cards();
        match choice {
            1 => {
                //エレメントについての言及は全くしていないことに注意！！

                println!("スピンさせました。");
                for target in &spin_targets {
                    commands.entity(*target).insert(SpinCondition::Spin);
                }
                play_fighter_from_hand_event_writer.send(PlayFighterFromHand {
                    played_card_id: event.played_card_id,
                    spin_cards_list: spin_targets,
                });
            }

            2 => {
                println!("やり直します");
                select_hand_event_writer.send(SelectHand);
            }
            _ => {}
        }

        // 今後の処理をここに追加（例: スピン実行など）
    }
}

pub fn play_tactics_spin_szone(
    szone: ResMut<Player1SZone>,
    mut play_tactics_spin_szone_event_reader: EventReader<PlayTacticsSpinSzone>,
    query: Query<(Entity, &Card, &SpinCondition, &Tactics), With<Player1>>,
    query2: Query<(Entity, &Card, &SpinCondition), With<Player1>>,
    mut commands: Commands,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
    mut play_tactics_from_hand_event_writer: EventWriter<PlayTacticsFromHand>,
    mut energy_adjustment_event_writer: EventWriter<EnergyAdjustment>,
) {
    for event in play_tactics_spin_szone_event_reader.read() {
        let mut available_cards = Vec::new();
        let mut spin_targets: Vec<Entity> = Vec::new();
        let mut used_energy_for_tactics = 0;
        //Sゾーンのカードのエナジーの合計をスピンコンディション関係なく取得。
        let mut total_energy_in_szone: u32 = szone
            .cards
            .iter()
            .filter_map(|entity| {
                if let Ok((_, card, _)) = query2.get(*entity) {
                    Some(card.energy) // リスピン中のカードのエナジーを取得
                } else {
                    None
                }
            })
            .sum();

        // played_cardのエナジーを取得
        let energy: u32 = if let Ok((_, card, _, tactics)) = query.get(event.played_card_id) {
            tactics.required_energy
        } else {
            // played_cardが無効な場合の処理
            println!("無効なplayed_cardです。");
            continue; // 次のイベントに進む
        };

        // Sゾーンのカードをリストアップ
        for &entity in &szone.cards {
            if let Ok((_, card, spincondition)) = query2.get(entity) {
                if let SpinCondition::ReSpin = spincondition {
                    available_cards.push(entity);
                    println!(
                        "{}: {} (Sゾーン)[{}]",
                        available_cards.len(),
                        card.name,
                        card.energy
                    );
                }
            }
        }

        // played_cardのエナジーが0の場合の処理
        if energy == 0 {
            println!("played_cardのエナジーが0です。");
            // エナジーが0の場合の独自のロジックをここに追加
            continue; // または、特定の処理を実行
        }

        // 選択ループ
        while used_energy_for_tactics < energy {
            println!(
                "スピンさせるカードを選択してください (1-{}) 0:やりなおす",
                available_cards.len()
            );

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(selected_index) = input.trim().parse::<usize>() {
                if selected_index == 0 {
                    println!("選択をキャンセルし、メインフェーズに戻ります。");
                    main_phase_management_event_writer.send(MainPhaseManagement);
                    return; // ループを抜ける
                }
                if selected_index > 0 && selected_index <= available_cards.len() {
                    let selected_entity = available_cards[selected_index - 1];

                    // すでに選択されたカードか確認
                    if spin_targets.contains(&selected_entity) {
                        println!("このカードはすでに選択されています。");
                        continue; // 再選択を促す
                    }

                    if let Ok((_, card, _)) = query2.get(selected_entity) {
                        // スピンさせるカードのリストを作る
                        spin_targets.push(selected_entity);
                        used_energy_for_tactics += card.energy;
                        println!(
                            "{} をスピンさせる対象として追加しました (現在の合計使用エナジー: {})",
                            card.name, used_energy_for_tactics
                        );
                    }
                } else {
                    println!("無効な選択です。");
                }
            } else {
                println!("有効な番号を入力してください。");
            }
        }

        println!(
            "合計エナジーが達成されました。スピン対象: {:?}",
            spin_targets
        );

        if let Ok((_, card, _)) = query2.get(event.played_card_id) {
            total_energy_in_szone += card.energy;
            println!(
                "プレイするタクティクスも含めたエナジーは{}です。",
                total_energy_in_szone
            );
        }
        println!("スピン予定のカード:");
        for target in &spin_targets {
            if let Ok((_, card, _)) = query2.get(*target) {
                println!("- {} [エナジー: {}]", card.name, card.energy);
            }
        }
        if total_energy_in_szone > 10 {
            println!("エナジーの調整が発生します。");

            let choice = check_the_will_of_spinning_cards();
            match choice {
                1 => {
                    //エレメントについての言及は全くしていないことに注意！！

                    println!("スピンさせました。");
                    for target in &spin_targets {
                        commands.entity(*target).insert(SpinCondition::Spin);
                    }
                    let penalty = total_energy_in_szone - 10;
                    println!("送信するペナルティエナジー: {}", penalty);
                    energy_adjustment_event_writer.send(EnergyAdjustment {
                        played_card_id: event.played_card_id,
                        spin_cards_list: spin_targets,
                        penalty_energy: penalty,
                    });
                }

                2 => {
                    println!("やり直します");
                    select_hand_event_writer.send(SelectHand);
                }
                _ => {}
            }
        } else {
            let choice = check_the_will_of_spinning_cards();
            match choice {
                1 => {
                    //エレメントについての言及は全くしていないことに注意！！

                    println!("スピンさせました。");
                    for target in &spin_targets {
                        commands.entity(*target).insert(SpinCondition::Spin);
                    }
                    play_tactics_from_hand_event_writer.send(PlayTacticsFromHand {
                        played_card_id: event.played_card_id,
                        spin_cards_list: spin_targets,
                    });
                }

                2 => {
                    println!("やり直します");
                    select_hand_event_writer.send(SelectHand);
                }
                _ => {}
            }
        }
    }
}

pub fn play_tactics_spin_szone_opponent(
    szone: ResMut<Player2SZone>,
    mut play_tactics_spin_szone_event_reader: EventReader<PlayTacticsSpinSzone>,
    query: Query<(Entity, &Card, &SpinCondition, &Tactics), With<Player2>>,
    query2: Query<(Entity, &Card, &SpinCondition), With<Player2>>,
    mut commands: Commands,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
    mut play_tactics_from_hand_event_writer: EventWriter<PlayTacticsFromHand>,
    mut energy_adjustment_event_writer: EventWriter<EnergyAdjustment>,
) {
    for event in play_tactics_spin_szone_event_reader.read() {
        let mut available_cards = Vec::new();
        let mut spin_targets: Vec<Entity> = Vec::new();
        let mut used_energy_for_tactics = 0;
        //Sゾーンのカードのエナジーの合計をスピンコンディション関係なく取得。
        let mut total_energy_in_szone: u32 = szone
            .cards
            .iter()
            .filter_map(|entity| {
                if let Ok((_, card, _)) = query2.get(*entity) {
                    Some(card.energy) // リスピン中のカードのエナジーを取得
                } else {
                    None
                }
            })
            .sum();

        // played_cardのエナジーを取得
        let energy: u32 = if let Ok((_, card, _, tactics)) = query.get(event.played_card_id) {
            tactics.required_energy
        } else {
            // played_cardが無効な場合の処理
            println!("無効なplayed_cardです。");
            continue; // 次のイベントに進む
        };

        // Sゾーンのカードをリストアップ
        for &entity in &szone.cards {
            if let Ok((_, card, spincondition)) = query2.get(entity) {
                if let SpinCondition::ReSpin = spincondition {
                    available_cards.push(entity);
                    println!(
                        "{}: {} (Sゾーン)[{}]",
                        available_cards.len(),
                        card.name,
                        card.energy
                    );
                }
            }
        }

        // played_cardのエナジーが0の場合の処理
        if energy == 0 {
            println!("played_cardのエナジーが0です。");
            // エナジーが0の場合の独自のロジックをここに追加
            continue; // または、特定の処理を実行
        }

        // 選択ループ
        while used_energy_for_tactics < energy {
            println!(
                "スピンさせるカードを選択してください (1-{}) 0:やりなおす",
                available_cards.len()
            );

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(selected_index) = input.trim().parse::<usize>() {
                if selected_index == 0 {
                    println!("選択をキャンセルし、メインフェーズに戻ります。");
                    main_phase_management_event_writer.send(MainPhaseManagement);
                    return; // ループを抜ける
                }
                if selected_index > 0 && selected_index <= available_cards.len() {
                    let selected_entity = available_cards[selected_index - 1];

                    // すでに選択されたカードか確認
                    if spin_targets.contains(&selected_entity) {
                        println!("このカードはすでに選択されています。");
                        continue; // 再選択を促す
                    }

                    if let Ok((_, card, _)) = query2.get(selected_entity) {
                        // スピンさせるカードのリストを作る
                        spin_targets.push(selected_entity);
                        used_energy_for_tactics += card.energy;
                        println!(
                            "{} をスピンさせる対象として追加しました (現在の合計使用エナジー: {})",
                            card.name, used_energy_for_tactics
                        );
                    }
                } else {
                    println!("無効な選択です。");
                }
            } else {
                println!("有効な番号を入力してください。");
            }
        }

        println!(
            "合計エナジーが達成されました。スピン対象: {:?}",
            spin_targets
        );

        if let Ok((_, card, _)) = query2.get(event.played_card_id) {
            total_energy_in_szone += card.energy;
            println!(
                "プレイするタクティクスも含めたエナジーは{}です。",
                total_energy_in_szone
            );
        }
        println!("スピン予定のカード:");
        for target in &spin_targets {
            if let Ok((_, card, _)) = query2.get(*target) {
                println!("- {} [エナジー: {}]", card.name, card.energy);
            }
        }
        if total_energy_in_szone > 10 {
            println!("エナジーの調整が発生します。");

            let choice = check_the_will_of_spinning_cards();
            match choice {
                1 => {
                    //エレメントについての言及は全くしていないことに注意！！

                    println!("スピンさせました。");
                    for target in &spin_targets {
                        commands.entity(*target).insert(SpinCondition::Spin);
                    }
                    let penalty = total_energy_in_szone - 10;
                    println!("送信するペナルティエナジー: {}", penalty);
                    energy_adjustment_event_writer.send(EnergyAdjustment {
                        played_card_id: event.played_card_id,
                        spin_cards_list: spin_targets,
                        penalty_energy: penalty,
                    });
                }

                2 => {
                    println!("やり直します");
                    select_hand_event_writer.send(SelectHand);
                }
                _ => {}
            }
        } else {
            let choice = check_the_will_of_spinning_cards();
            match choice {
                1 => {
                    //エレメントについての言及は全くしていないことに注意！！

                    println!("スピンさせました。");
                    for target in &spin_targets {
                        commands.entity(*target).insert(SpinCondition::Spin);
                    }
                    play_tactics_from_hand_event_writer.send(PlayTacticsFromHand {
                        played_card_id: event.played_card_id,
                        spin_cards_list: spin_targets,
                    });
                }

                2 => {
                    println!("やり直します");
                    select_hand_event_writer.send(SelectHand);
                }
                _ => {}
            }
        }
    }
}

pub fn check_the_will_of_spinning_cards() -> i32 {
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

pub fn play_zero_energy_fighter(
    mut play_zero_energy_fighter_event_reader: EventReader<PlayZeroEnergyFighterFromHand>,
    mut commands: Commands,
    mut play_fighter_finish_event_writer: EventWriter<PlayFighterFinish>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
    query: Query<(Entity, &Card, &Fighter)>,
) {
    for event in play_zero_energy_fighter_event_reader.read() {
        if let Ok((_, card, fighter)) = query.get(event.played_card_id) {
            println!("{}を手札から召喚しますか？", card.name);

            let choice = check_the_will_of_spinning_cards();
            match choice {
                1 => {
                    //エレメントについての言及は全くしていないことに注意！！
                    commands
                        .entity(event.played_card_id)
                        .insert(Location::FZone);
                    // Fighter コンポーネントを新しい summoned_sick で更新
                    commands.entity(event.played_card_id).insert(Fighter {
                        summoned_sick: true,
                        ..*fighter
                    }); // power はそのまま
                    println!("{}を手札から召喚しました", card.name);
                    println!("{:?}は召喚酔いをしています", event.played_card_id);
                    play_fighter_finish_event_writer.send(PlayFighterFinish {
                        played_card_id: event.played_card_id,
                    });
                }

                2 => {
                    println!("やり直します");
                    select_hand_event_writer.send(SelectHand);
                }
                _ => {}
            }
        }
    }
}

pub fn play_fighter_from_hand(
    mut play_fighter_from_hand_event_reader: EventReader<PlayFighterFromHand>,
    mut commands: Commands,
    mut play_fighter_finish_event_writer: EventWriter<PlayFighterFinish>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
    query: Query<(Entity, &Card, &Fighter)>,
) {
    for event in play_fighter_from_hand_event_reader.read() {
        if let Ok((_, card, fighter)) = query.get(event.played_card_id) {
            println!("{}を手札から召喚しますか？", card.name);

            let choice = check_the_will_of_spinning_cards();
            match choice {
                1 => {
                    //エレメントについての言及は全くしていないことに注意！！
                    commands
                        .entity(event.played_card_id)
                        .insert(Location::FZone);
                    // Fighter コンポーネントを新しい summoned_sick で更新
                    commands.entity(event.played_card_id).insert(Fighter {
                        summoned_sick: true,
                        ..*fighter
                    }); // power はそのまま
                    println!("{}を手札から召喚しました", card.name);
                    println!("{:?}は召喚酔いをしています", event.played_card_id);
                    play_fighter_finish_event_writer.send(PlayFighterFinish {
                        played_card_id: event.played_card_id,
                    });
                }

                2 => {
                    println!("やりなおします");
                    for target in &event.spin_cards_list {
                        commands.entity(*target).insert(SpinCondition::ReSpin);
                    }
                    println!("スピンしたカードを元に戻しました。");
                    select_hand_event_writer.send(SelectHand);
                }
                _ => {}
            }
        }
    }
}

pub fn play_tactics_from_hand(
    mut play_tactics_from_hand_event_reader: EventReader<PlayTacticsFromHand>,
    mut commands: Commands,
    mut play_tactics_finish_event_writer: EventWriter<PlayTacticsFinish>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
    query: Query<(Entity, &Card)>,
) {
    for event in play_tactics_from_hand_event_reader.read() {
        if let Ok((_, card)) = query.get(event.played_card_id) {
            println!("{}を手札から発動しますか？", card.name);
            let choice = check_the_will_of_spinning_cards();
            match choice {
                1 => {
                    //エレメントについての言及は全くしていないことに注意！！
                    commands
                        .entity(event.played_card_id)
                        .insert(Location::SZone);
                    println!("{}を手札から発動しました", card.name);
                    play_tactics_finish_event_writer.send(PlayTacticsFinish {
                        played_card_id: event.played_card_id,
                    });
                }

                2 => {
                    println!("やりなおします");
                    for target in &event.spin_cards_list {
                        commands.entity(*target).insert(SpinCondition::ReSpin);
                    }
                    println!("スピンしたカードを元に戻しました。");
                    select_hand_event_writer.send(SelectHand);
                }
                _ => {}
            }
        }
    }
}

pub fn play_fighter_finish(
    mut play_fighter_finish_event_reader: EventReader<PlayFighterFinish>,
    mut situation_log_event_writer: EventWriter<SituationLog>,
) {
    for event in play_fighter_finish_event_reader.read() {
        situation_log_event_writer.send(SituationLog);
    }
}

pub fn play_tactics_finish(
    mut play_tactics_finish_event_reader: EventReader<PlayTacticsFinish>,
    mut situation_log_event_writer: EventWriter<SituationLog>,
) {
    for event in play_tactics_finish_event_reader.read() {
        situation_log_event_writer.send(SituationLog);
    }
}

pub fn energy_adjustment(
    mut energy_adjustment_event_reader: EventReader<EnergyAdjustment>,
    szone: ResMut<Player1SZone>,
    query: Query<(Entity, &Card), (With<Player1>, Without<FirstEnergy>)>,
    mut commands: Commands,
    mut in_xzone_cards: ResMut<FacedownXZoneCards>, 
    mut tactics_with_penalty_event_writer: EventWriter<PlayTacticsWithPenaltyFromHand>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
) {
    for event in energy_adjustment_event_reader.read() {
        let mut selected_cards: Vec<Entity> = Vec::new();
        let mut total_energy_for_penalty: u32 = 0;
        let penalty_energy = event.penalty_energy;

        // Sゾーンのカードを取得
        let available_cards: Vec<(Entity, &Card)> = szone
            .cards
            .iter()
            .filter_map(|&entity| query.get(entity).ok())
            .collect();

        // ペナルティエナジー以上のエナジーを持つカードをチェック
        let high_energy_cards: Vec<(Entity, &Card)> = available_cards
            .iter()
            .filter(|(_, card)| card.energy >= penalty_energy)
            .cloned()
            .collect();

        // ペナルティエナジー以上のカードがある場合
        if !high_energy_cards.is_empty() {
            println!("ペナルティエナジー以上のカードがあります。選択可能なカード:");
            for (index, (entity, card)) in high_energy_cards.iter().enumerate() {
                println!("{}: {} [エナジー: {}]", index + 1, card.name, card.energy);
            }

            // ユーザーにカードを選ばせる
            loop {
                // 無効な選択時に再入力を促すループ
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                if let Ok(selected_index) = input.trim().parse::<usize>() {
                    if selected_index > 0 && selected_index <= high_energy_cards.len() {
                        let selected_entity = high_energy_cards[selected_index - 1].0;
                        selected_cards.push(selected_entity);
                        if let Ok((_, card)) = query.get(selected_entity) {
                            total_energy_for_penalty += card.energy;
                            println!("選択したカード: {} [エナジー: {}]", card.name, card.energy);
                        }
                        break; // 有効な選択がされたらループを抜ける
                    } else {
                        println!("無効な選択です。再度選択してください。");
                    }
                } else {
                    println!("有効な番号を入力してください。再度選択してください。");
                }
            }
        } else {
            // ペナルティエナジー以上のカードがない場合
            println!("ペナルティエナジー以上のカードはありません。全てのカードを表示します。");
            for (index, (entity, card)) in available_cards.iter().enumerate() {
                println!("{}: {} [エナジー: {}]", index + 1, card.name, card.energy);
            }

            // ユーザーにカードを選ばせる
            while total_energy_for_penalty <= penalty_energy {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                if let Ok(selected_index) = input.trim().parse::<usize>() {
                    if selected_index > 0 && selected_index <= available_cards.len() {
                        let selected_entity = available_cards[selected_index - 1].0;

                        // 既に選択されたカードか確認
                        if !selected_cards.contains(&selected_entity) {
                            selected_cards.push(selected_entity);
                            if let Ok((_, card)) = query.get(selected_entity) {
                                total_energy_for_penalty += card.energy;
                                println!(
                                    "選択したカード: {} [エナジー: {}]",
                                    card.name, card.energy
                                );
                            }
                        } else {
                            println!("このカードは既に選択されています。");
                        }
                    } else {
                        println!("無効な選択です。再度選択してください。");
                    }
                } else {
                    println!("有効な番号を入力してください。再度選択してください。");
                }

                // 合計エナジーとペナルティエナジーの差を確認
                println!(
                    "現在の合計エナジー: {}, ペナルティエナジー: {}",
                    total_energy_for_penalty, penalty_energy
                );
                if total_energy_for_penalty > penalty_energy {
                    break; // 合計エナジーがペナルティエナジーを上回ったらループを終了
                }

                // 再度ペナルティエナジー以上のカードがあるか確認
                let high_energy_cards: Vec<(Entity, &Card)> = available_cards
                    .iter()
                    .filter(|(_, card)| card.energy >= penalty_energy - total_energy_for_penalty)
                    .cloned()
                    .collect();

                if !high_energy_cards.is_empty() {
                    println!("ペナルティエナジーの差以上のカードがあります。選択可能なカード:");
                    for (index, (entity, card)) in high_energy_cards.iter().enumerate() {
                        println!("{}: {} [エナジー: {}]", index + 1, card.name, card.energy);
                    }
                }
            }
        }

        // 最終的に選んだカードのリストを表示
        println!("最終的に選ばれたカード:");
        for entity in selected_cards.clone() {
            if let Ok((_, card)) = query.get(entity) {
                println!("- {} [エナジー: {}]", card.name, card.energy);
            }
        }

        println!("ペナルティを受け入れますか？");
        let choice = check_the_will_of_spinning_cards();
        match choice {
            1 => {
                for entity in selected_cards.clone() {
                    commands.entity(entity).insert(Location::XZone);
                    commands.entity(entity).insert(SpinCondition::ReSpin);
                    commands.entity(entity).insert(FaceCondition::Facedown);
                    in_xzone_cards.cards.push(entity);
                    println!("{:?}はペナルティでXゾーンに裏側で置かれました。", entity)

                }

                tactics_with_penalty_event_writer.send(PlayTacticsWithPenaltyFromHand {
                    played_card_id: event.played_card_id,
                    spin_cards_list: event.spin_cards_list.clone(),
                    penalty_card_list: selected_cards.clone(),
                });
            }

            2 => {
                println!("やりなおします");
                for target in &event.spin_cards_list {
                    commands.entity(*target).insert(SpinCondition::ReSpin);
                }
                println!("スピンしたカードを元に戻しました。");
                select_hand_event_writer.send(SelectHand);
            }
            _ => {}
        }
        // 意思決定関数を入れた
    }
}

pub fn energy_adjustment_opponent(
    mut energy_adjustment_event_reader: EventReader<EnergyAdjustment>,
    szone: ResMut<Player2SZone>,
    query: Query<(Entity, &Card), (With<Player2>, Without<FirstEnergy>)>,
    mut commands: Commands,
    mut in_xzone_cards: ResMut<FacedownXZoneCards>, 
    mut tactics_with_penalty_event_writer: EventWriter<PlayTacticsWithPenaltyFromHand>,
    mut select_hand_event_writer: EventWriter<SelectHand>,
) {
    for event in energy_adjustment_event_reader.read() {
        let mut selected_cards: Vec<Entity> = Vec::new();
        let mut total_energy_for_penalty: u32 = 0;
        let penalty_energy = event.penalty_energy;

        // Sゾーンのカードを取得
        let available_cards: Vec<(Entity, &Card)> = szone
            .cards
            .iter()
            .filter_map(|&entity| query.get(entity).ok())
            .collect();

        // ペナルティエナジー以上のエナジーを持つカードをチェック
        let high_energy_cards: Vec<(Entity, &Card)> = available_cards
            .iter()
            .filter(|(_, card)| card.energy >= penalty_energy)
            .cloned()
            .collect();

        // ペナルティエナジー以上のカードがある場合
        if !high_energy_cards.is_empty() {
            println!("ペナルティエナジー以上のカードがあります。選択可能なカード:");
            for (index, (entity, card)) in high_energy_cards.iter().enumerate() {
                println!("{}: {} [エナジー: {}]", index + 1, card.name, card.energy);
            }

            // ユーザーにカードを選ばせる
            loop {
                // 無効な選択時に再入力を促すループ
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                if let Ok(selected_index) = input.trim().parse::<usize>() {
                    if selected_index > 0 && selected_index <= high_energy_cards.len() {
                        let selected_entity = high_energy_cards[selected_index - 1].0;
                        selected_cards.push(selected_entity);
                        if let Ok((_, card)) = query.get(selected_entity) {
                            total_energy_for_penalty += card.energy;
                            println!("選択したカード: {} [エナジー: {}]", card.name, card.energy);
                        }
                        break; // 有効な選択がされたらループを抜ける
                    } else {
                        println!("無効な選択です。再度選択してください。");
                    }
                } else {
                    println!("有効な番号を入力してください。再度選択してください。");
                }
            }
        } else {
            // ペナルティエナジー以上のカードがない場合
            println!("ペナルティエナジー以上のカードはありません。全てのカードを表示します。");
            for (index, (entity, card)) in available_cards.iter().enumerate() {
                println!("{}: {} [エナジー: {}]", index + 1, card.name, card.energy);
            }

            // ユーザーにカードを選ばせる
            while total_energy_for_penalty <= penalty_energy {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                if let Ok(selected_index) = input.trim().parse::<usize>() {
                    if selected_index > 0 && selected_index <= available_cards.len() {
                        let selected_entity = available_cards[selected_index - 1].0;

                        // 既に選択されたカードか確認
                        if !selected_cards.contains(&selected_entity) {
                            selected_cards.push(selected_entity);
                            if let Ok((_, card)) = query.get(selected_entity) {
                                total_energy_for_penalty += card.energy;
                                println!(
                                    "選択したカード: {} [エナジー: {}]",
                                    card.name, card.energy
                                );
                            }
                        } else {
                            println!("このカードは既に選択されています。");
                        }
                    } else {
                        println!("無効な選択です。再度選択してください。");
                    }
                } else {
                    println!("有効な番号を入力してください。再度選択してください。");
                }

                // 合計エナジーとペナルティエナジーの差を確認
                println!(
                    "現在の合計エナジー: {}, ペナルティエナジー: {}",
                    total_energy_for_penalty, penalty_energy
                );
                if total_energy_for_penalty > penalty_energy {
                    break; // 合計エナジーがペナルティエナジーを上回ったらループを終了
                }

                // 再度ペナルティエナジー以上のカードがあるか確認
                let high_energy_cards: Vec<(Entity, &Card)> = available_cards
                    .iter()
                    .filter(|(_, card)| card.energy >= penalty_energy - total_energy_for_penalty)
                    .cloned()
                    .collect();

                if !high_energy_cards.is_empty() {
                    println!("ペナルティエナジーの差以上のカードがあります。選択可能なカード:");
                    for (index, (entity, card)) in high_energy_cards.iter().enumerate() {
                        println!("{}: {} [エナジー: {}]", index + 1, card.name, card.energy);
                    }
                }
            }
        }

        // 最終的に選んだカードのリストを表示
        println!("最終的に選ばれたカード:");
        for entity in selected_cards.clone() {
            if let Ok((_, card)) = query.get(entity) {
                println!("- {} [エナジー: {}]", card.name, card.energy);
            }
        }

        println!("ペナルティを受け入れますか？");
        let choice = check_the_will_of_spinning_cards();
        match choice {
            1 => {
                for entity in selected_cards.clone() {
                    commands.entity(entity).insert(Location::XZone);
                    commands.entity(entity).insert(SpinCondition::ReSpin);
                    commands.entity(entity).insert(FaceCondition::Facedown);
                    in_xzone_cards.cards.push(entity);
                    println!("{:?}はペナルティでXゾーンに裏側で置かれました。", entity)
                }

                tactics_with_penalty_event_writer.send(PlayTacticsWithPenaltyFromHand {
                    played_card_id: event.played_card_id,
                    spin_cards_list: event.spin_cards_list.clone(),
                    penalty_card_list: selected_cards.clone(),
                });
            }

            2 => {
                println!("やりなおします");
                for target in &event.spin_cards_list {
                    commands.entity(*target).insert(SpinCondition::ReSpin);
                }
                println!("スピンしたカードを元に戻しました。");
                select_hand_event_writer.send(SelectHand);
            }
            _ => {}
        }
        // 意思決定関数を入れた
    }
}

pub fn play_tactics_with_penalty_from_hand(
    mut tactics_with_penalty_event_reader: EventReader<PlayTacticsWithPenaltyFromHand>,
    mut commands: Commands,
    mut play_tactics_finish_event_writer: EventWriter<PlayTacticsFinish>,
    query: Query<(Entity, &Card)>,
) {
    for event in tactics_with_penalty_event_reader.read() {
        if let Ok((_, card)) = query.get(event.played_card_id) {
            commands
                .entity(event.played_card_id)
                .insert(Location::SZone);
            println!("{}を手札から発動しました", card.name);
            play_tactics_finish_event_writer.send(PlayTacticsFinish {
                played_card_id: event.played_card_id,
            });
        }
    }
}
