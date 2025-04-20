use bevy::prelude::*;
pub mod game;
pub mod main_menu;
mod systems;
use systems::*; 
use game::loading::systems::setup_deck;
use game::play::PlayState;
use game::set_up::MulliganState;
use game::GameState;
use game::{GamePlugin, loading::LoadingPlugin};
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .init_state::<MulliganState>()
        .add_plugins(GamePlugin)
        .add_systems(Update, exit_game)
        .run();
}
