use bevy::prelude::*;
mod systems;
use crate::game::GameState;
use crate::game::play::resource::*;
use systems::*;

use super::PlayState;
pub struct TurnStartPlugin;
impl Plugin for TurnStartPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnPlayer::default())
            .init_resource::<TurnCount>()
            .add_systems(
                OnEnter(PlayState::TurnStart),
                (turn_start, automatically_move_to_setup_phase)
                    .run_if(in_state(PlayState::TurnStart)).run_if(in_state(GameState::Play))
                    .chain(),
            );
    }
}
