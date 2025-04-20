use bevy::prelude::*;
pub mod event;
mod systems;
use crate::game::{GameState, play::PlayState, resource::Winner};
use event::{RespinFzone, RespinSzone, SetupDraw};
use systems::*;

use super::Controller;
pub struct SetupPhasePlugin;
impl Plugin for SetupPhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetupDraw>()
            .add_event::<RespinFzone>()
            .add_event::<RespinSzone>()
            .insert_resource(Winner::default())
            .add_systems(
                OnEnter(PlayState::SetupPhase),
                set_up_start.run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (set_up_draw, respin_fzone_cards, respin_szone_cards)
                    .run_if(in_state(Controller::Player))
                    .run_if(in_state(PlayState::SetupPhase))
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (
                    set_up_draw_opponent,
                    respin_fzone_cards_opponent,
                    respin_szone_cards_opponent,
                )
                    .run_if(in_state(Controller::Opponent))
                    .run_if(in_state(PlayState::SetupPhase))
                    .run_if(in_state(GameState::Play)),
            );
    }
}
