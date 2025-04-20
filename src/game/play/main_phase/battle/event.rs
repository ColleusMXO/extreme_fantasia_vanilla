use bevy::prelude::*;

#[derive(Event)]
pub struct AttackStart;


#[derive(Event)]
pub struct AttackEnd{
    pub attacker_id: Entity
}

#[derive(Event)]
pub struct SelectAttacker;


#[derive(Event)]
pub struct Attack {
    pub attacker_id: Entity,
}

#[derive(Event)]
pub struct ChooseGuard {
    pub attacker_id: Entity,
    pub attacked_card_id: Entity,
}

#[derive(Event)]
pub struct BeingAttacked {
    pub attacker_id: Entity,
    pub attacked_card_id: Entity,
}


#[derive(Event)]
pub struct FirstEnergyPenalty{
    pub attacker_id: Entity, 
}

#[derive(Event)]
pub struct BattleTacticsWin{
    pub attacker_id: Entity, 
}

#[derive(Event)]
pub struct BattleFighterWin{
    pub winner_id: Entity, 
}

#[derive(Event)]
pub struct DestroyedTactics{
    pub destroyed_tactics_id: Entity, 
}

#[derive(Event)]
pub struct DestroyedFighter{
    pub destroyed_fighter_id: Entity, 
}

#[derive(Event)]
pub struct ChooseWithdrawOrGY{
    pub attacker_id: Entity,
    pub attacked_card_id: Entity,
}

#[derive(Event)]
pub struct OpponentWithdrawOrGY{
    pub attacker_id: Entity,
    pub attacked_card_id: Entity,
}

#[derive(Event)]
pub struct Aiuchi{
    pub attacker_id: Entity,
    pub attacked_card_id: Entity,
}


#[derive(Event)]
pub struct AttackTactics{
    pub attacker_id:Entity, 
    pub attacked_card_id: Entity
}

#[derive(Event)]
pub struct AttackFZone{
    pub attacker_id:Entity, 
    pub attacked_card_id: Entity
}

#[derive(Event)]
pub struct AttackSZone{
    pub attacker_id:Entity, 
    pub attacked_card_id: Entity
}


