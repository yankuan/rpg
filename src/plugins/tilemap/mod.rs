// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfigs, Update, Plugin};


use self::systems::{tiles::spawn_tiles,movetiles::move_tiles};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod systems;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_tiles, move_tiles).run_if(in_state(GameState::Running)));
        //app.add_systems(Update, spawn_tiles.run_if(in_state(GameState::Running)));
    }
    
}
