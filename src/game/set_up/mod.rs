use bevy::prelude::*;
mod systems;
use systems::*;
mod event;
pub mod resource;
use crate::game::*;
use event::*;
use resource::*;

use super::play::{Controller, set_up_phase::event::GameOver};
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FirstHand::default())
            .insert_resource(FirstHandOpponent::default())
            .insert_resource(PlayerSort::default())
            .init_state::<MulliganState>()
            .add_event::<GameOver>()
            .add_event::<Mulligan>()
            .add_event::<MulliganManagement>()
            .add_event::<MulliganFinish>()
            .add_event::<MulliganOpponent>()
            .add_event::<MulliganFinishOpponent>()
            .add_event::<MulliganManagementOpponent>()
            .add_event::<MulliganPenalty>()
            .insert_resource(MulliganCounter::default())
            .insert_resource(MulliganCounterOpponent::default())
            .add_systems(OnEnter(GameState::Setup), mulligan_start)
            .add_systems(
                Update,
                (mulligan, mulligan_finish, mulligan_management, mulligan_opponent, mulligan_management_opponent, mulligan_penalty).run_if(in_state(GameState::Setup)),
            );
    }
}
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MulliganState {
    #[default]
    Default,
    First,
    Second,
    Over,
}
