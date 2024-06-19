// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::{prelude::Component, reflect::Reflect,prelude::States};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect, States, Hash)]
pub enum PlayerState {
    #[default]
    Idle,
    Walk,
    Talk,
    Fight,
    Stunned,
    OnFight,//处于被攻击的状态
    UseMelee,
    UseMagic,
    UsePotion,
}
