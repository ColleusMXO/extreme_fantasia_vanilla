use std::io;

use bevy::prelude::*;

use super::event::*;
use crate::game::loading::component::*;
use crate::game::loading::resource::*;
use crate::game::play::main_phase::free::event::MainPhaseManagement;
use crate::game::play::main_phase::free::event::SituationLog;
use crate::game::play::main_phase::resource::*;

use crate::game::play::main_phase::event::*;
use crate::game::resource::*;

pub fn attack_start(
    mut attack_start_event_reader: EventReader<AttackStart>,
    mut select_attacker_event_writer: EventWriter<SelectAttacker>,
) {
    for event in attack_start_event_reader.read() {
        println!("バトルに移行します");
        select_attacker_event_writer.send(SelectAttacker);
    }
}

pub fn select_attacker(
    fzone: ResMut<Player1FZone>,
    query: Query<(Entity, &Card, &Fighter, &Location, &SpinCondition), With<Player1>>, // Fighterコンポーネントを追加
    mut attack_event_writer: EventWriter<Attack>,
    mut select_attacker_event_reader: EventReader<SelectAttacker>,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
) {
    for _event in select_attacker_event_reader.read() {
        let mut available_fighters = Vec::new();

        println!("Fゾーンにある攻撃可能なあなたのファイターは:");

        for &entity in &fzone.cards {
            if let Ok((_, card, fighter, _, spincondition)) = query.get(entity) {
                if !fighter.summoned_sick {
                    if let SpinCondition::ReSpin = spincondition {
                        available_fighters.push(entity);
                        println!(
                            "{}: {}[{}]",
                            available_fighters.len(),
                            card.name,
                            fighter.power
                        ); // 召喚酔いでない場合
                    }
                }
            }
        }

        if available_fighters.is_empty() {
            println!("召喚酔いしていないファイターがFゾーンにありません。");
            main_phase_management_event_writer.send(MainPhaseManagement);
            return;
        }

        println!(
            "ファイターを選択してください (1-{} または 0で戻る):",
            available_fighters.len()
        );

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(selected_index) = input.trim().parse::<usize>() {
                if selected_index == 0 {
                    // 0を選択した場合、管理フェーズに戻る
                    main_phase_management_event_writer.send(MainPhaseManagement);
                    break; // ループを抜ける
                } else if selected_index > 0 && selected_index <= available_fighters.len() {
                    let selected_entity = available_fighters[selected_index - 1];

                    if let Ok((_, card, _, _, _)) = query.get(selected_entity) {
                        attack_event_writer.send(Attack {
                            attacker_id: selected_entity,
                        });
                        println!("{} を攻撃するファイターとして選択しました", card.name); // カード名を表示
                    }
                    break; // 有効な選択が行われたらループを抜ける
                } else {
                    println!("無効な選択です。もう一度入力してください。");
                }
            } else {
                println!("有効な番号を入力してください。");
            }
        }
    }
}

pub fn select_attacker_opponent(
    fzone: ResMut<Player2FZone>,
    query: Query<(Entity, &Card, &Fighter, &Location, &SpinCondition), With<Player2>>, // Fighterコンポーネントを追加
    mut attack_event_writer: EventWriter<Attack>,
    mut select_attacker_event_reader: EventReader<SelectAttacker>,
    mut main_phase_management_event_writer: EventWriter<MainPhaseManagement>,
) {
    for _event in select_attacker_event_reader.read() {
        let mut available_fighters = Vec::new();

        println!("Fゾーンにある攻撃可能なあなたのファイターは:");

        for &entity in &fzone.cards {
            if let Ok((_, card, fighter, _, spincondition)) = query.get(entity) {
                if !fighter.summoned_sick {
                    if let SpinCondition::ReSpin = spincondition {
                        available_fighters.push(entity);
                        println!(
                            "{}: {}[{}]",
                            available_fighters.len(),
                            card.name,
                            fighter.power
                        ); // 召喚酔いでない場合
                    }
                }
            }
        }

        if available_fighters.is_empty() {
            println!("召喚酔いしていないファイターがFゾーンにありません。");
            main_phase_management_event_writer.send(MainPhaseManagement);
            return;
        }

        println!(
            "ファイターを選択してください (1-{} または 0で戻る):",
            available_fighters.len()
        );

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(selected_index) = input.trim().parse::<usize>() {
                if selected_index == 0 {
                    // 0を選択した場合、管理フェーズに戻る
                    main_phase_management_event_writer.send(MainPhaseManagement);
                    break; // ループを抜ける
                } else if selected_index > 0 && selected_index <= available_fighters.len() {
                    let selected_entity = available_fighters[selected_index - 1];

                    if let Ok((_, card, _, _, _)) = query.get(selected_entity) {
                        attack_event_writer.send(Attack {
                            attacker_id: selected_entity,
                        });
                        println!("{} を攻撃するファイターとして選択しました", card.name); // カード名を表示
                    }
                    break; // 有効な選択が行われたらループを抜ける
                } else {
                    println!("無効な選択です。もう一度入力してください。");
                }
            } else {
                println!("有効な番号を入力してください。");
            }
        }
    }
}

pub fn choose_card_being_attacked(
    szone_opponent: ResMut<Player2SZone>,
    fzone_opponent: ResMut<Player2FZone>,
    query: Query<(Entity, &Card, Option<&Fighter>, &SpinCondition), With<Player2>>, // Fighterコンポーネントを追加
    mut commands: Commands,
    mut being_attacked_event_writer: EventWriter<BeingAttacked>,
    mut attack_event_reader: EventReader<Attack>,
    mut choose_guard_event_writer: EventWriter<ChooseGuard>,
    mut select_attacker_event_writer: EventWriter<SelectAttacker>,
) {
    for event in attack_event_reader.read() {
        let mut available_cards = Vec::new();

        // Sゾーンのカードをリストアップ
        for &entity in &szone_opponent.cards {
            if let Ok((_, card, fighter_option, _)) = query.get(entity) {
                available_cards.push(entity);
                if let Some(fighter) = fighter_option {
                    println!(
                        "{}: {} (Sゾーン)[パワー: {}]",
                        available_cards.len(),
                        card.name,
                        fighter.power
                    );
                } else {
                    println!("{}: {} (Sゾーン)", available_cards.len(), card.name);
                }
            }
        }

        // Fゾーンのスピン中のカードをリストアップ
        for &entity in &fzone_opponent.cards {
            if let Ok((_, card, fighter, spincondition)) = query.get(entity) {
                if let SpinCondition::Spin = spincondition {
                    available_cards.push(entity);
                    // スピンしている場合
                    if let Some(fighter) = fighter {
                        println!(
                            "{}: {} (Fゾーン)[パワー: {}]",
                            available_cards.len(),
                            card.name,
                            fighter.power
                        );
                    } else {
                        println!("{}: {} (Fゾーン)", available_cards.len(), card.name);
                    }
                }
            }
        }

        if available_cards.is_empty() {
            println!("攻撃対象となるカードがありません。");
            return;
        }

        println!("攻撃対象を選択してください (1-{}):", available_cards.len());

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(selected_index) = input.trim().parse::<usize>() {
                if selected_index == 0 {
                    // 0が選択された場合、操作をキャンセル
                    println!("攻撃をキャンセルしました。");
                    select_attacker_event_writer.send(SelectAttacker);

                    return; // メソッドを終了
                } else if selected_index > 0 && selected_index <= available_cards.len() {
                    let selected_entity = available_cards[selected_index - 1];

                    if let Ok((_, card, _, _)) = query.get(selected_entity) {
                        choose_guard_event_writer.send(ChooseGuard {
                            attacked_card_id: selected_entity,
                            attacker_id: event.attacker_id,
                        });

                        println!("{} を攻撃対象として選択しました", card.name); // カード名を表示
                        println!("攻撃ファイターをスピンしました。");

                        commands
                            .entity(event.attacker_id)
                            .insert(SpinCondition::Spin);
                    }
                    break; // 有効な選択が行われたらループを抜ける
                } else {
                    println!("無効な選択です。もう一度入力してください。");
                }
            } else {
                println!("有効な番号を入力してください。");
            }
        }
    }
}

pub fn choose_card_being_attacked_opponent(
    szone_opponent: ResMut<Player1SZone>,
    fzone_opponent: ResMut<Player1FZone>,
    query: Query<(Entity, &Card, Option<&Fighter>, &SpinCondition), With<Player1>>, // Fighterコンポーネントを追加
    mut commands: Commands,
    mut being_attacked_event_writer: EventWriter<BeingAttacked>,
    mut attack_event_reader: EventReader<Attack>,
    mut choose_guard_event_writer: EventWriter<ChooseGuard>,
    mut select_attacker_event_writer: EventWriter<SelectAttacker>,
) {
    for event in attack_event_reader.read() {
        let mut available_cards = Vec::new();

        // Sゾーンのカードをリストアップ
        for &entity in &szone_opponent.cards {
            if let Ok((_, card, fighter_option, _)) = query.get(entity) {
                available_cards.push(entity);
                if let Some(fighter) = fighter_option {
                    println!(
                        "{}: {} (Sゾーン)[パワー: {}]",
                        available_cards.len(),
                        card.name,
                        fighter.power
                    );
                } else {
                    println!("{}: {} (Sゾーン)", available_cards.len(), card.name);
                }
            }
        }

        // Fゾーンのスピン中のカードをリストアップ
        for &entity in &fzone_opponent.cards {
            if let Ok((_, card, fighter, spincondition)) = query.get(entity) {
                if let SpinCondition::Spin = spincondition {
                    available_cards.push(entity);
                    // スピンしている場合
                    if let Some(fighter) = fighter {
                        println!(
                            "{}: {} (Fゾーン)[パワー: {}]",
                            available_cards.len(),
                            card.name,
                            fighter.power
                        );
                    } else {
                        println!("{}: {} (Fゾーン)", available_cards.len(), card.name);
                    }
                }
            }
        }

        if available_cards.is_empty() {
            println!("攻撃対象となるカードがありません。");
            return;
        }

        println!("攻撃対象を選択してください (1-{}):", available_cards.len());

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(selected_index) = input.trim().parse::<usize>() {
                if selected_index == 0 {
                    // 0が選択された場合、操作をキャンセル
                    println!("攻撃をキャンセルしました。");
                    select_attacker_event_writer.send(SelectAttacker);

                    return; // メソッドを終了
                } else if selected_index > 0 && selected_index <= available_cards.len() {
                    let selected_entity = available_cards[selected_index - 1];

                    if let Ok((_, card, _, _)) = query.get(selected_entity) {
                        choose_guard_event_writer.send(ChooseGuard {
                            attacked_card_id: selected_entity,
                            attacker_id: event.attacker_id,
                        });

                        println!("{} を攻撃対象として選択しました", card.name); // カード名を表示
                        println!("攻撃ファイターをスピンしました。");

                        commands
                            .entity(event.attacker_id)
                            .insert(SpinCondition::Spin);
                    }
                    break; // 有効な選択が行われたらループを抜ける
                } else {
                    println!("無効な選択です。もう一度入力してください。");
                }
            } else {
                println!("有効な番号を入力してください。");
            }
        }
    }
}

pub fn choose_guard(
    query: Query<(Entity, &Card, &SpinCondition, &Fighter), (With<Player2>, With<Guard>)>,
    fzone_opponent: ResMut<Player2FZone>,
    mut choose_guard_eventreader: EventReader<ChooseGuard>,
    mut attack_fzone_event_writer: EventWriter<AttackFZone>,
    mut being_attacked_event_writer: EventWriter<BeingAttacked>,
    mut commands: Commands,
) {
    for event in choose_guard_eventreader.read() {
        let mut has_respin = false;

        // fzone_opponent に含まれるエンティティをループ
        let mut available_guards = Vec::new();

        for &entity in &fzone_opponent.cards {
            if let Ok((_, card, spincondition, fighter)) = query.get(entity) {
                if let SpinCondition::ReSpin = spincondition {
                    available_guards.push(entity);
                    println!(
                        "{}: {}[{}]",
                        available_guards.len(),
                        card.name,
                        fighter.power
                    ); // 召喚酔いでない場合
                }
            }
        }

        if available_guards.is_empty() {
            println!("ガードはありません");
            being_attacked_event_writer.send(BeingAttacked {
                attacker_id: event.attacker_id,
                attacked_card_id: event.attacked_card_id,
            });
            return;
        } else {
            loop {
                println!(
                    "ガードファイターを選んでいます。 (1-{})[0]でガードをやめる",
                    available_guards.len()
                );

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                if let Ok(selected_index) = input.trim().parse::<usize>() {
                    if selected_index == 0 {
                        // 攻撃をやめる場合
                        println!("ガードされませんでした");
                        being_attacked_event_writer.send(BeingAttacked {
                            attacked_card_id: event.attacked_card_id,
                            attacker_id: event.attacker_id,
                        });
                        return;
                    }
                    if selected_index > 0 && selected_index <= available_guards.len() {
                        let selected_entity = available_guards[selected_index - 1];

                        if let Ok((_, card, _, _)) = query.get(selected_entity) {
                            attack_fzone_event_writer.send(AttackFZone {
                                attacked_card_id: selected_entity,
                                attacker_id: event.attacker_id,
                            });
                            println!("{} にガードされました", card.name); // カード名を表示
                            commands.entity(selected_entity).insert(SpinCondition::Spin);
                            println!("{}はスピンしました", card.name);
                            return;
                        }
                    } else {
                        println!("ガードされませんでした");
                        being_attacked_event_writer.send(BeingAttacked {
                            attacked_card_id: event.attacked_card_id,
                            attacker_id: event.attacker_id,
                        });
                    }
                } else {
                    println!("有効な番号を入力してください。");
                }
            }
        }
    }
}

pub fn choose_guard_opponent(
    query: Query<(Entity, &Card, &SpinCondition, &Fighter), (With<Player1>, With<Guard>)>,
    fzone_opponent: ResMut<Player1FZone>,
    mut choose_guard_eventreader: EventReader<ChooseGuard>,
    mut attack_fzone_event_writer: EventWriter<AttackFZone>,
    mut being_attacked_event_writer: EventWriter<BeingAttacked>,
    mut commands: Commands,
) {
    for event in choose_guard_eventreader.read() {
        let mut has_respin = false;

        // fzone_opponent に含まれるエンティティをループ
        let mut available_guards = Vec::new();

        for &entity in &fzone_opponent.cards {
            if let Ok((_, card, spincondition, fighter)) = query.get(entity) {
                if let SpinCondition::ReSpin = spincondition {
                    available_guards.push(entity);
                    println!(
                        "{}: {}[{}]",
                        available_guards.len(),
                        card.name,
                        fighter.power
                    ); // 召喚酔いでない場合
                }
            }
        }

        if available_guards.is_empty() {
            println!("ガードはありません");
            being_attacked_event_writer.send(BeingAttacked {
                attacker_id: event.attacker_id,
                attacked_card_id: event.attacked_card_id,
            });
            return;
        } else {
            loop {
                println!(
                    "ガードファイターを選んでいます。 (1-{})[0]でガードをやめる",
                    available_guards.len()
                );

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                if let Ok(selected_index) = input.trim().parse::<usize>() {
                    if selected_index == 0 {
                        // 攻撃をやめる場合
                        println!("ガードされませんでした");
                        being_attacked_event_writer.send(BeingAttacked {
                            attacked_card_id: event.attacked_card_id,
                            attacker_id: event.attacker_id,
                        });
                        return;
                    }
                    if selected_index > 0 && selected_index <= available_guards.len() {
                        let selected_entity = available_guards[selected_index - 1];

                        if let Ok((_, card, _, _)) = query.get(selected_entity) {
                            attack_fzone_event_writer.send(AttackFZone {
                                attacked_card_id: selected_entity,
                                attacker_id: event.attacker_id,
                            });
                            println!("{} にガードされました", card.name); // カード名を表示
                            commands.entity(selected_entity).insert(SpinCondition::Spin);
                            println!("{}はスピンしました", card.name);
                            return;
                        }
                    } else {
                        println!("ガードされませんでした");
                        being_attacked_event_writer.send(BeingAttacked {
                            attacked_card_id: event.attacked_card_id,
                            attacker_id: event.attacker_id,
                        });
                    }
                } else {
                    println!("有効な番号を入力してください。");
                }
            }
        }
    }
}

pub fn handle_being_attacked(
    query: Query<(
        Entity,
        &Card,
        Option<&Fighter>,
        &Location,
        Option<&FirstEnergy>,
        Option<&Tactics>,
    )>,
    mut being_attacked_event_reader: EventReader<BeingAttacked>,
    mut first_energy_penalty_event_writer: EventWriter<FirstEnergyPenalty>,
    mut attack_tactics_event_writer: EventWriter<AttackTactics>,
    mut attack_szone_event_writer: EventWriter<AttackSZone>,
    mut attack_fzone_event_writer: EventWriter<AttackFZone>,
    mut commands: Commands,
) {
    for event in being_attacked_event_reader.read() {
        if let Ok((_, card, fighter_option, location, first_energy_option, tactics_option)) =
            query.get(event.attacked_card_id)
        {
            match *location {
                Location::FZone => {
                    // Fゾーンにいる場合の処理
                    println!("{} は Fゾーンにいます", card.name);
                    attack_fzone_event_writer.send(AttackFZone {
                        attacker_id: event.attacker_id,
                        attacked_card_id: event.attacked_card_id,
                    });
                    // 追加の処理をここに記述する
                }
                Location::SZone => {
                    // Sゾーンにいる場合の処理
                    if let Some(_) = fighter_option {
                        println!("{} は Sゾーンにいるファイターです", card.name);
                        attack_szone_event_writer.send(AttackSZone {
                            attacker_id: event.attacker_id,
                            attacked_card_id: event.attacked_card_id,
                        });
                        // ファイターの特殊な処理をここに記述する
                    }
                }
                _ => {}
            }

            // FirstEnergyコンポーネントがある場合の処理
            if first_energy_option.is_some() {
                println!("{} は ファーストエナジー です", card.name);
                first_energy_penalty_event_writer.send(FirstEnergyPenalty {
                    attacker_id: event.attacker_id,
                });
                // FirstEnergy に関連する処理をここに記述する
            }

            // Tacticsコンポーネントがある場合の処理
            if tactics_option.is_some() {
                println!("{} は タクティクスです", card.name);
                attack_tactics_event_writer.send(AttackTactics {
                    attacker_id: event.attacker_id,
                    attacked_card_id: event.attacked_card_id,
                });
                // Tactics に関連する処理をここに記述する
            }
        } else {
            println!(
                "攻撃対象の情報を取得できませんでした。エンティティID: {:?}",
                event.attacked_card_id
            );
        }
    }
}

pub fn attack_tactics(
    mut attack_tactics_event_reader: EventReader<AttackTactics>,
    mut battle_tactics_win_event_writer: EventWriter<BattleTacticsWin>,
    mut destroyed_tactics_event_writer: EventWriter<DestroyedTactics>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
    mut in_xzone_card: ResMut<FacedownXZoneCardsOpponent>,
    mut commands: Commands,
) {
    for event in attack_tactics_event_reader.read() {
        battle_tactics_win_event_writer.send(BattleTacticsWin {
            attacker_id: event.attacker_id,
        });
        println!("{:?}はバトルに勝利しました", event.attacker_id);
        commands
            .entity(event.attacked_card_id)
            .insert(FaceCondition::Facedown);
        commands
            .entity(event.attacked_card_id)
            .insert(Location::XZone);
        commands
            .entity(event.attacked_card_id)
            .insert(SpinCondition::ReSpin);
        in_xzone_card.cards.push(event.attacked_card_id);

        destroyed_tactics_event_writer.send(DestroyedTactics {
            destroyed_tactics_id: event.attacked_card_id,
        });
        println!(
            "{:?}は破壊されました。Ｘゾーンに送られます。",
            event.attacked_card_id
        );

        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn attack_tactics_opponent(
    mut attack_tactics_event_reader: EventReader<AttackTactics>,
    mut battle_tactics_win_event_writer: EventWriter<BattleTacticsWin>,
    mut destroyed_tactics_event_writer: EventWriter<DestroyedTactics>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
    mut in_xzone_card: ResMut<FacedownXZoneCards>,
    mut commands: Commands,
) {
    for event in attack_tactics_event_reader.read() {
        battle_tactics_win_event_writer.send(BattleTacticsWin {
            attacker_id: event.attacker_id,
        });
        println!("{:?}はバトルに勝利しました", event.attacker_id);
        commands
            .entity(event.attacked_card_id)
            .insert(FaceCondition::Facedown);
        commands
            .entity(event.attacked_card_id)
            .insert(Location::XZone);
        commands
            .entity(event.attacked_card_id)
            .insert(SpinCondition::ReSpin);
        in_xzone_card.cards.push(event.attacked_card_id);

        destroyed_tactics_event_writer.send(DestroyedTactics {
            destroyed_tactics_id: event.attacked_card_id,
        });
        println!(
            "{:?}は破壊されました。Ｘゾーンに送られます。",
            event.attacked_card_id
        );

        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn attack_szone(
    mut attack_szone_event_reader: EventReader<AttackSZone>,
    mut battle_fighter_win_event_writer: EventWriter<BattleFighterWin>,
    mut destroyed_fighter_event_writer: EventWriter<DestroyedFighter>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
    mut in_xzone_card: ResMut<FacedownXZoneCardsOpponent>,
    query: Query<(Entity, &Card, &Fighter)>,
    mut commands: Commands,
) {
    for event in attack_szone_event_reader.read() {
        // クエリから攻撃者と攻撃対象の情報を取得
        let attacker_query = query.get(event.attacker_id);
        let attacked_query = query.get(event.attacked_card_id);

        match (attacker_query, attacked_query) {
            (
                Ok((_, attacker_card, attacker_fighter)),
                Ok((_, attacked_card, attacked_fighter)),
            ) => {
                let attacker_power = attacker_fighter.power; // 攻撃者のパワーを取得
                let attacked_power = attacked_fighter.power; // 攻撃対象のパワーを取得

                // 攻撃者のパワーが攻撃対象のパワーより大きい場合
                if attacker_power > attacked_power {
                    println!(
                        "{:?} は {:?} に勝利しました",
                        attacker_card.name, attacked_card.name
                    );

                    // 勝利イベントを発行
                    battle_fighter_win_event_writer.send(BattleFighterWin {
                        winner_id: event.attacker_id,
                    });

                    // 攻撃対象を破壊するイベントを発行
                    destroyed_fighter_event_writer.send(DestroyedFighter {
                        destroyed_fighter_id: event.attacked_card_id,
                    });

                    commands
                        .entity(event.attacked_card_id)
                        .insert(SpinCondition::ReSpin);
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::XZone);
                    commands
                        .entity(event.attacked_card_id)
                        .insert(FaceCondition::Facedown);
                    println!(
                        "{:?}は破壊されました。Ｘゾーンに送られます。",
                        event.attacked_card_id
                    );
                    in_xzone_card.cards.push(event.attacked_card_id);
                } else {
                    // 攻撃者のパワーが攻撃対象のパワー以下の場合
                    println!("何も起きませんでした");
                }

                // 攻撃終了イベントを発行
                attack_end_event_writer.send(AttackEnd {
                    attacker_id: event.attacker_id,
                });
            }
            _ => {
                // もしいずれかのエンティティが見つからない場合のエラーログ
                println!("攻撃者または攻撃対象の情報が見つかりません。");
            }
        }
    }
}

pub fn attack_szone_opponent(
    mut attack_szone_event_reader: EventReader<AttackSZone>,
    mut battle_fighter_win_event_writer: EventWriter<BattleFighterWin>,
    mut destroyed_fighter_event_writer: EventWriter<DestroyedFighter>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
    mut in_xzone_card: ResMut<FacedownXZoneCards>,
    query: Query<(Entity, &Card, &Fighter)>,
    mut commands: Commands,
) {
    for event in attack_szone_event_reader.read() {
        // クエリから攻撃者と攻撃対象の情報を取得
        let attacker_query = query.get(event.attacker_id);
        let attacked_query = query.get(event.attacked_card_id);

        match (attacker_query, attacked_query) {
            (
                Ok((_, attacker_card, attacker_fighter)),
                Ok((_, attacked_card, attacked_fighter)),
            ) => {
                let attacker_power = attacker_fighter.power; // 攻撃者のパワーを取得
                let attacked_power = attacked_fighter.power; // 攻撃対象のパワーを取得

                // 攻撃者のパワーが攻撃対象のパワーより大きい場合
                if attacker_power > attacked_power {
                    println!(
                        "{:?} は {:?} に勝利しました",
                        attacker_card.name, attacked_card.name
                    );

                    // 勝利イベントを発行
                    battle_fighter_win_event_writer.send(BattleFighterWin {
                        winner_id: event.attacker_id,
                    });

                    // 攻撃対象を破壊するイベントを発行
                    destroyed_fighter_event_writer.send(DestroyedFighter {
                        destroyed_fighter_id: event.attacked_card_id,
                    });

                    commands
                        .entity(event.attacked_card_id)
                        .insert(SpinCondition::ReSpin);
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::XZone);
                    commands
                        .entity(event.attacked_card_id)
                        .insert(FaceCondition::Facedown);
                    println!(
                        "{:?}は破壊されました。Ｘゾーンに送られます。",
                        event.attacked_card_id
                    );
                    in_xzone_card.cards.push(event.attacked_card_id);
                } else {
                    // 攻撃者のパワーが攻撃対象のパワー以下の場合
                    println!("何も起きませんでした");
                }

                // 攻撃終了イベントを発行
                attack_end_event_writer.send(AttackEnd {
                    attacker_id: event.attacker_id,
                });
            }
            _ => {
                // もしいずれかのエンティティが見つからない場合のエラーログ
                println!("攻撃者または攻撃対象の情報が見つかりません。");
            }
        }
    }
}

pub fn attack_fzone(
    mut attack_fzone_event_reader: EventReader<AttackFZone>,
    mut battle_fighter_win_event_writer: EventWriter<BattleFighterWin>,
    mut aiuchi_event_writer: EventWriter<Aiuchi>,
    mut choose_withdraw_event_writer: EventWriter<ChooseWithdrawOrGY>,
    mut opponent_choose_withdraw_event_writer: EventWriter<OpponentWithdrawOrGY>,
    query: Query<(Entity, &Card, &Fighter)>,
) {
    for event in attack_fzone_event_reader.read() {
        // クエリから攻撃者と攻撃対象の情報を取得
        let attacker_query = query.get(event.attacker_id);
        let attacked_query = query.get(event.attacked_card_id);

        match (attacker_query, attacked_query) {
            (
                Ok((_, attacker_card, attacker_fighter)),
                Ok((_, attacked_card, attacked_fighter)),
            ) => {
                let attacker_power = attacker_fighter.power; // 攻撃者のパワーを取得
                let attacked_power = attacked_fighter.power; // 攻撃対象のパワーを取得

                // 攻撃者のパワーが攻撃対象のパワーより大きい場合
                if attacker_power > attacked_power {
                    println!("バトルに勝ちました。");
                    battle_fighter_win_event_writer.send(BattleFighterWin {
                        winner_id: event.attacker_id,
                    });
                    opponent_choose_withdraw_event_writer.send(OpponentWithdrawOrGY {
                        attacker_id: event.attacker_id,
                        attacked_card_id: event.attacked_card_id,
                    });
                }
                if attacker_power == attacked_power {
                    println!("相打ちです");
                    aiuchi_event_writer.send(Aiuchi {
                        attacker_id: event.attacker_id,
                        attacked_card_id: event.attacked_card_id,
                    });
                }
                if attacker_power < attacked_power {
                    println!("バトルに負けました");

                    battle_fighter_win_event_writer.send(BattleFighterWin {
                        winner_id: event.attacked_card_id,
                    });
                    choose_withdraw_event_writer.send(ChooseWithdrawOrGY {
                        attacker_id: event.attacker_id,
                        attacked_card_id: event.attacked_card_id,
                    });
                }

                // 攻撃終了イベントを発行
            }
            _ => {
                // もしいずれかのエンティティが見つからない場合のエラーログ
                println!("攻撃者または攻撃対象の情報が見つかりません。");
            }
        }
    }
}

pub fn check_the_will_of_withdraw_or_gy() -> i32 {
    loop {
        println!("[1]:撤退させる [2]:墓地に送る ");
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

pub fn choose_withdraw_or_gy(
    mut choose_withdraw_event_writer: EventReader<ChooseWithdrawOrGY>,
    mut commands: Commands,
    query: Query<(Entity, &Card, &Location)>,
    szone: ResMut<Player1SZone>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
) {
    for event in choose_withdraw_event_writer.read() {
        let mut total_energy = 0;

        for entity in szone.cards.iter() {
            if let Ok((_, card, _)) = query.get(*entity) {
                total_energy += card.energy;
            }
        }
        println!("合計エナジーは現在:{}", total_energy);

        if let Ok((_, card, _)) = query.get(event.attacker_id) {
            println!("{}はエナジー:{}", card.name, card.energy);
            total_energy += card.energy;
        }

        let choice = check_the_will_of_withdraw_or_gy();
        match choice {
            1 => {
                if total_energy > 10 {
                    println!(
                        "このカードはエナジーが大きすぎて、撤退できません。自動的に墓地に行きます。"
                    );
                    commands
                        .entity(event.attacker_id)
                        .insert(Location::GraveYard);
                    commands
                        .entity(event.attacker_id)
                        .insert(SpinCondition::ReSpin);
                } else {
                    println!("撤退させました");
                    commands.entity(event.attacker_id).insert(Location::SZone);
                }
            }

            2 => {
                println!("墓地に行きました");
                commands
                    .entity(event.attacker_id)
                    .insert(Location::GraveYard);
                commands
                    .entity(event.attacker_id)
                    .insert(SpinCondition::ReSpin);
            }
            _ => {}
        }
        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn choose_withdraw_or_gy_opponent(
    mut choose_withdraw_event_writer: EventReader<ChooseWithdrawOrGY>,
    mut commands: Commands,
    query: Query<(Entity, &Card, &Location)>,
    szone: ResMut<Player2SZone>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
) {
    for event in choose_withdraw_event_writer.read() {
        let mut total_energy = 0;

        for entity in szone.cards.iter() {
            if let Ok((_, card, _)) = query.get(*entity) {
                total_energy += card.energy;
            }
        }
        println!("合計エナジーは現在:{}", total_energy);

        if let Ok((_, card, _)) = query.get(event.attacker_id) {
            println!("{}はエナジー:{}", card.name, card.energy);
            total_energy += card.energy;
        }

        let choice = check_the_will_of_withdraw_or_gy();
        match choice {
            1 => {
                if total_energy > 10 {
                    println!(
                        "このカードはエナジーが大きすぎて、撤退できません。自動的に墓地に行きます。"
                    );
                    commands
                        .entity(event.attacker_id)
                        .insert(Location::GraveYard);
                    commands
                        .entity(event.attacker_id)
                        .insert(SpinCondition::ReSpin);
                } else {
                    println!("撤退させました");
                    commands.entity(event.attacker_id).insert(Location::SZone);
                }
            }

            2 => {
                println!("墓地に行きました");
                commands
                    .entity(event.attacker_id)
                    .insert(Location::GraveYard);
                commands
                    .entity(event.attacker_id)
                    .insert(SpinCondition::ReSpin);
            }
            _ => {}
        }
        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn aiuchi(
    mut aiuchi_event_reader: EventReader<Aiuchi>,
    mut commands: Commands,
    query: Query<(Entity, &Card, &Location)>,
    szone_opponent: ResMut<Player2SZone>,
    szone: ResMut<Player1SZone>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
) {
    for event in aiuchi_event_reader.read() {
        let mut total_energy = 0;

        for entity in szone.cards.iter() {
            if let Ok((_, card, _)) = query.get(*entity) {
                total_energy += card.energy;
            }
        }
        println!("あなたのSゾーンの合計エナジーは現在:{}", total_energy);

        if let Ok((_, card, _)) = query.get(event.attacker_id) {
            println!("{}はエナジー:{}", card.name, card.energy);
            total_energy += card.energy;
        }

        let choice = check_the_will_of_withdraw_or_gy();
        match choice {
            1 => {
                if total_energy > 10 {
                    println!(
                        "このカードはエナジーが大きすぎて、撤退できません。自動的に墓地に行きます。"
                    );
                    commands
                        .entity(event.attacker_id)
                        .insert(Location::GraveYard);
                    commands
                        .entity(event.attacker_id)
                        .insert(SpinCondition::ReSpin);
                } else {
                    println!("撤退させました");
                    commands.entity(event.attacker_id).insert(Location::SZone);
                }
            }

            2 => {
                println!("墓地に行きました");
                commands
                    .entity(event.attacker_id)
                    .insert(Location::GraveYard);
                commands
                    .entity(event.attacker_id)
                    .insert(SpinCondition::ReSpin);
            }
            _ => {}
        }

        let mut total_energy = 0;

        for entity in szone_opponent.cards.iter() {
            if let Ok((_, card, _)) = query.get(*entity) {
                total_energy += card.energy;
            }
        }
        println!("相手のSゾーンの合計エナジーは現在:{}", total_energy);

        if let Ok((_, card, _)) = query.get(event.attacked_card_id) {
            println!("{}はエナジー:{}", card.name, card.energy);
            total_energy += card.energy;
        }

        let choice = check_the_will_of_withdraw_or_gy();
        match choice {
            1 => {
                if total_energy > 10 {
                    println!(
                        "このカードはエナジーが大きすぎて、撤退できません。自動的に墓地に行きます。"
                    );
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::GraveYard);
                    commands
                        .entity(event.attacked_card_id)
                        .insert(SpinCondition::ReSpin);
                } else {
                    println!("相手は撤退させました");
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::SZone);
                }
            }

            2 => {
                println!("相手のファイターは墓地に行きました");
                commands
                    .entity(event.attacked_card_id)
                    .insert(Location::GraveYard);
                commands
                    .entity(event.attacked_card_id)
                    .insert(SpinCondition::ReSpin);
            }
            _ => {}
        }

        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn aiuchi_opponent(
    mut aiuchi_event_reader: EventReader<Aiuchi>,
    mut commands: Commands,
    query: Query<(Entity, &Card, &Location)>,
    szone_opponent: ResMut<Player1SZone>,
    szone: ResMut<Player2SZone>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
) {
    for event in aiuchi_event_reader.read() {
        let mut total_energy = 0;

        for entity in szone.cards.iter() {
            if let Ok((_, card, _)) = query.get(*entity) {
                total_energy += card.energy;
            }
        }
        println!("あなたのSゾーンの合計エナジーは現在:{}", total_energy);

        if let Ok((_, card, _)) = query.get(event.attacker_id) {
            println!("{}はエナジー:{}", card.name, card.energy);
            total_energy += card.energy;
        }

        let choice = check_the_will_of_withdraw_or_gy();
        match choice {
            1 => {
                if total_energy > 10 {
                    println!(
                        "このカードはエナジーが大きすぎて、撤退できません。自動的に墓地に行きます。"
                    );
                    commands
                        .entity(event.attacker_id)
                        .insert(Location::GraveYard);
                    commands
                        .entity(event.attacker_id)
                        .insert(SpinCondition::ReSpin);
                } else {
                    println!("撤退させました");
                    commands.entity(event.attacker_id).insert(Location::SZone);
                }
            }

            2 => {
                println!("墓地に行きました");
                commands
                    .entity(event.attacker_id)
                    .insert(Location::GraveYard);
                commands
                    .entity(event.attacker_id)
                    .insert(SpinCondition::ReSpin);
            }
            _ => {}
        }

        let mut total_energy = 0;

        for entity in szone_opponent.cards.iter() {
            if let Ok((_, card, _)) = query.get(*entity) {
                total_energy += card.energy;
            }
        }
        println!("相手のSゾーンの合計エナジーは現在:{}", total_energy);

        if let Ok((_, card, _)) = query.get(event.attacked_card_id) {
            println!("{}はエナジー:{}", card.name, card.energy);
            total_energy += card.energy;
        }

        let choice = check_the_will_of_withdraw_or_gy();
        match choice {
            1 => {
                if total_energy > 10 {
                    println!(
                        "このカードはエナジーが大きすぎて、撤退できません。自動的に墓地に行きます。"
                    );
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::GraveYard);
                    commands
                        .entity(event.attacked_card_id)
                        .insert(SpinCondition::ReSpin);
                } else {
                    println!("相手は撤退させました");
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::SZone);
                }
            }

            2 => {
                println!("相手のファイターは墓地に行きました");
                commands
                    .entity(event.attacked_card_id)
                    .insert(Location::GraveYard);
                commands
                    .entity(event.attacked_card_id)
                    .insert(SpinCondition::ReSpin);
            }
            _ => {}
        }

        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn opponent_choose_withdraw_or_gy(
    mut opponent_choose_withdraw_event_writer: EventReader<OpponentWithdrawOrGY>,
    mut commands: Commands,
    query: Query<(Entity, &Card, &Location)>,
    szone_opponent: ResMut<Player2SZone>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
) {
    for event in opponent_choose_withdraw_event_writer.read() {
        let mut total_energy = 0;

        for entity in szone_opponent.cards.iter() {
            if let Ok((_, card, _)) = query.get(*entity) {
                total_energy += card.energy;
            }
        }
        println!("合計エナジーは現在:{}", total_energy);

        if let Ok((_, card, _)) = query.get(event.attacked_card_id) {
            println!("{}はエナジー:{}", card.name, card.energy);
            total_energy += card.energy;
        }

        let choice = check_the_will_of_withdraw_or_gy();
        match choice {
            1 => {
                if total_energy > 10 {
                    println!(
                        "このカードはエナジーが大きすぎて、撤退できません。自動的に墓地に行きます。"
                    );
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::GraveYard);
                    commands
                        .entity(event.attacked_card_id)
                        .insert(SpinCondition::ReSpin);
                } else {
                    println!("撤退させました");
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::SZone);
                }
            }

            2 => {
                println!("墓地に行きました");
                commands
                    .entity(event.attacked_card_id)
                    .insert(Location::GraveYard);
                commands
                    .entity(event.attacked_card_id)
                    .insert(SpinCondition::ReSpin);
            }
            _ => {}
        }
        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn opponent_choose_withdraw_or_gy_opponent(
    mut opponent_choose_withdraw_event_writer: EventReader<OpponentWithdrawOrGY>,
    mut commands: Commands,
    query: Query<(Entity, &Card, &Location)>,
    szone_opponent: ResMut<Player1SZone>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
) {
    for event in opponent_choose_withdraw_event_writer.read() {
        let mut total_energy = 0;

        for entity in szone_opponent.cards.iter() {
            if let Ok((_, card, _)) = query.get(*entity) {
                total_energy += card.energy;
            }
        }
        println!("合計エナジーは現在:{}", total_energy);

        if let Ok((_, card, _)) = query.get(event.attacked_card_id) {
            println!("{}はエナジー:{}", card.name, card.energy);
            total_energy += card.energy;
        }

        let choice = check_the_will_of_withdraw_or_gy();
        match choice {
            1 => {
                if total_energy > 10 {
                    println!(
                        "このカードはエナジーが大きすぎて、撤退できません。自動的に墓地に行きます。"
                    );
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::GraveYard);
                    commands
                        .entity(event.attacked_card_id)
                        .insert(SpinCondition::ReSpin);
                } else {
                    println!("撤退させました");
                    commands
                        .entity(event.attacked_card_id)
                        .insert(Location::SZone);
                }
            }

            2 => {
                println!("墓地に行きました");
                commands
                    .entity(event.attacked_card_id)
                    .insert(Location::GraveYard);
                commands
                    .entity(event.attacked_card_id)
                    .insert(SpinCondition::ReSpin);
            }
            _ => {}
        }
        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn first_energy_penalty(
    mut first_energy_penalty_event_reader: EventReader<FirstEnergyPenalty>,
    mut commands: Commands,
    mut in_library_cards: ResMut<InLibraryCardsOpponent>,
    mut in_xzone_card: ResMut<FacedownXZoneCardsOpponent>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
) {
    for event in first_energy_penalty_event_reader.read() {
        if let Some(&top_card) = in_library_cards.cards.first() {
            commands.entity(top_card).insert(Location::XZone);
            in_library_cards.cards.remove(0); // 先頭のカードを削除
            println!("プレイヤー2は1枚ペナルティを受けました。{:?}", top_card);
            in_xzone_card.cards.push(top_card);
        } else {
            println!("No cards available in the deck.");
        }
        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn first_energy_penalty_opponent(
    mut first_energy_penalty_event_reader: EventReader<FirstEnergyPenalty>,
    mut commands: Commands,
    mut in_library_cards: ResMut<InLibraryCards>,
    mut in_xzone_card: ResMut<FacedownXZoneCards>,
    mut attack_end_event_writer: EventWriter<AttackEnd>,
) {
    for event in first_energy_penalty_event_reader.read() {
        if let Some(&top_card) = in_library_cards.cards.first() {
            commands.entity(top_card).insert(Location::XZone);
            in_library_cards.cards.remove(0); // 先頭のカードを削除
            println!("プレイヤー1は1枚ペナルティを受けました。{:?}", top_card);
            in_xzone_card.cards.push(top_card);
        } else {
            println!("No cards available in the deck.");
        }
        attack_end_event_writer.send(AttackEnd {
            attacker_id: event.attacker_id,
        });
    }
}

pub fn attack_end(
    mut attack_end_event_reader: EventReader<AttackEnd>,
    mut situation_log_event_writer: EventWriter<SituationLog>,
) {
    for event in attack_end_event_reader.read() {
        println!("{:?}はアタック終了しました", event.attacker_id);
        situation_log_event_writer.send(SituationLog);
    }
}
