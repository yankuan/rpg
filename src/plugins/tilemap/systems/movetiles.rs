// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;

use crate::plugins::{tilemap::components::Tilemap, camera::components::GameCamera};

pub fn move_tiles(
    mut tilemap_query: Query<
        (
            &Name,
            &mut Transform
        ),
        With<Tilemap>,
    >,
    delta: Res<Time>,
    mut game_camera_query: Query<&mut Transform, (With<GameCamera>, Without<Tilemap>)>,
) {
    
    for (name,mut tile_transform, ) in tilemap_query.iter_mut() {
        //println!("{}",name);
        
        if name.to_string() == "yunTilemap" {
            let mo = Vec2::new(1.0, 0.0);
            let sky_velocity = mo * 200.0 * delta.delta_seconds();
            tile_transform.translation += sky_velocity.extend(0.0) * delta.delta_seconds();
            let game_camera_transform = game_camera_query
            .get_single_mut().expect("error");
            //println!("{}",game_camera_transform.translation.x);
            //println!("{}",tile_transform.translation.x);
            if game_camera_transform.translation.x - tile_transform.translation.x < -120. {
                println!("aaaa");
                tile_transform.translation.x = tile_transform.translation.x - 800.
    
            }
        }
    }
    
    
    
}
