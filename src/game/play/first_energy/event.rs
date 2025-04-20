use bevy::prelude::*;

#[derive(Event)]
pub struct Open{
    pub open_cards_id: Entity, 
}