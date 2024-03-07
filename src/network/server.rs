use crate::{player::{PlayerInput, PlayerId, PlayerState, PLAYER_SCALE, Player, get_spawn}, camera::{get_camera, CameraVerticalMotion}, weapons::Weapon, hitbox::Hitbox, world::Maze};
use bevy_rapier3d::prelude::*;
use std::time::*;
use local_ip_address::local_ip;


use super::mods::*;

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}

// server setup
pub fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ConnectionConfig::default());
    let my_local_ip = local_ip().unwrap().to_string() + ":5000";
    let public_addr = my_local_ip.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addr: public_addr,
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(current_time,server_config, socket).unwrap();

    (server, transport)
}

/// handle clients connection/disconnection
pub fn server_receive_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    assets: Res<AssetServer>,
    maze: Res<Maze>
) {
    for event in server_events.iter() {
        match event {

            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                let spawn_position = get_spawn(lobby.players.len(), &maze);
                let player_transform = 
                    Transform::from_xyz(spawn_position.0, 1.5, spawn_position.1)
                    .with_scale(PLAYER_SCALE*Vec3::ONE);

                // Spawn player
                let player_entity = commands.spawn((
                    Name::new("Player1"),
                    SceneBundle{
                        scene: assets.load("character/player.glb#Scene0"),
                        transform: player_transform,
                        ..default()
                    },
                    PlayerInput::default(),

                    CameraVerticalMotion(0.),
                                        
                    PlayerId { id: *client_id },
                    Weapon::rifle(),
                    Player::default(),

                    RigidBody::Dynamic,
                    Velocity{
                        linvel: 0.1 * Vec3::ONE,
                        ..default()
                    },
                    LockedAxes::ROTATION_LOCKED,
                ))
                // adding colliders and camera
                .with_children(|parent|{
                    parent.spawn(Hitbox::head());
                    parent.spawn(Hitbox::body());
                    parent.spawn(Hitbox::legs());
                    parent.spawn(get_camera());
                })
                .id();

                // use the PlayerConnected event to send the list of the players that
                // are already connected
                for &player_id in lobby.players.keys() {
                    let message = bincode::serialize(&ServerMessages::PlayerConnected { id: player_id }).unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
                }

                lobby.players.insert(*client_id, player_entity);

                // broadcast the new client's id
                let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id }).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }

            // disconnection -> despawn player and remove it from the HashMap
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message = bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id }).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }
}

/// read user input and insert it in the player's entity
pub fn receive_user_input(
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    lobby: ResMut<Lobby>,
){
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            let player_input: PlayerInput = bincode::deserialize(&message)
                .expect("[ERR]: player input deserialization failed");
            
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(player_input);
            }
        }
    }
}

// broadcast game state
pub fn server_sync_players(
    mut server: ResMut<RenetServer>,
    player_q: Query<(&Transform, &PlayerId, &Player, &CameraVerticalMotion, &Children, &Weapon), Without<Camera>>,
    camera_q: Query<&GlobalTransform, With<Camera>>
){
    let mut players: HashMap<u64, PlayerState> = HashMap::new();

    for (transform, player_id, state, camera_rotation, children, weapon) in player_q.iter() {
        for &child in children{
            if let Ok(camera_transform) = camera_q.get(child){

                let mut bullet_direction = camera_transform.forward();
                bullet_direction.x += weapon.spray_pattern[weapon.consecutive_shots][0];
                bullet_direction.y += weapon.spray_pattern[weapon.consecutive_shots][1];


        let player_state = PlayerState{
            translation: transform.translation.into(),
            rotation: transform.rotation.into(),
                    cam_vertical_motion: camera_rotation.0,
                    look_to: camera_transform.forward().into(),
                    state: state.clone(),
        };       

                players.insert(player_id.id, player_state);
                break;
            }
        }
    }

    let sync_message = bincode::serialize(&players).unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, sync_message);
}