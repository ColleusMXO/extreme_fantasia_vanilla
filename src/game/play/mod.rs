use bevy::prelude::*;
use main_phase::MainPhasePlugin;
use set_up_phase::SetupPhasePlugin;
use turn_end::TurnEndPlugin;
use turn_start::TurnStartPlugin;
use withdraw_phase::WithdrawPhasePlugin;
use x_phase::XPhasePlugin;
pub mod main_phase;
pub mod resource;
pub mod set_up_phase;
pub mod first_energy;
mod systems;
pub mod turn_end;
pub mod turn_start;
pub mod withdraw_phase;
pub mod x_phase;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PlayState {
    #[default]
    Initial, 
    TurnStart,
    SetupPhase,
    WithdrawPhase,
    XPhase,
    MainPhase,
    TurnEnd,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Controller {
    #[default]
    Player,
    Opponent,
}

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayState>()
            .init_state::<Controller>()
            .add_plugins(TurnStartPlugin)
            .add_plugins(SetupPhasePlugin)
            .add_plugins(WithdrawPhasePlugin)
            .add_plugins(XPhasePlugin)
            .add_plugins(MainPhasePlugin)
            .add_plugins(TurnEndPlugin);
    }
}
