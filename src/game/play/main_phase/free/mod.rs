use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod event;
use event::*;

use crate::game::{play::{Controller, PlayState}, GameState};
pub struct FreePlugin;
impl Plugin for FreePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SituationLog>()
            .add_event::<MainPhaseManagement>()
            .add_event::<ContinueMainPhase>()
            .add_systems(
                Update,
                (situation_log, main_phase_management, continue_main_phase)
                    .run_if(in_state(Controller::Player))
                    .run_if(in_state(PlayState::MainPhase))
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (
                    situation_log_opponent,
                    main_phase_management,
                    continue_main_phase,
                )
                    .run_if(in_state(Controller::Opponent))
                    .run_if(in_state(PlayState::MainPhase))
                    .run_if(in_state(GameState::Play)),
            );
    }
}
