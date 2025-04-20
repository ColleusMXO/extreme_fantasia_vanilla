use bevy::prelude::*;
pub mod component;
pub mod systems;

pub mod resource;
use crate::game::GameState;
use crate::game::loading::systems::*;
use resource::*;
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InLibraryCards::default())
            .insert_resource(InLibraryCardsOpponent::default())
            .add_systems(
                OnEnter(GameState::Loading),
                (
                    setup_deck,
                    opponent_setup_deck,
                    setup_first_energy,
                    opponent_setup_first_energy,
                    my_cards_in_deck,
                    opponent_cards_in_deck,
                    list_facedown_cards_in_xzone,
                    list_facedown_cards_in_xzone_opponent, 
                    automatically_move_to_setup_state,
                )
                    .chain(),
            );
    }
}
