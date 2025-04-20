use super::component::*; // Cardをインポート
use super::resource::*;
use crate::game::GameState;
use crate::game::resource::*;
use bevy::prelude::*;

pub fn show_information(query: Query<(&Card, &Location)>) {
    for (card, location) in query.iter() {
        let location_str = match location {
            Location::InHand => "InHand",
            Location::SZone => "SZone",
            Location::XZone => "XZone",
            Location::FZone => "FZone",
            Location::InLibrary => "InLibrary",
            Location::GraveYard => "GraveYard",
        };

        println!(
            "Card Name: {},  Energy: {}, Location:{}",
            card.name, card.energy, location_str
        );
    }
}

pub fn my_cards_in_deck(
    mut in_library_cards: ResMut<InLibraryCards>, // リソースとして受け取る
    query: Query<(Entity, &Card, &Location), With<Player1>>,
) {
    in_library_cards.cards.clear(); // 前回の結果をクリア
    println!("Cards in player1 deck are:");

    for (entity, card, location) in query.iter() {
        if let Location::InLibrary = location {
            in_library_cards.cards.push(entity); // エンティティIDをリストに追加
            println!("{}, ", card.name);
        }
    }
}

pub fn opponent_cards_in_deck(
    mut in_library_cards_opponent: ResMut<InLibraryCardsOpponent>, // リソースとして受け取る
    query: Query<(Entity, &Card, &Location), With<Player2>>,
) {
    in_library_cards_opponent.cards.clear(); // 前回の結果をクリア
    println!("Cards in player2 deck are:");

    for (entity, card, location) in query.iter() {
        if let Location::InLibrary = location {
            in_library_cards_opponent.cards.push(entity); // エンティティIDをリストに追加
            println!("{}, ", card.name);
        }
    }
}

pub fn setup_first_energy(mut commands: Commands) {
    let my_entity = commands
        .spawn((
            Card {
                name: String::from("勇敢のエナジー"),
                energy: 2,
            },
            Location::SZone,
            SpinCondition::ReSpin,
            FaceCondition::Facedown,
            FirstEnergy {},
            Player1 {},
        ))
        .id();

    println!("{:?} is spawned", my_entity);
}

pub fn opponent_setup_first_energy(mut commands: Commands) {
    let my_entity = commands
        .spawn((
            Card {
                name: String::from("情熱のエナジー"),
                energy: 2,
            },
            Location::SZone,
            SpinCondition::ReSpin,
            FaceCondition::Facedown,
            FirstEnergy {},
            Player2 {},
        ))
        .id();

    println!("{:?} is spawned", my_entity);
}

pub fn opponent_setup_deck(mut commands: Commands) {
    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("飛翔のタカノメ"),
                    energy: 1,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Fighter {
                    power: 500,
                    summoned_sick: true,
                },
                Guard {},
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("十字紋のカクレボウ"),
                    energy: 2,
                },
                Location::InLibrary,
                SpinCondition::Spin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 1000,
                    summoned_sick: false,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("匠のテダマ"),
                    energy: 3,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 2000,
                    summoned_sick: false,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ツバサ・マシン"),
                    energy: 3,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 2000,
                    summoned_sick: false,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("背水のフサギテ"),
                    energy: 4,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 3000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("先陣のホムラザクラ"),
                    energy: 4,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 3000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("策謀のエンジュ"),
                    energy: 5,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 3000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("参戦のガントッパ"),
                    energy: 5,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 4000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("灼熱旋風のランセ"),
                    energy: 6,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 6000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("大筒掃射のダイハッパ"),
                    energy: 8,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 6000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("悪辣鬼顔のシュラモン"),
                    energy: 7,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 6000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }
    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("決戦突破のダイコガネ"),
                    energy: 8,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 8000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("幽炎のラゲン"),
                    energy: 6,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 5000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("大龍将グレン・ブレイド"),
                    energy: 10,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 10000,
                    summoned_sick: true,
                },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("必勝成敗の式目"),
                    energy: 0,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Tactics { required_energy: 6 },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("煉獄"),
                    energy: 2,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 5 },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ブレイジング・エリア"),
                    energy: 0,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 4 },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("クロガネ・エイド"),
                    energy: 2,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 3 },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("エナジー・ダンス"),
                    energy: 1,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 2 },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ゼキの工房"),
                    energy: 1,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 4 },
                Player2 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }
}

pub fn automatically_move_to_setup_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Setup);
    println!("ローディング終了しました。")
}

pub fn list_facedown_cards_in_xzone(
    query: Query<(Entity, &Card, &Location, &FaceCondition), With<Player1>>,
    mut in_xzone_card: ResMut<FacedownXZoneCards>,
) {
    println!("初期設定で裏向きでXゾーンにあるカードをロードしました。");

    // 裏向きのカードをリストアップ ただ、順番が滅茶滅茶な事に注意!!これはデバッグ用の機能
    for (entity, card, location, face_condition) in query.iter() {
        if let Location::XZone = location {
            if let FaceCondition::Facedown = face_condition {
                in_xzone_card.cards.push(entity); // エンティティIDを保存
                println!("{}: {:?}", in_xzone_card.cards.len(), entity); // 番号を振って表示
            }
        }
    }
}

pub fn list_facedown_cards_in_xzone_opponent(
    query: Query<(Entity, &Card, &Location, &FaceCondition), With<Player2>>,
    mut in_xzone_card: ResMut<FacedownXZoneCardsOpponent>,
) {
    println!("初期設定で裏向きで相手のXゾーンにあるカードをロードしました。");

    // 裏向きのカードをリストアップ ただ、順番が滅茶滅茶な事に注意!!これはデバッグ用の機能
    for (entity, card, location, face_condition) in query.iter() {
        if let Location::XZone = location {
            if let FaceCondition::Facedown = face_condition {
                in_xzone_card.cards.push(entity); // エンティティIDを保存
                println!("{}: {:?}", in_xzone_card.cards.len(), entity); // 番号を振って表示
            }
        }
    }
}

pub fn setup_deck(mut commands: Commands) {
    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("リトルナイト"),
                    energy: 1,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Fighter {
                    power: 1000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("なまけ騎士　ハームゥ"),
                    energy: 0,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Fighter {
                    power: 500,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("アサシン・タイクーン"),
                    energy: 2,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Fighter {
                    power: 2000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("騎士ニャタタ・ビィ"),
                    energy: 3,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Fighter {
                    power: 2000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("密林の騎士リンキー"),
                    energy: 2,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Fighter {
                    power: 1000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }
    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ワンダフル・ウィザード　ドギィ"),
                    energy: 4,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Guard {},
                Fighter {
                    power: 1000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ワンダフルウィザード・ワッチチ"),
                    energy: 3,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Fighter {
                    power: 1000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }
    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ワンダフル・ウィザード　ゴン"),
                    energy: 3,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Fighter {
                    power: 500,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("三剣士のトランスロ"),
                    energy: 4,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Fighter {
                    power: 3000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("三剣士のソードホーク"),
                    energy: 6,
                },
                Location::InLibrary,
                SpinCondition::Spin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 5000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("師範マスター・ゼブラ"),
                    energy: 7,
                },
                Location::InLibrary,
                SpinCondition::Spin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 5000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("若き英雄　ナポ"),
                    energy: 6,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 6000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("アサシン・ロー"),
                    energy: 5,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 5000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("雷の戦士ポーラ"),
                    energy: 5,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 4000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("疾風の騎士ゴート・パラディン"),
                    energy: 7,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Open,
                Guard {},
                Fighter {
                    power: 6000,
                    summoned_sick: true,
                },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..3 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("エナジーベリー"),
                    energy: 2,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 1 },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ハームゥのラッパ"),
                    energy: 1,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 2 },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..1 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("反撃の剣撃"),
                    energy: 3,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 5 },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ふんわりトルネード"),
                    energy: 1,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 3 },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("あつあつフレア"),
                    energy: 3,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 4 },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }

    for n in 0..2 {
        let my_entity = commands
            .spawn((
                Card {
                    name: String::from("ホキュウムッシ"),
                    energy: 3,
                },
                Location::InLibrary,
                SpinCondition::ReSpin,
                FaceCondition::Facedown,
                Tactics { required_energy: 3 },
                Player1 {},
            ))
            .id();

        println!("{:?} is spawned", my_entity);
    }
}
