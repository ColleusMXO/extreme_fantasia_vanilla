use bevy::prelude::*;

#[derive(Event)]
pub struct Withdraw {
    pub id: Entity,
}

#[derive(Event)]
pub struct WithdrawFinish {
    pub id: Entity,
}

#[derive(Event)]
pub struct PickFighterToWithdraw;

#[derive(Event)]
pub struct Rethink;

#[derive(Event)]
pub struct CheckCondition;

#[derive(Event)]
pub struct WithdrawManagement;

#[derive(Event)]
pub struct CheckSzoneCondition;

