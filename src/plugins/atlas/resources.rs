// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Handle, Resource},
    sprite::TextureAtlas,
};

#[derive(Resource)]
pub struct GameAtlases {
    pub player: Handle<TextureAtlas>,
    pub tileset: Handle<TextureAtlas>,
    pub qj_tileset: Handle<TextureAtlas>,
    pub sky_tileset: Handle<TextureAtlas>,
    pub yun_tileset: Handle<TextureAtlas>,
    pub enemy: Handle<TextureAtlas>
}
