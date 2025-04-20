use bevy::prelude::*;

use super::resource::*;
use crate::game::loading::component::*;
use crate::game::loading::resource::*;
use crate::game::set_up::resource::*;
use std::io::{self, Write};

fn sum_energy_in_szone(query: Query<(&Card, &Location)>) {
    let mut usable_energy = 0;
    for (card, location) in query.iter() {
        if let Location::SZone = location {
            usable_energy += card.energy; // energyを加算
        }
    }
    println!("Total usable energy in SZone: {}", usable_energy); // 結果を表示
}

fn select_a_card_in_hand(query: Query<(Entity, &Card, &Location)>, mut commands: Commands) {
    let mut cards_in_hand = Vec::new();

    println!("Cards in your hand are:");

    for (entity, card, location) in query.iter() {
        if let Location::InHand = location {
            cards_in_hand.push(entity); // エンティティIDを保存
            println!("{}: {}", cards_in_hand.len(), card.name); // 番号を振って表示
        }
    }

    if cards_in_hand.is_empty() {
        println!("No cards in hand.");
        return;
    }

    println!("Select a card by number (1-{}):", cards_in_hand.len());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if let Ok(selected_index) = input.trim().parse::<usize>() {
        if selected_index > 0 && selected_index <= cards_in_hand.len() {
            let selected_entity = cards_in_hand[selected_index - 1];

            if let Ok((_, card, _)) = query.get(selected_entity) {
                commands.entity(selected_entity).insert(Location::FZone);
                println!(" {} was moved to Fzone.", card.name); // カード名を表示
            }
            println!("You selected card with Entity ID: {:?}", selected_entity);
        } else {
            println!("Invalid selection.");
        }
    } else {
        println!("Please enter a valid number.");
    }
}
