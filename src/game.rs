//! This file contains a bunch of high-level games functionalities 
//! such as the bevy's GameState (Menu, InGame ...), window managing etc ....
use bevy::{prelude::*, window::{PrimaryWindow, WindowMode}};

use crate::player::{CurrentPlayer, Player};

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState{
    #[default]
    InGame,
    GameMenu,
    MainMenu,
    DeathScreen
}
// show or hide the cursor
pub fn toggle_game_menu(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    key: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
){
    let mut window = windows.single_mut();
    
    // toggle game state (inGame/ GameMenu)
    if key.just_pressed(KeyCode::Escape){
        if *current_state.get() == AppState::InGame{
            window.cursor.visible = true;
            next_state.set(AppState::GameMenu);
        }
        
        if *current_state.get() == AppState::GameMenu{
            window.cursor.visible = false;
            next_state.set(AppState::InGame);
        }
    }
}

pub fn dead_screen(mut cmd: Commands){
    cmd.spawn(TextBundle {
        text: Text::from_section(
            "YOU ARE DEAD",
            TextStyle{
                font_size: 50.,
                ..default()
            }
        ),
        style: Style{
            align_self: AlignSelf::Center,
            margin: UiRect {
                left: Val::Percent(5.),
                bottom: Val::Percent(40.),
                ..default()
            },
            ..default()
        },
        ..default()        
    });
}

pub fn toggle_fullscreen(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<Input<KeyCode>>,

){
    if keys.just_pressed(KeyCode::F11){
        let mut window = windows
            .get_single_mut()
            .expect("cannot find main window in function toggle_fullscreen");

        window.mode = WindowMode::Windowed;
     }
}

/// press 'm' to disable the sound
pub fn toggle_sound(
    player_q: Query<&Player, With<CurrentPlayer>>,
    sounds: Query<&AudioSink>,
){
    let Ok(player) = player_q.get_single() else {return};

    if player.is_muted {
        for sink in sounds.iter(){
            sink.set_volume(0.)
        }
    }
}