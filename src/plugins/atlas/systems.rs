// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Assets, Commands, NextState, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

use crate::{assets::GameAssets, plugins::atlas::resources::GameAtlases, states::AppState};

pub(super) fn build_atlases(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let texture_atlas_player = TextureAtlas::from_grid(
        game_assets.image_player.clone(),
        Vec2::new(32.0, 32.0),
        4,
        9,
        None,
        None,
    );

    let texture_atlas_tileset = TextureAtlas::from_grid(
        game_assets.image_tileset.clone(),
        Vec2::new(32.0, 32.0),
        30,
        12,
        None,
        None,
    );

    let texture_atlas_qj_tileset = TextureAtlas::from_grid(
        game_assets.image_qj_tileset.clone(),
        Vec2::new(32.0, 32.0),
        20,
        12,
        None,
        None,
    );

    let texture_atlas_sky_tileset = TextureAtlas::from_grid(
        game_assets.image_sky_tileset.clone(),
        Vec2::new(32.0, 32.0),
        30,
        12,
        None,
        None,
    );

    let texture_atlas_yun_tileset = TextureAtlas::from_grid(
        game_assets.image_yun_tileset.clone(),
        Vec2::new(32.0, 32.0),
        30,
        12,
        None,
        None,
    );


    let texture_atlas_enemy = TextureAtlas::from_grid(
        game_assets.image_enemy.clone(),
        Vec2::new(15.0, 16.0),
        6,
        2,
        None,
        None,
    );
   
    commands.insert_resource(GameAtlases {
        player: texture_atlases.add(texture_atlas_player),
        tileset: texture_atlases.add(texture_atlas_tileset),
        qj_tileset: texture_atlases.add(texture_atlas_qj_tileset), //前景 大树
        sky_tileset: texture_atlases.add(texture_atlas_sky_tileset), //后景 天空
        yun_tileset: texture_atlases.add(texture_atlas_yun_tileset), //后景 云
        enemy: texture_atlases.add(texture_atlas_enemy),

    });

    next_state.set(AppState::InGame);
}
