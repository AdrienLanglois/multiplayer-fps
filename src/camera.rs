// this camera is only used for the server
// the player's camera is directly set on the player component as a children
// cf client.rs

use std::f32::consts::PI;

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_server_camera);
    }
}

#[derive(Component)]
pub struct CameraVerticalMotion(pub f32);

#[derive(Component)]
struct ServerCamera;

pub fn spawn_server_camera(mut commands: Commands) {
    let camera = (Camera3dBundle {
        transform: Transform{
            translation: Vec3::new(-11.7, 3.6, -2.),
        ..default()
        },
        camera: Camera{
            order:isize::MAX,
            ..default()
        },
        ..default()
        
    },
    ServerCamera,
    Name::new("server Camera"));

    commands.spawn(camera);
}

/// not a bevy system. Get a 3D camera bundle
pub fn get_camera() -> impl Bundle{
    let mut camera_transform = Transform::from_xyz(0.,0.65,0.);
    camera_transform.rotate_y(PI);

    (Camera3dBundle{
        transform: camera_transform,
        camera: Camera{
            order: rand::random(),
            ..default()
        },
        ..default()
    },
    Name::new("Camera"))
}


