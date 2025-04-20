use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Player1Hand {
    pub cards: Vec<Entity>,
}
#[derive(Default, Resource)]
pub struct Player1SZone {
    pub cards: Vec<Entity>,
}

#[derive(Default, Resource)]
pub struct Player1FZone {
    pub cards: Vec<Entity>,
}
#[derive(Default, Resource)]
pub struct Player1XZone {
    pub cards: Vec<Entity>,
}

#[derive(Default, Resource)]
pub struct Player2Hand {
    pub cards: Vec<Entity>,
}



#[derive(Default, Resource)]
pub struct Player2SZone {
    pub cards: Vec<Entity>,
}

#[derive(Default, Resource)]
pub struct Player2XZone {
    pub cards: Vec<Entity>,
}


#[derive(Default, Resource)]
pub struct Player2FZone {
    pub cards: Vec<Entity>,
}
