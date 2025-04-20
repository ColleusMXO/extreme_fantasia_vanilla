use bevy::prelude::*;
mod systems;
use crate::game::{play::PlayState, resource::Winner, GameState};
use systems::*;
pub struct TurnEndPlugin;
impl Plugin for TurnEndPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Winner::default())
            .add_systems(OnEnter(PlayState::TurnEnd), change_turn_player.run_if(in_state(PlayState::TurnEnd)).run_if(in_state(GameState::Play)));
    }
}
