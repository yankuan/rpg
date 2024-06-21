use std::os::windows;

use bevy::render::camera;
// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    input::mouse, prelude::*, sprite::SpriteBundle, time::Time
};
use bevy::prelude::*;
use crate::plugins::camera::components::GameCamera;
use crate::plugins::{
    player::{
        components::{
            Player, PlayerDebuffSlowWalk, PlayerDirection, PlayerSize, PlayerVelocity,
            PlayerWalkSpeed, TilemapRoad,
        },
        states::PlayerState,
    },
    tilemap::components::TilemapColliders,
};

pub fn player_movement_input(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mouse:Res<Input<MouseButton>>,
    game_camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut windows: Query<&mut Window>,
    mut query: Query<(&mut PlayerState, &mut PlayerDirection,&mut Transform), With<Player>>,
    
) {
 
    if query.is_empty() {
        println!("cc1");
        return;
    }

    let (mut player_state, mut player_direction,mut trans) = query
        .get_single_mut()
        .expect("0 or more than 1 `Player` found.");

    if *player_state != PlayerState::Idle && *player_state != PlayerState::Walk {
        println!("cc2");
        return;
    }
   
    const ARROW_KEYS: [KeyCode; 4] = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];

    if keyboard.any_pressed(ARROW_KEYS) && *player_state != PlayerState::Walk {
        *player_state = PlayerState::Walk;
    } else if keyboard.any_just_released(ARROW_KEYS) && !keyboard.any_pressed(ARROW_KEYS) {
        *player_state = PlayerState::Idle;
    }

    let mut new_player_direction = *player_direction;
    if keyboard.pressed(KeyCode::Left) {

         new_player_direction = PlayerDirection::Left;
     
    } else if keyboard.pressed(KeyCode::Right) {
    
         new_player_direction = PlayerDirection::Right;
    } else if keyboard.pressed(KeyCode::Down) {
    
        new_player_direction = PlayerDirection::Down;
    }else if keyboard.pressed(KeyCode::Up) {
    
        new_player_direction = PlayerDirection::Up;
    }

    if new_player_direction != *player_direction {
        *player_direction = new_player_direction;
    }


    if mouse.just_pressed(MouseButton::Left) {
        let mut window = windows.single_mut();
        let (camera, camera_transform) = game_camera_query.single();
        if let Some(position) = window.cursor_position() {
            println!("鼠标点击点的坐标：(x={}, y={})", position.x, position.y);
           
            //trans.translation.y = position.x; //214 反 50
            //trans.translation.x = position.y //212 正
            // Calculate a world position based on the cursor's position.
            let Some(point) = camera.viewport_to_world_2d(camera_transform, position) else {
                return;
            };
            println!("世界坐标：(x={}, y={})", point.x, point.y);
            trans.translation.y = point.y; //214 反 50
            trans.translation.x = point.x; //212 正

        }
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200., 200., 10.)),
            ..default()
        })
        .insert(Name::from("Collider")); 

    }
    
}

pub fn player_movement(
    mut player_query: Query<
        (
            &mut Transform,
            &mut PlayerVelocity,
            &PlayerState,
            &PlayerDirection,
            //&PlayerWalkSpeed,
            &PlayerSize,
            //&TilemapRoad
        ),
        With<Player>,
    >,
    //debuff_slow_walk_query: Query<&PlayerDebuffSlowWalk>,
    tilemap_collider_query: Query<&TilemapColliders>,
    delta: Res<Time>,
) {
    let (
        mut player_transform,
        mut player_velocity,
        player_state,
        player_direction,
        //player_walk_speed,
        player_size,
        //road,
    ) = player_query
        .get_single_mut()
        .expect("0 or more than 1 `Player` found.");

    //let debuff_slow_walk = !debuff_slow_walk_query.is_empty();
   
    if *player_state == PlayerState::Walk {
        let move_vector = match player_direction {
            PlayerDirection::Up => Vec2::new(0.0, 1.0),
            PlayerDirection::Down => Vec2::new(0.0, -1.0),
            PlayerDirection::Left => Vec2::new(-1.0, 0.0),
            PlayerDirection::Right => Vec2::new(1.0, 0.0),
            PlayerDirection::UpLeft => Vec2::new(-1.0, 1.0).normalize(),
            PlayerDirection::UpRight => Vec2::new(1.0, 1.0).normalize(),
            PlayerDirection::DownLeft => Vec2::new(-1.0, -1.0).normalize(),
            PlayerDirection::DownRight => Vec2::new(1.0, -1.0).normalize(),
        };


        player_velocity.0 += move_vector * 2000.0 * delta.delta_seconds();
        player_velocity.0 = player_velocity.0.clamp_length_max(64.);




    } else if player_velocity.0 != Vec2::ZERO {
        // Deceleration.
        let old_signum = player_velocity.0.signum();
        player_velocity.0 =
            (player_velocity.0 - 2000.0 * old_signum * delta.delta_seconds()).clamp_length_min(0.0);

        let new_signum = player_velocity.0.signum();
        if old_signum != new_signum {
            player_velocity.0 = Vec2::ZERO;
        }
    }

    if player_velocity.0 != Vec2::ZERO {
        // Get new position on each axis.
        let mut new_position_horizontal = player_transform.translation;
        new_position_horizontal.x += player_velocity.0.x * delta.delta_seconds();
        let mut new_position_vertical = player_transform.translation;
        // new_position_vertical.y += player_velocity.0.y * delta.delta_seconds();
        new_position_vertical.y = 0.;
        // Calculate separate bounding-boxes for each axis-movement.
        let new_player_rect_horizontal =
            Rect::from_center_size(new_position_horizontal.truncate(), player_size.0 / 2.0);
        let new_player_rect_vertical =
            Rect::from_center_size(new_position_vertical.truncate(), player_size.0 / 2.0);


        // Check for collision with `TilemapColliders`.
        
        if let Ok(tilemap_colliders) = tilemap_collider_query.get_single() {
            for tilemap_collider_rect in tilemap_colliders.0.iter() {
                if !tilemap_collider_rect
                    .intersect(new_player_rect_horizontal)
                    .is_empty()
                {
                    player_velocity.0.x = 0.0;

                }

                if !tilemap_collider_rect
                    .intersect(new_player_rect_vertical)
                    .is_empty()
                {
                    player_velocity.0.y = 0.0;
                }
            }
        }
    

        // Moving the player.
        //player_velocity.0.y = 0.;
        player_transform.translation += player_velocity.0.extend(0.0) * delta.delta_seconds();
        //player_transform.translation.y = 168.0; //214 反 50
        //player_transform.translation.x = 162.0; //212 正
        //player_transform.translation.y = 212.0; //214 反 50
        //player_transform.translation.x = 118.0; //214 正 50
        println!("{}",player_transform.translation.x);
        println!("{}",player_transform.translation.y);
    
    }
}

pub fn player_movement_reset(mut query: Query<&mut PlayerState, With<Player>>) {
    if let Ok(mut player_state) = query.get_single_mut() {
        *player_state = PlayerState::Idle;
    }
}
