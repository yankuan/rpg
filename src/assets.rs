// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // Images.
    #[asset(path = "images/player.png")]
    pub image_player: Handle<Image>,
    #[asset(path = "images/new3.png")]
    pub image_tileset: Handle<Image>,
    #[asset(path = "images/new3_qj.png")]
    pub image_qj_tileset: Handle<Image>, //前景
    #[asset(path = "images/new3_sky.png")]
    pub image_sky_tileset: Handle<Image>, //天空
    #[asset(path = "images/new3_yun.png")]
    pub image_yun_tileset: Handle<Image>, //云
    #[asset(path = "images/enemy.png")]
    pub image_enemy: Handle<Image>,


    // Fonts.
    #[asset(path = "fonts/qingfengfuan.ttf")]
    pub font_medium: Handle<Font>,
    #[asset(path = "fonts/qingfengfuan.ttf")]
    pub font_bold: Handle<Font>,


}
