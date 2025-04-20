use bevy::{ecs::entity, prelude::*};

#[derive(Event)]
pub struct ListFacedownCardsInXzone;

#[derive(Event)]
pub struct XPhaseManagement;

#[derive(Event)]
pub struct Xskill{
    pub first_energy_id: Entity, 
}

#[derive(Event)]
pub struct OpenedCards{
    pub opened_cards: Vec<Entity>, 
}