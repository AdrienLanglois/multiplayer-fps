use std::f32::consts::PI;

use bevy::prelude::*;

use crate::player::{PlayerId, PlayerInput, Player};

#[derive(Component)]
pub struct FlashlightOn(pub bool);


/// not a bevy system. returns the flashlight bundle for the client
pub fn get_flashlight_bundle(id: u64) -> impl Bundle{
    let mut flashlight_transform = Transform::from_xyz(0., 0.65, -0.2);
    flashlight_transform.rotate_y(PI);
    
    // setting the flashlight
    let flashlight_bundle = SpotLightBundle{
        transform: flashlight_transform,                    
        spot_light: SpotLight{
            intensity: 0.,
            range: 100.,
            outer_angle:0.25,
            inner_angle:0.05,
            color: Color::rgb(236./255.,229./255.,220./255.),
            shadows_enabled: true,
            ..default()
        },
        ..default()
    };

    

    (flashlight_bundle ,PlayerId{id}, Name::new("Flash light"))
}

pub fn get_inner_flashlight(id: u64) -> impl Bundle{

    let mut flashlight_transform = Transform::from_xyz(0., 0.65, -0.2);
    flashlight_transform.rotate_y(PI);
    
    let flashlight_bundle = SpotLightBundle{
        transform: flashlight_transform,                    
        spot_light: SpotLight{
            intensity: 0.,
            range: 100.,
            outer_angle:0.10,
            inner_angle:0.05,
            color: Color::rgb(236./255.,229./255.,220./255.),
            shadows_enabled: true,
            ..default()
        },
        ..default()
    };

    (flashlight_bundle ,PlayerId{id}, Name::new("Flash light"))

}

pub fn toggle_flashlight(
    mut query: Query<(& PlayerInput, &mut Player)>,
){
       for (input, mut player) in query.iter_mut(){
            player.flashlight_on = input.toggle_flashlight;

            if input.mute{
                player.is_muted = !player.is_muted;
            }
       }
}