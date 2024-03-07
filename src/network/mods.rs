pub use bevy::prelude::*;
pub use bevy::prelude::{shape::Plane, *};
use bevy::window::{WindowMode, WindowLevel, WindowTheme};
pub use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
    },
    transport::{NetcodeClientPlugin, NetcodeServerPlugin, client_connected},
    RenetClientPlugin, RenetServerPlugin,
};
pub use bevy_renet::renet::transport::
    {NetcodeClientTransport, NetcodeServerTransport, NetcodeTransportError};

pub use std::time::SystemTime;
pub use std::{collections::HashMap, net::UdpSocket};


pub use serde::{Deserialize, Serialize};

use crate::game::*;
use crate::player::{player_input, PlayerPlugin};

pub use super::{
    client::*,
    server::*,
};

////////////////////// RESOURCES, CONSTS, COMPONENTS ...

pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>,
}

#[derive(Resource)]
pub struct ClientId(pub u64);

///////////////////////// PLUGIN 
pub struct ClientPlugin;

impl Plugin for ClientPlugin{
    fn build(&self, app: &mut App) {
        // set window
        app.add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }));
        
        // init game
        app.init_resource::<Lobby>();
        app.add_state::<AppState>();

        // client initialization
        app.add_plugins(RenetClientPlugin);
        app.add_plugins(NetcodeClientPlugin);
        let (client, client_id,transport) = new_renet_client();
        app.insert_resource(client);
        app.insert_resource(client_id);
        app.insert_resource(transport);

        // client sender/listener systems
        app.add_systems(Update,(
            player_input.run_if(in_state(AppState::InGame)),
            toggle_game_menu,
            panic_on_error_system,
            client_connections_handler,
            client_sync_players,
            client_send_input.run_if(in_state(AppState::InGame)),
            dead_screen.run_if(in_state(AppState::DeathScreen))
        )
        .run_if(client_connected()));
    }
}

pub struct ServerPlugin;


impl Plugin for ServerPlugin{
    fn build(&self, app: &mut App) {
        // init server app
        app.add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                title: "Shadow Showndown Server".to_owned(),
                window_level: WindowLevel::AlwaysOnTop,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            
            ..default()
        }));
        app.init_resource::<Lobby>();

        // server initialization
        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);
        let (server, transport) = new_renet_server();
        app.insert_resource(server);
        app.insert_resource(transport);

        // server listener/sender systems
        app.add_systems(Update,(
            server_receive_events,
            receive_user_input,
            server_sync_players,
        ).run_if(resource_exists::<RenetServer>()),);

        // handle players
        app.add_plugins(PlayerPlugin);
        
        app.add_systems(Update, panic_on_error_system);
    }
}

