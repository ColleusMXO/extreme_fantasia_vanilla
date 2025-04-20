use bevy::prelude::*;
#[derive(Default, Resource)]
pub struct FirstHand {
    pub cards: Vec<Entity>,
}

#[derive(Default, Resource)]
pub struct FirstHandOpponent {
    pub cards: Vec<Entity>,
}
#[derive(Default, Resource)] // Resourceトレイトを使ってリソースにする
pub struct PlayerSort {
    pub firstplayer_id: u32,
    pub secondplayer_id: u32, // 先行プレイヤーのID（1 または 2）
}

#[derive(Default, Resource)]
pub struct MulliganCounter {
    pub counter: u32,
}

#[derive(Default, Resource)]
pub struct MulliganCounterOpponent {
    pub counter: u32,
}