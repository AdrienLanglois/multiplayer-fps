use game::toggle_sound;
use hud::HudPlugin;
use network::mods::{ClientPlugin, ServerPlugin};
use player::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use sounds::volume_system;
use world::WorldPlugin;
use std::env;
pub use bevy::prelude::*;


mod player;
mod camera;
mod world;
mod weapons;
mod game;
mod flashlight;
mod shoot;
mod hitbox;
mod animation;
mod hud;
mod sounds;
mod minimap;
mod bullet_tracer;

mod network{
    mod client;
    mod server;
    pub mod mods;
}

////////// USAGE ////////////
// "cargo run" => run the game
// "cargo run server" => host a game
// don't forget the --release flag for optimisation in a real game

fn main() {    
    if is_server_mode(){
        run_server_app();
    }else{
        run_client_app();
    }
    
}

fn is_server_mode() -> bool{
    if let Some(first_arg) = env::args().into_iter().nth(1){
        return first_arg == "server"
    }

    return false
}


fn run_server_app(){
    App::new()

    .add_plugins((
        ServerPlugin,

        WorldPlugin,
        camera::CameraPlugin,
        WorldInspectorPlugin::new(),
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default(),
    ))
    .add_systems(Update, sounds::handle_sound_emmission)

    .run();
}


fn run_client_app(){
    App::new()
    .add_plugins((
        ClientPlugin,
        PlayerPlugin,
        WorldPlugin,
        HudPlugin,
        animation::AnimationPlugin,
        minimap::MinimapPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        WorldInspectorPlugin::new()
    ))
    .add_systems(Startup,(
        sounds::sound_player_setup,
        //minimap::set_minimap,
    ))
    .add_systems(Update, (
        game::toggle_fullscreen,
        volume_system,
        toggle_sound,
    ))
    
    .run();
}


