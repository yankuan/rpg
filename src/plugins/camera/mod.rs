// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, Plugin, Update, IntoSystemConfigs};

use crate::states::AppState;

use self::systems::update_game_camera;
pub mod systems;
pub mod bundles;
pub mod components;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (update_game_camera).run_if(in_state(AppState::InGame)));
    }
}
