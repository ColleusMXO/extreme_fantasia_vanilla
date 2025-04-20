use bevy::prelude::*;

#[derive(Resource)]
pub struct TurnCount {
    pub value: u32,
}

impl Default for TurnCount {
    fn default() -> Self {
        TurnCount { value: 0 }
    }
}

#[derive(Default, Resource)] // Resourceトレイトを使ってリソースにする
pub struct TurnPlayer {
    pub turnplayer_id: u32,
    pub non_turnplayer_id: u32,
}