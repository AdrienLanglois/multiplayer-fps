use bevy::prelude::*;

use crate::player::{ CurrentPlayer, Player};

pub struct HudPlugin;

impl Plugin for HudPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_hud)
        .add_systems(Update, update_hud);
    }
}

#[derive(Component)]
struct HPText;
#[derive(Component)]
struct AmmoText;



fn setup_hud(
    assets: Res<AssetServer>,
    mut cmd: Commands,
){

    cmd.spawn((
        ImageBundle {
            image: UiImage {
                texture: assets.load("crosshair.png"),
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(15.),
                height: Val::Px(15.),
                left: Val::Percent(50.),
                top: Val::Percent(50.),
                ..default()
            },
            ..default()
            },
        Name::new("Crosshair")
    ));

    cmd.spawn(
        // bottom box
        NodeBundle{
            style: Style{
                width: Val::Percent(50.),
                height: Val::Percent(10.0),
                justify_content: JustifyContent::SpaceEvenly,
                position_type: PositionType::Absolute,
                left: Val::Percent(25.),
                bottom: Val::Px(10.),
                ..default()
            },
            ..default()
        }
    ).with_children(|container|{
        // health
        container.spawn(NodeBundle::default())
        .with_children(|health_container|{
            health_container.spawn(get_image_bundle("hud/health.png", 80., 80., &assets));
            health_container.spawn((
                HPText,
                get_text_bundle("100".to_string(), 50.)
            ));
        });
        container.spawn(NodeBundle::default())
        .with_children(|health_container|{
            health_container.spawn(get_image_bundle("hud/weapon_icon.png", 80., 80., &assets));
            health_container.spawn((
                AmmoText,
                get_text_bundle("30/30".to_string(), 50.)
            ));
        });
    });
}

fn get_image_bundle(src: &str,width:f32, height:f32, assets:&Res<AssetServer>) -> ImageBundle {
    ImageBundle {
        image: UiImage {
            texture: assets.load(src),
            ..default()
        },
        style:Style{
            width:Val::Px(width),
            height:Val::Px(height),
            ..default()
        },
        ..default()
    }
}

fn get_text_bundle(content: String, font_size:f32) -> TextBundle{
    TextBundle {
        text: Text::from_section(
            content,
            TextStyle{
                font_size:font_size,
                ..default()
            }
        ),
        style: Style{
            align_self: AlignSelf::Center,
            margin: UiRect {
                left: Val::Percent(5.),
                bottom: Val::Percent(15.),
                ..default()
            },
            ..default()
        },
        ..default()        
    }
}

fn update_hud(
    mut hp_text_q: Query<&mut Text, With<HPText>>,
    mut ammo_text_q: Query<&mut Text, (With<AmmoText>, Without<HPText>)>,
    player_q: Query<&Player, With<CurrentPlayer>>,
) {
    let Ok(player) = player_q.get_single() else{return};

    for mut text in hp_text_q.iter_mut(){
        text.sections[0].value = player.hp.to_string();
    }

    for mut text in ammo_text_q.iter_mut(){
        text.sections[0].value = format!("{}/30", player.ammos);
    }
}