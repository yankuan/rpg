// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct AnimationStartEvent {
    pub target_entity: Entity,
    pub clip_name: String,
    pub reset_timer: bool,
}
