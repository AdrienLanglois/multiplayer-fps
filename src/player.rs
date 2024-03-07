use std::time::Instant;

use bevy::{prelude::*, input::mouse::MouseMotion};
use serde::{Serialize, Deserialize};
use bevy_rapier3d::prelude::RapierContext;

use crate::{camera::CameraVerticalMotion, flashlight::toggle_flashlight, shoot::{shoot, bullet_system, reload_system}, weapons::Weapon, bullet_tracer, world::Maze};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PlayerInput>()
        .init_resource::<RapierContext>()

        .add_systems(Update, (
            move_players,
            toggle_flashlight,
            shoot,
            bullet_system,
            bullet_tracer::handle_lifetime,
            reload_system,
        ));
    }
}

///////////// Events, Components, Resources ... ///////////////////

pub const PLAYER_MOVE_SPEED: f32 = 7.0;
pub const PLAYER_SLOW_MOVE_SPEED: f32 = 3.5;
pub const MOUSE_SENSIBILITY:f32 = 0.3;
pub const FLASHLIGHT_INTENSITY:f32 = 4000.;
pub const PLAYER_SCALE:f32 = 3.;

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub toggle_flashlight: bool,
    pub reload: bool,
    pub mouse: Vec2,
    pub left_click_just_pressed: bool,
    pub left_click: bool,
    pub is_walking: bool,
    pub mute: bool,
    pub show_map: bool,
}

// message send by the server to the client for synchronization 
#[derive(Serialize, Deserialize,Clone)]
pub struct PlayerState{
    pub translation:[f32;3],
    pub rotation: [f32;4],
    pub cam_vertical_motion: f32,
    pub look_to: [f32;3],
    pub state: Player
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {
    pub flashlight_on: bool,
    pub hp: f32,
    pub ammos: u8,
    pub is_reloading: bool,
    pub just_reloaded: bool,
    pub reload_timer: f32,
    pub is_shooting: bool,
    pub is_walking: bool,
    pub is_running: bool,
    pub emmited_sound: Option<String>,
    pub is_muted: bool,
    
}

impl Default for Player{
    fn default() -> Self {
        Self {
            flashlight_on: false,
            hp: HP::default().0,
            ammos: Weapon::rifle().ammos ,
            is_reloading: false,
            just_reloaded: false,
            reload_timer: 0.,
            is_shooting: false,
            is_running: false,
            is_walking: false,
            emmited_sound: None,
            is_muted: false,
        }
    }
}

#[derive(Debug, Component)]
pub struct PlayerId {
    pub id: u64,
}

#[derive(Component)]
pub struct CurrentPlayer(pub u64);

#[derive(Component)]
pub struct HP(pub f32);
impl Default for HP{
    fn default() -> Self {
        HP(100.)
    }
}

//////////////////// systems //////////////////////

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_inputs: Res<Input<MouseButton>>,
    mut player_input: ResMut<PlayerInput>
) {
    // keys
    player_input.left = keys.pressed(KeyCode::Q);
    player_input.right = keys.pressed(KeyCode::D);
    player_input.up = keys.pressed(KeyCode::Z);
    player_input.down = keys.pressed(KeyCode::S);
    player_input.reload = keys.pressed(KeyCode::R);
    player_input.is_walking = keys.pressed(KeyCode::ShiftLeft);
    player_input.mute = keys.just_pressed(KeyCode::M);
    if keys.just_pressed(KeyCode::F) {
        player_input.toggle_flashlight = !player_input.toggle_flashlight;
    }
    if keys.just_pressed(KeyCode::Tab){
        player_input.show_map = !player_input.show_map;
    }
    
   
    // mouse motion
    if mouse_motion.is_empty(){
        player_input.mouse = Vec2::ZERO;
    }
    for motion in mouse_motion.iter(){
        player_input.mouse = -motion.delta;
    }
    // mouse click
    player_input.left_click = mouse_inputs.pressed(MouseButton::Left);
    player_input.left_click_just_pressed = mouse_inputs.just_pressed(MouseButton::Left);    
}

pub fn move_players(
    mut player_query: Query<(
        &mut Transform,
        &PlayerInput,
        &mut CameraVerticalMotion,
        &Children),
        Without<Camera>>,
    mut camera_q: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    for (mut transform, input, mut camera_motion,  children) in player_query.iter_mut() {
        // move player (translation)
        let mut direction = Vec3::ZERO;

        if input.left {direction += transform.left()}
        if input.right {direction += transform.right()}
        if input.up {direction += transform.forward()}
        if input.down {direction += transform.back()}

        direction = direction.normalize_or_zero();
        let speed = if input.is_walking {PLAYER_SLOW_MOVE_SPEED} else {PLAYER_MOVE_SPEED};

        transform.translation -= direction * speed * time.delta_seconds();
         
        // move camera horizontally
        // mouse micro-movement detection
        if input.mouse.x.abs() == 1.{
            transform.rotate_y(input.mouse.x * 0.00872664626); // 0.5 degrees
        }else{
            let horizontal_motion = input.mouse.x * MOUSE_SENSIBILITY * time.delta_seconds();        
            transform.rotate_y(horizontal_motion);
        }
    
        // move camera vertically
        let vertical_motion = if input.mouse.y.abs() == 1.{
            -input.mouse.y * 0.00872664626 // 0.5 degrees
        }else{
            -input.mouse.y * MOUSE_SENSIBILITY * time.delta_seconds()
        };

        camera_motion.0 = vertical_motion;
        for &child in children.iter(){
            if let Ok(mut cam_transform) = camera_q.get_mut(child){
                // prevent player from rotating the camera more than 90 degrees 
                // around the z axis (vertical motion)
                // let can_rotate = 
                //     (cam_transform.rotation.z + vertical_motion < 0.5)
                //     && (cam_transform.rotation.z + vertical_motion > -0.5);

                // if can_rotate{break;}

            
                cam_transform.rotate_x(vertical_motion);
                break;
            }
        }
    }
}
        
/// get the spawn position of a new player
/// Players spawn at each corner of the maze
/// first player to join the game spawns top-left, 2nd player spawns top-right etc ...
pub fn get_spawn(players_already_connected:usize, maze: &Res<Maze>) -> (f32,f32){
    match players_already_connected{
        0 => (
            maze.tile_size,
            maze.tile_size
        ),
        1 => (
            (maze.map[0].len() - 2) as f32 * maze.tile_size,
            maze.tile_size
        ),
        2 => (
            maze.tile_size,
            (maze.map.len() - 2) as f32 * maze.tile_size,
        ),
        _ => (
            (maze.map[0].len() - 2) as f32 * maze.tile_size,
            (maze.map.len() - 2) as f32 * maze.tile_size 
        )
    }
}   
