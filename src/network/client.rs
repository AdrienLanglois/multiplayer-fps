use crate::{player::{PlayerInput, PlayerState, FLASHLIGHT_INTENSITY, PlayerId, HP, PLAYER_SCALE, CurrentPlayer, Player}, camera:: get_camera, flashlight::{get_flashlight_bundle, get_inner_flashlight}, weapons::WeaponAsset, game::AppState, sounds::{FootStepSound, play_sound_effect}, shoot::BULLET_VELOCITY, animation, bullet_tracer};
use super::mods::*;
use std::env;
use bevy_rapier3d::prelude::{GravityScale, Velocity, RigidBody, Collider};
use local_ip_address::local_ip;


pub fn new_renet_client() -> (RenetClient,ClientId, NetcodeClientTransport) {
    let server_ip_addr = match env::args().into_iter().nth(1){
        Some(addr) => addr,
        None => local_ip().unwrap().to_string()
    };

    let Ok(server_addr) = (server_ip_addr.clone() + ":5000").parse() else {
        panic!("ERR: {} is not a valid ip adress", server_ip_addr);
    };
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (client,ClientId(client_id), transport)
}

pub fn client_connections_handler(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    client_id: Res<ClientId>,
    mut lobby: ResMut<Lobby>,
    players_q: Query<&Transform, With<Player>>,
    assets: Res<AssetServer>,
){
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {

        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
    
            // spawn player and insert it in the lobby hashmap
            ServerMessages::PlayerConnected { id } => {
                println!("Player {} connected.", id);

                let player_transform = Transform::from_xyz(0.0, 1.5, 0.0).with_scale(PLAYER_SCALE*Vec3::ONE);

                let mut player = commands.spawn((
                        SceneBundle{
                            scene: assets.load("character/player.glb#Scene0"),
                            transform: player_transform,
                            ..default()
                        },
                        Name::new("Player")
                    ));

                // adding flishlight
                let flashlight = get_flashlight_bundle(id);
                let inner_flashlight = get_inner_flashlight(id);
                player.with_children(|parent| {
                    parent.spawn(flashlight);
                    parent.spawn(inner_flashlight);

                });

                //setting weapon (only for the character controlled by the player)
                let mut weapon_transform = Transform::from_xyz(-0.1, 0.5, 0.2).with_scale(0.1 * Vec3::ONE);
                weapon_transform.rotate_y(3.3);

                if client_id.0 == id {
                    player.insert(CurrentPlayer(id));
                    player.with_children(|parent| {
                        // add the player's camera
                        parent.spawn(get_camera());
                    });
                }
                
                for &player_entity in lobby.players.values(){
                    let Ok(player_transform) = players_q.get(player_entity) else {continue;};
                    
                    player.with_children(|parent|{
                        parent.spawn(SpatialAudioBundle{
                            source: assets.load("sounds/music_test.ogg"),
                            settings: PlaybackSettings::LOOP,
                            spatial: SpatialSettings::new(Transform::from_xyz(0., 0.5, 0.), 1., player_transform.translation)
                        });
                    });
                }


                player.insert(PlayerId{id});
                player.insert(HP::default());
                lobby.players.insert(id, player.id());
            }
    
            // remove player from the lobby hashmap
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
                if let Some(player_entity) = lobby.players.remove(&id) {
                    commands.entity(player_entity).despawn();
                }
            }
        }
    }
}

pub fn client_sync_players(
    mut flashlight_q: Query<(&mut SpotLight, &PlayerId, &mut Transform), Without<Camera>>,
    mut camera_q: Query<&mut Transform, With<Camera>>,  
    player_q: Query<&Children, With<PlayerId>>,
    current_player_q: Query<&CurrentPlayer>,
    footsteps_q: Query<(&FootStepSound, Entity)>,

    assets: Res<AssetServer>,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,

    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    lobby: ResMut<Lobby>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // get every message received in this frame
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let players: HashMap<u64, PlayerState> = bincode::deserialize(&message).unwrap();

        'players_loop: for (player_id, player) in players.iter() {
            
            let player_entity = match lobby.players.get(player_id){
                Some(p) => p,
                None => continue 'players_loop
            };

            // check for dead players
            for current_player in current_player_q.iter(){
                if current_player.0 == *player_id && player.state.hp <= 0.{
                    next_state.set(AppState::DeathScreen);
                }
            }
            
            // update player state
            commands
            .entity(*player_entity)
            .insert(Player::from(player.state.clone()));
                
            play_sound_effect(&mut commands, &player, &assets, &footsteps_q, player_id, player_entity);

            //update player position
            let transform = Transform {
                translation: (player.translation).into(),
                rotation: Quat::from_array(player.rotation),
                scale: PLAYER_SCALE * Vec3::ONE,
            };
            
            commands.entity(*player_entity).insert(transform);

            
            // update flashlight
            // the first flashlight found will be the outer flashlight
            // the second is the inner flashlight
            let mut first_flashlight_found = false;

            for (mut light, playerid, mut light_transform) in flashlight_q.iter_mut(){
                if *player_id != playerid.id {continue}

                light_transform.rotate_x(player.cam_vertical_motion);

                if player.state.flashlight_on{
                    if first_flashlight_found{
                        light.intensity = FLASHLIGHT_INTENSITY;
                        break;
                    }else{
                        light.intensity = FLASHLIGHT_INTENSITY - 2000.;
                    }
                    first_flashlight_found = true;
                }else{
                    light.intensity = 0.;
                }                    
            }

            // update player's camera vertical motion
            if let Ok(children) = player_q.get(*player_entity){
                for &child in children{
                    if let Ok(mut cam_transform) = camera_q.get_mut(child){
                        cam_transform.rotate_x(player.cam_vertical_motion);           
                        break;
                    }
                }
            }

            if player.state.is_shooting{
                bullet_tracer::spawn_bullet_tracer(
                    &mut commands, &mut meshes, &mut materials, &transform, player.look_to.into()
                );
            }
        }
        // bullet tracer
    }
}

pub fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {

    let input_message = bincode::serialize(&*player_input).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

// If any error is found we just panic
pub fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}

