use bevy::prelude::*;
pub mod game_over;
pub mod loading;
pub mod play;
pub mod set_up;
pub mod resource;
mod systems;
use game_over::GameOverPlugin;
use systems::update_turn_player;
use loading::LoadingPlugin;
use play::{turn_end::TurnEndPlugin, PlayPlugin};
use set_up::SetupPlugin;


#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Setup,
    GameOver,
    Play,
}

use resource::*;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingPlugin)
            .add_plugins(SetupPlugin)
            .add_plugins(PlayPlugin)
            .add_plugins(GameOverPlugin)
            .insert_resource(FacedownXZoneCards::default())
            .insert_resource(FacedownXZoneCardsOpponent::default())
            .add_systems(Update, update_turn_player.run_if(in_state(GameState::Play)));
    }
}
