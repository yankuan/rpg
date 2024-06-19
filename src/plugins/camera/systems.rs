// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use bevy::prelude::*;

use crate::plugins::player::components::Player;

use super::components::GameCamera;

pub fn update_game_camera(
    mut game_camera_query: Query<&mut Transform, With<GameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
) {

    if game_camera_query.is_empty() || player_query.is_empty() {
        return;
    }
   
    let mut game_camera_transform = game_camera_query
        .get_single_mut()
        .expect("0 or more than 1 game-camera found.");

     
        let player_transform = player_query
            .get_single()
            .expect("0 or more than 1 player found.");
        
        //println!("{:?}",player_transform.translation.x);
        //let left = 110.0;
        //let right = 112.0;
        let bottom = 72.111;
        let top = 229.81;
        // 判断相机是否超出上下边界
        let half_height = 0.5;
        let bottom_limit = bottom + half_height;
        let top_limit = top - half_height;
        let y = player_transform.translation.y.max(bottom_limit).min(top_limit);
        //let y = player_transform.translation.y;

        // 判断相机是否超出左右边界
        
        //let half_width = 0.5;
        //let left_limit = left + half_width;
        //let right_limit = right - half_width;
        //let x = player_transform.translation.x.max(left_limit).min(right_limit);    
        let x = player_transform.translation.x;

        game_camera_transform.translation = Vec3::new(
            x,
            y,
            100.,
        );
        
        
    
}