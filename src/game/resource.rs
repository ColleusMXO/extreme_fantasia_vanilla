use bevy::prelude::*;

#[derive(Default, Resource)] // Resourceトレイトを使ってリソースにする
pub struct Winner {
    pub winner_id: u32,
}

#[derive(Default, Resource)]
pub struct FacedownXZoneCards {
    pub cards: Vec<Entity>,
}

#[derive(Default, Resource)]
pub struct FacedownXZoneCardsOpponent {
    pub cards: Vec<Entity>,
}