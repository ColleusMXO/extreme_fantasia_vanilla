use bevy::prelude::*;

use super::free::event::SituationLog;


pub fn main_phase_start(mut situation_log_event_writer: EventWriter<SituationLog>) {
    situation_log_event_writer.send(SituationLog);
    println!("メインフェーズ開始")
}