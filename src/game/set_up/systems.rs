use super::MulliganState;
use super::event::*;
use super::resource::MulliganCounter;
use crate::game::GameState;
use crate::game::loading::component::*;
use crate::game::loading::resource::*;
use crate::game::play::Controller;
use crate::game::play::PlayState;
use crate::game::set_up::resource::*;
use bevy::{prelude::*, utils::hashbrown::HashMap};
use rand::prelude::*;
use std::io::{self, Write};
use std::ops::Mul;

pub const MAX_MULLIGAN_COUNT: u32 = 10;
pub const MULLIGAN_PENALTY_COUNT: u32 = 2;
pub fn mulligan_start(mut mulligan_event_writer: EventWriter<Mulligan>) {
    mulligan_event_writer.send(Mulligan);
}
pub fn mulligan(
    query: Query<(Entity, &Card, &Location), With<Player1>>,
    mut commands: Commands,
    mut in_library_cards: ResMut<InLibraryCards>,
    mut mulligan_event_reader: EventReader<Mulligan>,
    mut mulligan_management_event_writer: EventWriter<MulliganManagement>,
) {
    for event in mulligan_event_reader.read() {
        //手札を山に
        for (entity, card, location) in query.iter() {
            if let Location::InHand = location {
                commands.entity(entity).insert(Location::InLibrary); // 手札のカードをすべて山札に移動
                println!("{}, をデッキに戻しました", card.name);
                in_library_cards.cards.push(entity);
            }
        }
        //シャッフルする
        let mut rng = rand::rng();
        in_library_cards.cards.shuffle(&mut rng);
        println!("Shuffled cards in library:");
        //5枚引く
        for _ in 0..5 {
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
            }
        }

        mulligan_management_event_writer.send(MulliganManagement);
    }
}

pub fn check_the_will_of_mulligan() -> i32 {
    loop {
        println!("[1]:マリガンをする [2]:キープする");
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

pub fn mulligan_management(
    mut mulligan_counter: ResMut<MulliganCounter>,
    mut mulligan_event_writer: EventWriter<Mulligan>,
    mut mulligan_opponent_event_writer: EventWriter<MulliganOpponent>,
    mut mulligan_management_event_reader: EventReader<MulliganManagement>,
    query: Query<(Entity, &Card, &Location), With<Player1>>,
) {
    // リソースを初期化
    for event in mulligan_management_event_reader.read() {
        //手札を表示
        for (entity, card, location) in query.iter() {
            if let Location::InHand = location {
                println!("{}, ", card.name);
            }
        }

        if mulligan_counter.counter >= MAX_MULLIGAN_COUNT {
            println!("マリガンの回数が上限に達しました。");
            mulligan_opponent_event_writer.send(MulliganOpponent);
            return;
        }

        let choice = check_the_will_of_mulligan();

        match choice {
            1 => {
                // はい
                mulligan_counter.counter += 1;
                println!("マリガン{}回目です", mulligan_counter.counter);
                // 次のイベントを発信
                mulligan_event_writer.send(Mulligan); // 必要に応じて次のイベントを発信
            }
            2 => {
                // いいえマリガンを終了して、相手のマリガンに移る。
                mulligan_opponent_event_writer.send(MulliganOpponent);
            }
            _ => {}
        }
    }
}

pub fn mulligan_opponent(
    query: Query<(Entity, &Card, &Location), With<Player2>>,
    mut commands: Commands,
    mut in_library_cards: ResMut<InLibraryCardsOpponent>,
    mut mulligan_opponent_event_reader: EventReader<MulliganOpponent>,
    mut mulligan_management_event_writer: EventWriter<MulliganManagementOpponent>,
) {
    for event in mulligan_opponent_event_reader.read() {
        //手札を山に
        for (entity, card, location) in query.iter() {
            if let Location::InHand = location {
                commands.entity(entity).insert(Location::InLibrary); // 手札のカードをすべて山札に移動
                println!("{}, をデッキに戻しました", card.name);
                in_library_cards.cards.push(entity);
            }
        }
        //シャッフルする
        let mut rng = rand::rng();
        in_library_cards.cards.shuffle(&mut rng);
        println!("Shuffled cards in library:");
        //5枚引く
        for _ in 0..5 {
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
            }
        }
        //手札を表示する。

        mulligan_management_event_writer.send(MulliganManagementOpponent);
    }
}

pub fn mulligan_management_opponent(
    mut mulligan_counter: ResMut<MulliganCounterOpponent>,
    mut mulligan_opponent_event_writer: EventWriter<MulliganOpponent>,
    mut mulligan_penalty_event_writer: EventWriter<MulliganPenalty>,
    mut mulligan_management_opponent_event_reader: EventReader<MulliganManagementOpponent>,
    query: Query<(Entity, &Card, &Location), With<Player2>>,
) {
    // リソースを初期化
    for event in mulligan_management_opponent_event_reader.read() {
        for (entity, card, location) in query.iter() {
            if let Location::InHand = location {
                println!("{}, ", card.name);
            }
        }

        if mulligan_counter.counter >= MAX_MULLIGAN_COUNT {
            println!("マリガンの回数が上限に達しました。");
            mulligan_penalty_event_writer.send(MulliganPenalty);
            return;
        }

        let choice = check_the_will_of_mulligan();

        match choice {
            1 => {
                // はい
                mulligan_counter.counter += 1;
                println!("マリガン{}回目です", mulligan_counter.counter);
                // 次のイベントを発信
                mulligan_opponent_event_writer.send(MulliganOpponent); // 必要に応じて次のイベントを発信
            }
            2 => {
                // いいえ
                mulligan_penalty_event_writer.send(MulliganPenalty);
            }
            _ => {}
        }
    }

    // マリガンの回数が上限に達した場合、イベントBを発信
}

pub fn mulligan_penalty(
    mut mulligan_counter: ResMut<MulliganCounter>,
    mut mulligan_counter_opponent: ResMut<MulliganCounterOpponent>,
    query: Query<(Entity, &Card, &Location), With<Player1>>,
    query_opponent: Query<(Entity, &Card, &Location), With<Player2>>,
    mut commands: Commands,
    mut in_library_cards: ResMut<InLibraryCards>,
    mut in_library_cards_opponent: ResMut<InLibraryCardsOpponent>,
    mut mulligan_finish_event_writer: EventWriter<MulliganFinish>,
    mut mulligan_penalty_event_reader: EventReader<MulliganPenalty>,
) {
    for event in mulligan_penalty_event_reader.read() {
        if mulligan_counter.counter >= MULLIGAN_PENALTY_COUNT {
            if let Some(&top_card) = in_library_cards_opponent.cards.first() {
                if let Ok((_, card, _)) = query_opponent.get(top_card) {
                    println!(
                        "相手は {}, を引きました entity id: {:?},",
                        card.name, top_card
                    );
                    commands.entity(top_card).insert(Location::InHand);
                    in_library_cards_opponent.cards.remove(0); // 先頭のカードを削除
                } else {
                    println!("No cards available in the deck.");
                }
            } else {
                println!("デッキにカードがありません。");
            }
        }
        if mulligan_counter_opponent.counter >= MULLIGAN_PENALTY_COUNT {
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
            }
        }
        mulligan_finish_event_writer.send(MulliganFinish);
    }
}

pub fn mulligan_finish(
    mut first_hand: ResMut<FirstHand>, // リソースとして受け取る
    mut first_hand_opponent: ResMut<FirstHandOpponent>,
    query: Query<(Entity, &Card, &Location), With<Player1>>,
    query_opponent: Query<(Entity, &Card, &Location), With<Player2>>,
    mut first_player: ResMut<PlayerSort>,
    mut next_controller: ResMut<NextState<Controller>>,
    query_first_energy: Query<(Entity, &Card, &Location), With<FirstEnergy>>, // 正しい構文
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_state2: ResMut<NextState<PlayState>>,
    mut mulligan_finish_event_reader: EventReader<MulliganFinish>,
) {
    for event in mulligan_finish_event_reader.read() {
        first_hand.cards.clear(); // 前回の結果をクリア
        println!("your first hands are:");

        for (entity, card, location) in query.iter() {
            if let Location::InHand = location {
                first_hand.cards.push(entity); // エンティティIDをリストに追加
                println!("{}, ", card.name);
            }
        }

        first_hand_opponent.cards.clear(); // 前回の結果をクリア
        println!("opponent first hands are:");

        for (entity, card, location) in query_opponent.iter() {
            if let Location::InHand = location {
                first_hand_opponent.cards.push(entity); // エンティティIDをリストに追加
                println!("{}, ", card.name);
            }
        }
        // 先行後攻決め

        let (turn_player, non_turn_player) = janken();

        first_player.firstplayer_id = turn_player;
        first_player.secondplayer_id = non_turn_player;
        match turn_player {
            1 => {
                next_controller.set(Controller::Player); // Player が先行の場合
            }
            2 => {
                next_controller.set(Controller::Opponent); // Opponent が先行の場合
            }
            _ => {
                println!("無効なプレイヤーIDです。");
            }
        }
        println!(
            "先行: Player {}, 後攻: Player {}",
            first_player.firstplayer_id, first_player.secondplayer_id,
        );
        //ファーストエナジーオープン
        println!("ファーストエナジーオープン！！");
        for (entity, card, location) in query_first_energy.iter() {
            commands.entity(entity).insert(FaceCondition::Open);
            println!("ファーストエナジーは、{}, です。", card.name)
        }
        //ステート移行
        next_state.set(GameState::Play);
        next_state2.set(PlayState::TurnStart);
        println!("対戦を開始します")
    }
}
pub fn check_last_hand(
    mut first_hand: ResMut<FirstHand>, // リソースとして受け取る
    query: Query<(Entity, &Card, &Location)>,
) {
    first_hand.cards.clear(); // 前回の結果をクリア
    println!("your first hands are:");

    for (entity, card, location) in query.iter() {
        if let Location::InHand = location {
            first_hand.cards.push(entity); // エンティティIDをリストに追加
            println!("{}, ", card.name);
        }
    }
}
pub fn choose_first(
    mut first_player: ResMut<PlayerSort>,
    mut next_controller: ResMut<NextState<Controller>>,
) {
    let (turn_player, non_turn_player) = janken();

    // turn_player を FirstPlayer リソースに格納
    first_player.firstplayer_id = turn_player;
    first_player.secondplayer_id = non_turn_player;
    match turn_player {
        1 => {
            next_controller.set(Controller::Player); // Player が先行の場合
        }
        2 => {
            next_controller.set(Controller::Opponent); // Opponent が先行の場合
        }
        _ => {
            println!("無効なプレイヤーIDです。");
        }
    }
    println!(
        "先行: Player {}, 後攻: Player {}",
        first_player.firstplayer_id, first_player.secondplayer_id,
    );
}

pub fn automatically_move_to_over_state(mut next_state: ResMut<NextState<MulliganState>>) {
    next_state.set(MulliganState::Over);
    println!("最終手札が確定しました")
}

pub fn automatically_move_to_play_state(
    mut next_state: ResMut<NextState<GameState>>,
    mut next_state2: ResMut<NextState<PlayState>>,
) {
    next_state.set(GameState::Play);
    next_state2.set(PlayState::TurnStart);
    println!("対戦を開始します")
}

pub fn first_energy_open(
    query: Query<(Entity, &Card, &Location), With<FirstEnergy>>, // 正しい構文
    mut commands: Commands,
) {
    println!("ファーストエナジーオープン！！");
    for (entity, card, location) in query.iter() {
        commands.entity(entity).insert(FaceCondition::Open);
        println!("ファーストエナジーは、{}, です。", card.name)
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // 出力をフラッシュしてプロンプトを表示
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim().to_string();

        // 入力が1, 2, 3のいずれかであることを確認
        if trimmed_input == "1" || trimmed_input == "2" || trimmed_input == "3" {
            return trimmed_input; // 有効な入力を返す
        } else {
            println!("無効な選択です。1, 2, または 3 を選んでください。");
            input.clear(); // 入力をクリアして再度プロンプトを表示
        }
    }
}



fn get_turn_choice(prompt: &str) -> String {
    let mut input = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // 出力をフラッシュしてプロンプトを表示
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim().to_string();

        // 入力が1または2であることを確認
        if trimmed_input == "1" || trimmed_input == "2" {
            return trimmed_input; // 有効な入力を返す
        } else {
            println!("無効な選択です。1または2を選んでください。");
            input.clear(); // 入力をクリアして再度プロンプトを表示
        }
    }
}

fn determine_winner(player1_choice: &str, player2_choice: &str) -> u32 {
    let player1_hand = match player1_choice {
        "1" => "グー",
        "2" => "チョキ",
        "3" => "パー",
        _ => {
            println!("無効な選択です。Player 1の選択を無効とします。");
            return 0; // 無効な場合はゼロを返す
        }
    };

    let player2_hand = match player2_choice {
        "1" => "グー",
        "2" => "チョキ",
        "3" => "パー",
        _ => {
            println!("無効な選択です。Player 2の選択を無効とします。");
            return 0; // 無効な場合はゼロを返す
        }
    };

    let outcomes: HashMap<&str, &str> = [("グー", "チョキ"), ("チョキ", "パー"), ("パー", "グー")]
        .iter()
        .cloned()
        .collect();

    if player1_hand == player2_hand {
        println!("あいこです。もう一度選んでください。");

        // あいこの場合、再度入力を受け付ける
        let new_player1_choice = get_user_input("Player 1の選択> ");
        let new_player2_choice = get_user_input("Player 2の選択> ");
        return determine_winner(&new_player1_choice, &new_player2_choice); // 再帰的に呼び出し
    } else if outcomes.get(player1_hand).unwrap() == &player2_hand {
        println!("Player 1の勝ちです！");
        return 1; // Player 1 の勝ち
    } else {
        println!("Player 2の勝ちです！");
        return 2; // Player 2 の勝ち
    }
}

fn decide_turn(winner: u32) -> (u32, u32) {
    let winner_choice = if winner == 1 {
        println!("Player 1: 先行または後攻を選んでください。(1: 先行, 2: 後攻)");
        get_turn_choice("")
    } else {
        println!("Player 2: 先行または後攻を選んでください。(1: 先行, 2: 後攻)");
        get_turn_choice("")
    };

    match winner_choice.as_str() {
        "1" => {
            println!("勝者は先行を選びました。");
            (winner, 3 - winner) // 1が勝者ならturn_playerはwinner、non_turn_playerは対戦相手
        }
        "2" => {
            println!("勝者は後攻を選びました。");
            (3 - winner, winner) // 2が勝者ならnon_turn_playerはwinner
        }
        _ => {
            println!("無効な選択肢です。");
            (0, 0) // 無効な場合はゼロを返す
        }
    }
}

fn janken() -> (u32, u32) {
    println!(
        "じゃんけんをします。Player 1: 1: グー、2: チョキ、3: パー のいずれかを選択してください。"
    );
    let player1_choice = get_user_input("Player 1の選択> ");

    println!("Player 2: 1: グー、2: チョキ、3: パー のいずれかを選択してください。");
    let player2_choice = get_user_input("Player 2の選択> ");

    let winner = determine_winner(&player1_choice, &player2_choice);

    // ターンプレイヤーと非ターンプレイヤーを決定
    let (turn_player, non_turn_player) = decide_turn(winner);

    (turn_player, non_turn_player) // ターンプレイヤーと非ターンプレイヤーを返す
}
