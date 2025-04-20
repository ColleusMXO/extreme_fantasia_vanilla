use bevy::prelude::*;
use systems::game_over;

mod systems;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, game_over);
    }
}
