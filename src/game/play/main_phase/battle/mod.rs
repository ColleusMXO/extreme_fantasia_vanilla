use bevy::prelude::*;
pub mod event;
pub mod systems;
use systems::*;

use crate::game::{
    GameState,
    play::{Controller, PlayState},
};
use event::*;
pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackStart>()
            .add_event::<Attack>()
            .add_event::<SelectAttacker>()
            .add_event::<BeingAttacked>()
            .add_event::<FirstEnergyPenalty>()
            .add_event::<AttackEnd>()
            .add_event::<AttackFZone>()
            .add_event::<AttackSZone>()
            .add_event::<AttackTactics>()
            .add_event::<Aiuchi>()
            .add_event::<DestroyedTactics>()
            .add_event::<DestroyedFighter>()
            .add_event::<BattleTacticsWin>()
            .add_event::<BattleFighterWin>()
            .add_event::<ChooseWithdrawOrGY>()
            .add_event::<OpponentWithdrawOrGY>()
            .add_event::<ChooseGuard>()
            .add_systems(
                Update,
                (
                    attack_start,
                    select_attacker,
                    choose_card_being_attacked,
                    handle_being_attacked,
                    first_energy_penalty,
                    attack_tactics,
                    attack_szone,
                    attack_fzone,
                    choose_withdraw_or_gy,
                    opponent_choose_withdraw_or_gy,
                    attack_end,
                    choose_guard,
                    aiuchi,
                )
                    .run_if(in_state(Controller::Player))
                    .run_if(in_state(PlayState::MainPhase))
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (
                    attack_start,
                    select_attacker_opponent,
                    choose_card_being_attacked_opponent,
                    handle_being_attacked,
                    first_energy_penalty_opponent,
                    attack_tactics_opponent,
                    attack_szone_opponent,
                    attack_fzone,
                    choose_withdraw_or_gy_opponent,
                    opponent_choose_withdraw_or_gy_opponent,
                    attack_end,
                    choose_guard_opponent,
                    aiuchi_opponent,
                )
                    .run_if(in_state(Controller::Opponent))
                    .run_if(in_state(PlayState::MainPhase)).run_if(in_state(GameState::Play)),
            );
    }
}
