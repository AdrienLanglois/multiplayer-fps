use bevy::prelude::*;

use crate::{world::Maze, player::CurrentPlayer};

//////////////////////////// PLUGIN, COMPONENTS, EVENTS ///////////////////////////////

pub struct MinimapPlugin;
impl Plugin for MinimapPlugin{

    fn build(&self, app: &mut App) {
        app.add_event::<ShowMinimapEvent>();
        app.add_systems(Update,(
            toggle_minimap,
            update_minimap.before(toggle_minimap)
        ));
    }
}

#[derive(Component)]
struct Minimap;

#[derive(Component)]
struct PlayerIndicator;

#[derive(Event)]
struct ShowMinimapEvent;

//////////////////////////// FUNCTIONS, SYSTEMS ///////////////////////////////


fn toggle_minimap(
    key: Res<Input<KeyCode>>,
    minimap_q : Query<(Entity, With<Minimap>)>,
    mut cmd: Commands,
    maze: Res<Maze>,
){
    if !key.just_pressed(KeyCode::Tab){
        return;
    }

    if let Ok(entity) = minimap_q.get_single(){
        println!("despawn minimap");
        cmd.entity(entity.0).despawn();
    }else{
        minimap_event_listener(&maze, &mut cmd)
    }
}

/// not a bevy system
fn minimap_event_listener(
    maze: &Res<Maze>,
    cmd: &mut Commands, 
){
   
    println!("event trigered");

    let width = maze.map[0].len() as f32 * TILE_SIZE;
    let height = maze.map.len() as f32 * TILE_SIZE;

    cmd.spawn((
        Minimap,
        Name::new("minimap"),
        // minimap container
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                position_type: PositionType::Absolute,
                top: Val::Px(10.),
                right: Val::Px(10.),

                display: Display::Flex,
                column_gap: Val::Px(-20.),
                ..default()
            },
            background_color: Color::rgba(0.,0.,0.,0.).into(),
            ..default()
        }
        ))
        .with_children(|parent|{

            let mut x = 0.;
            let mut y = 0.;

            for line in maze.map.iter(){
                for tile in line.iter(){
                    let tile_bundle = match tile{
                        0 => print_tile(x,y,TILE_SIZE, Color::rgba(0., 176./255., 60./255., 0.5)), // path
                        1 => print_tile(x,y,TILE_SIZE, Color::rgba(68./255.,102./255.,0.,0.5)), // wall
                        _ => unreachable!()
                    };
                    parent.spawn(tile_bundle);
                    x += TILE_SIZE;
                }
                x=0.;
                y+=TILE_SIZE;
            }
        });
}

const TILE_SIZE:f32 = 20.;

fn print_tile(x:f32, y:f32, size:f32, color:Color) -> NodeBundle{
    NodeBundle{
        style: Style{
            position_type: PositionType::Relative,
            width: Val::Px(size),
            height: Val::Px(size),
            left: Val::Px(x),
            top: Val::Px(y),
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

/// when the minimap is displayed on the screen, we need to 
/// update the position of the player while he is moving in the maze
fn update_minimap(
    mut cmd: Commands,
    player_q: Query<&Transform, (With<CurrentPlayer>, Changed<Transform>)>,
    minimap_q: Query<Entity, With<Minimap>>,
    maze: Res<Maze>
){
    if let Ok(minimap_entity) = minimap_q.get_single() {
        if let Ok(player_transform) = player_q.get_single() {
            cmd.entity(minimap_entity).with_children(|parent|{
                parent.spawn(
                    print_tile(
                        player_transform.translation.x * maze.tile_size,
                        player_transform.translation.z * maze.tile_size,
                        TILE_SIZE,
                        Color::WHITE
                    )
                );
            });
       };
    };
}