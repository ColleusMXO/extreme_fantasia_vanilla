use bevy::prelude::*;

mod battle;
mod cardplay;
mod event;
mod free;
mod resource;
mod systems;

use battle::{BattlePlugin, systems::*};
use cardplay::{CardPlayPlugin, systems::*};
use event::*;
use free::FreePlugin;
use resource::*;

use crate::game::GameState;

use super::PlayState;
use systems::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MainPhaseState {
    #[default]
    Free,
    Battle,
    CardPlay,
}

pub struct MainPhasePlugin;
impl Plugin for MainPhasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Player1FZone::default())
            .insert_resource(Player2FZone::default())
            .insert_resource(Player1SZone::default())
            .insert_resource(Player2SZone::default())
            .insert_resource(Player1XZone::default())
            .insert_resource(Player2XZone::default())
            .insert_resource(Player1Hand::default())
            .insert_resource(Player2Hand::default())
            .init_state::<MainPhaseState>()
            .add_systems(OnEnter(PlayState::MainPhase), main_phase_start.run_if(in_state(PlayState::MainPhase)).run_if(in_state(GameState::Play)))
            .add_plugins(FreePlugin)
            .add_plugins(BattlePlugin)
            .add_plugins(CardPlayPlugin);
    }
}
