use bevy::prelude::*;

#[derive(Event)]
pub struct GameOver{
    pub winner_id: u32
}

#[derive(Event)]
pub struct SetupDraw;

#[derive(Event)]
pub struct RespinFzone;

#[derive(Event)]
pub struct RespinSzone;

