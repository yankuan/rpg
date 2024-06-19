// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;



use self::systems::{
    animation::player_animation,
    movement::{player_movement, player_movement_input, player_movement_reset}
};
use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod states;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(OnEnter(GameState::Running), player_movement_reset);
            
            app.add_systems(
                Update,
                (
                    player_movement_input.before(player_movement),
                    player_animation,
                    player_movement,
                ).run_if(in_state(GameState::Running)),
            );
    }
}
