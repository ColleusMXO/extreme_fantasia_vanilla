use bevy::prelude::*;
#[derive(Event)]
pub struct SelectHand;
#[derive(Event)]
pub struct ChooseCard;
#[derive(Event)]
pub struct PlayStart;
#[derive(Event)]
pub struct CheckPlayedCardType {
    pub played_card_id: Entity,
}

#[derive(Event)]
pub struct PlayFighterSpinSzone {
    pub played_card_id: Entity,
}
#[derive(Event)]
pub struct PlayFighterFromHand {
    pub played_card_id: Entity,
    pub spin_cards_list: Vec<Entity>,
}
#[derive(Event)]
pub struct PlayZeroEnergyFighterFromHand {
    pub played_card_id: Entity,
}

#[derive(Event)]
pub struct PlayFighterFinish {
    pub played_card_id: Entity,
}

#[derive(Event)]
pub struct PlayTacticsFinish {
    pub played_card_id: Entity,
}

#[derive(Event)]
pub struct PlayTacticsSpinSzone {
    pub played_card_id: Entity,
}

#[derive(Event)]
pub struct PlayTacticsFromHand {
    pub played_card_id: Entity,
    pub spin_cards_list: Vec<Entity>,
}

#[derive(Event)]
pub struct PlayTacticsWithPenaltyFromHand {
    pub played_card_id: Entity,
    pub spin_cards_list: Vec<Entity>,
    pub penalty_card_list: Vec<Entity>,
}

#[derive(Event)]
pub struct EnergyAdjustment {
    pub played_card_id: Entity,
    pub spin_cards_list: Vec<Entity>,
    pub penalty_energy: u32,
}
