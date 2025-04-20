use bevy::prelude::*;

#[derive(Component)]
pub struct Fighter {
    pub power: u32, 
    pub summoned_sick: bool,
}

#[derive(Component)]
pub struct Player1{}

#[derive(Component)]
pub struct Guard{}

#[derive(Component)]
pub struct Player2{}

#[derive(Component)]
pub struct FirstEnergy {}

#[derive(Component)]
pub struct Tactics {
    pub required_energy: u32, 
}

#[derive(Component)]
pub struct Card {
    pub name: String,
    pub energy: u32,
}

#[derive(Component, Default)]
pub enum Location {
    InHand,
    SZone,
    XZone,
    FZone,
    GraveYard, 
    #[default]
    InLibrary,
}
#[derive(Component, Default)]
pub enum SpinCondition {
    Spin,
    #[default]
    ReSpin,
}

#[derive(Component, Default)]
pub enum FaceCondition {
    Open,
    #[default]
    Facedown,
}

