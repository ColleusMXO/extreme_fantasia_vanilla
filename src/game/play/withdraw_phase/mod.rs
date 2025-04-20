use bevy::prelude::*;
use event::{
    CheckCondition, PickFighterToWithdraw, Rethink, Withdraw, WithdrawFinish, WithdrawManagement,
};
use resource::{FieldCondition, FieldConditionOpponent};
mod systems;
use crate::game::GameState;

use super::{Controller, PlayState};
use systems::*;
mod event;
pub mod resource;

pub struct WithdrawPhasePlugin;
impl Plugin for WithdrawPhasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FieldCondition::default())
            .insert_resource(FieldConditionOpponent::default())
            .add_event::<Withdraw>()
            .add_event::<PickFighterToWithdraw>()
            .add_event::<CheckCondition>()
            .add_event::<WithdrawManagement>()
            .add_event::<WithdrawFinish>()
            .add_systems(
                OnEnter(PlayState::WithdrawPhase),
                (withdraw_start).run_if(in_state(GameState::Play)).run_if(in_state(PlayState::WithdrawPhase)),
            )
            .add_systems(
                Update,
                (
                    check_condition,
                    withdraw_management,
                    pick_fighter_to_withdraw,
                    withdraw,
                )
                    .run_if(in_state(Controller::Player)).run_if(in_state(PlayState::WithdrawPhase)).run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (
                    check_condition_opponent,
                    withdraw_management_opponent,
                    pick_fighter_to_withdraw_opponent,
                    withdraw_opponent,
                )
                    .run_if(in_state(Controller::Opponent)).run_if(in_state(PlayState::WithdrawPhase)).run_if(in_state(GameState::Play)),
            );
    }
}
