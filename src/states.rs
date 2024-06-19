// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States, Reflect)]
pub enum AppState {
    #[default]
    LoadingAssets,
    BuildingAtlases,
    //Introduction,
    InGame,
    Combat,
    GameOver,
    GameWin,
}
