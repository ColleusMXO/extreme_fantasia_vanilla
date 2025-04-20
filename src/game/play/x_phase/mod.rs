use bevy::prelude::*;

pub mod resource;
mod systems;
use event::{ListFacedownCardsInXzone, XPhaseManagement, Xskill};
use systems::*;

use crate::game::GameState;

use super::{Controller, PlayState};
pub mod event;

pub struct XPhasePlugin;
impl Plugin for XPhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Xskill>()
            .add_event::<ListFacedownCardsInXzone>()
            .add_event::<XPhaseManagement>()
            .add_systems(OnEnter(PlayState::XPhase), x_phase_start)
            .add_systems(
                Update,
                (xskill, see_facedown_cards_in_xzone, x_phase_management)
                    .run_if(in_state(Controller::Player))
                    .run_if(in_state(PlayState::XPhase))
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (
                    xskill_opponent,
                    see_facedown_cards_in_xzone_opponent,
                    x_phase_management_opponent,
                )
                    .run_if(in_state(Controller::Opponent))
                    .run_if(in_state(PlayState::XPhase))
                    .run_if(in_state(GameState::Play)),
            );
    }
}
