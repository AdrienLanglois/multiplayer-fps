
use bevy::prelude::*;

use crate::player::{Player, PlayerInput, PlayerState, CurrentPlayer};

#[derive(Component)]
pub struct Emitter;


#[derive(Component)]
pub struct FootStepSound(pub u64);
#[derive(Component)]
struct Music;

pub fn sound_player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ambiance music
    commands.spawn((
        Name::new("Ambiance music"),
        AudioBundle {
            source: asset_server.load("sounds/night_ambiance_village.ogg"),
            settings: PlaybackSettings::LOOP
        },
        Music,
    ));

    // reload sound on spawn
    commands.spawn((
        Name::new("Sound Effect"),
        AudioBundle {
            source: asset_server.load("sounds/ak-reload-with-rack.ogg"),
            settings: PlaybackSettings::DESPAWN
        },
    ));
}

pub fn handle_sound_emmission(
    mut player_states_q: Query<(&mut Player, &PlayerInput)>,
){
    for (mut player, input) in player_states_q.iter_mut(){
        player.emmited_sound = match &player{
            p if p.is_shooting => Some("sounds/gunshot.ogg".to_string()),
            p if p.just_reloaded => Some("sounds/ak-reload-with-rack.ogg".to_string()),
            _ => None
        };

        if player.emmited_sound.is_some() {continue;}

        player.is_running = !input.is_walking && 
            (input.left || input.right || input.up || input.down);
    }
}

/// not a bevy system
pub fn play_sound_effect(
    commands: &mut Commands,
    player: &PlayerState,
    assets: &Res<AssetServer>,
    footsteps_q: &Query<(&FootStepSound, Entity)>,
    player_id: &u64,
    player_entity: &Entity
){

    // play sound effect (other than footstep)
    if let Some(audio_path) = &player.state.emmited_sound{

        let sound_bundle = (
            Name::new("Sound Effect"),
            AudioBundle {
                source: assets.load(audio_path),
                settings: PlaybackSettings::DESPAWN
            },
            Emitter
        );

        commands.entity(*player_entity).with_children(|parent|{
            parent.spawn(sound_bundle);
        });
    }

    // make sure that a single player cannot make multiple footstep noise at once
    let mut is_making_footsteps_noise = false;
    for (footstep, entity) in footsteps_q.iter(){
        if footstep.0 != *player_id {continue;}
        // when the player stops running, stop making noise
        if !player.state.is_running{
            commands.entity(entity).despawn();
            break;
        }
        
        is_making_footsteps_noise = true;
        break;
    }

    // footstep noise if running
    if player.state.is_running && !is_making_footsteps_noise{
        let entity = commands.spawn((
            Name::new("Footstep Sound"),
            FootStepSound(*player_id),
            Emitter,
            AudioBundle {
                source: assets.load("sounds/footsteps-forest.ogg"),
                settings: PlaybackSettings::DESPAWN
            },
        )).id();

        commands
            .entity(*player_entity)
            .add_child(entity);
    }
}



// in the volume system, we define the sound propagation very simply : 
// the shortest the distance between you and the emmiter, the louder the sound
// The sound does not propagate after the MAX_SOUND_RANGE value
const MAX_SOUND_RANGE: f32 = 35.;
// below this value, you'll hear the sound at maximum volume
const MIN_SOUND_RANGE: f32 = 5.;

pub fn volume_system(
    audio_q: Query<(&AudioSink, &Parent), With<Emitter>>,
    emitter_q: Query<&Transform, Without<CurrentPlayer>>,
    current_player_q: Query<&Transform, With<CurrentPlayer>>

){
    for (sink, parent) in audio_q.iter(){

        // if the current player is the emitter of the sound, play the sound at maximum volume 
        // otherwise, calculate the distance between the emmiter and the current player 
        // and set the volume appropriatly.
        if current_player_q.get(parent.get()).is_ok(){
            sink.set_volume(1.);
        }else{
            let Ok(emitter) = emitter_q.get(parent.get()) else{
                eprintln!("ERR: emitter of the sound not found");
                continue;
            };
            let Ok(current_player) = current_player_q.get_single() else{
                eprintln!("ERR: current player not found");
                continue;
            };
            let dist = match (current_player.translation - emitter.translation).length(){
                x if x<1. => 1.,
                x => x
            };

            // x is distance, v is the volume
            let volume = match dist{
                x if x < MIN_SOUND_RANGE => 1.,
                x => match 1. - (1./(MAX_SOUND_RANGE))*x {
                    v if v < 0. => 0.,
                    v => v
                }
            };

            sink.set_volume(volume);
        }
    }
}