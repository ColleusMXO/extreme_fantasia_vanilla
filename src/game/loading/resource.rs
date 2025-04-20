use bevy::prelude::*;


#[derive(Default, Resource)]
pub struct InLibraryCards {
   pub cards: Vec<Entity>,
}
#[derive(Default, Resource)]
pub struct InLibraryCardsOpponent {
   pub cards: Vec<Entity>,
}
