use bevy::prelude::*;

#[derive(Default, Resource)] // Resourceトレイトを使ってリソースにする
pub struct FieldCondition {
    pub has_fzone_card: bool,
    pub has_xzone_card: bool,
}

#[derive(Default, Resource)] // Resourceトレイトを使ってリソースにする
pub struct FieldConditionOpponent {
    pub has_fzone_card: bool,
    pub has_xzone_card: bool,
}