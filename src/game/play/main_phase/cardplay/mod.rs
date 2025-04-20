use bevy::prelude::*;

pub mod systems;
use systems::*;
pub mod event;
use crate::game::{
    GameState,
    play::{Controller, PlayState},
};
use event::*;
pub struct CardPlayPlugin;
impl Plugin for CardPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayStart>()
            .add_event::<SelectHand>()
            .add_event::<ChooseCard>()
            .add_event::<PlayTacticsSpinSzone>()
            .add_event::<PlayFighterSpinSzone>()
            .add_event::<PlayFighterFromHand>()
            .add_event::<PlayTacticsFromHand>()
            .add_event::<CheckPlayedCardType>()
            .add_event::<PlayZeroEnergyFighterFromHand>()
            .add_event::<PlayFighterFinish>()
            .add_event::<PlayTacticsFinish>()
            .add_event::<EnergyAdjustment>()
            .add_event::<PlayTacticsWithPenaltyFromHand>()
            .add_systems(
                Update,
                (
                    play_start,
                    select_hand,
                    check_played_card_type,
                    play_fighter_spin_szone,
                    play_zero_energy_fighter,
                    play_fighter_finish,
                    play_fighter_from_hand,
                    play_tactics_from_hand,
                    play_tactics_spin_szone,
                    play_tactics_finish,
                    energy_adjustment,
                    play_tactics_with_penalty_from_hand,
                )
                    .run_if(in_state(Controller::Player))
                    .run_if(in_state(PlayState::MainPhase))
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (
                    play_start,
                    select_hand_opponent,
                    check_played_card_type,
                    play_fighter_spin_szone_opponent,
                    play_zero_energy_fighter,
                    play_fighter_finish,
                    play_fighter_from_hand,
                    play_tactics_from_hand,
                    play_tactics_spin_szone_opponent,
                    play_tactics_finish,
                    energy_adjustment_opponent,
                    play_tactics_with_penalty_from_hand,
                )
                    .run_if(in_state(Controller::Opponent))
                    .run_if(in_state(PlayState::MainPhase))
                    .run_if(in_state(GameState::Play)),
            );
    }
}
