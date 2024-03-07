use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::weapons::WeaponAsset;

pub struct WorldPlugin;

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<Maze>();
        app.add_systems(Startup,( 
            spawn_world,
            spawn_maze,
            set_light
        ));
    }
}

#[derive(Resource)]
pub struct Maze{
    pub map: Vec<Vec<i32>>,
    pub tile_size: f32,
    pub wall_height: f32,
}

impl Default for Maze{
    fn default() -> Self {
        let tile_size = 4.5;
        let wall_height = 4.;
        let map = vec![
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,1,0,1,0,1,0,1,0,1,0,1,1,0,1,0,1,1,0,1,1,1,0,1,1,1,1,0,1,0,1],
            vec![1,0,0,0,1,0,1,0,1,0,1,0,1,0,0,1,0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,1],
            vec![1,0,1,0,1,0,0,0,1,0,0,0,1,1,0,0,0,0,0,1,0,0,0,1,0,0,0,1,0,1,0,1],
            vec![1,0,1,0,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1],
            vec![1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,1,1,1,1,1,1,1,1,0,1,1,1,0,1,0,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1],
            vec![1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,1,1,0,1,1,1,0,1,1,1,1,1,0,1,0,1,1,1,0,1,0,1,1,1,1,1,1,1,0,1],
            vec![1,0,1,0,0,0,0,0,0,1,0,0,0,1,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,1,0,1,1,0,1,0,1,0,1,0,1,0,1,1,1,1,1,0,1,1,1,1,1,1,0,1,1,1,1],
            vec![1,0,1,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,1,0,1,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,1,1,1,1,1,1,0,1],
            vec![1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],        
        ];
        Self{map, tile_size, wall_height}
    }
}



fn set_light(
    mut ambient_light: ResMut<AmbientLight>,
    mut cmd: Commands
){
    // dark ambient light
    ambient_light.brightness = 0.02; // lower this value for darker ambiance
    ambient_light.color = Color::hex("333333").unwrap();

    // main light
    cmd.spawn((
        Name::new("Main Light"),
        PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        }
    ));
}

fn spawn_world(
    mut cmd: Commands,

    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>
){  
    // weapon
    let weapon_bundle = (SceneBundle{
        scene: assets.load("ak47.glb#Scene0"),
        transform: Transform::from_xyz(-10., 1., -10.).with_scale(0.1 * Vec3::ONE),
        ..default()
    },);

    cmd.spawn((
        weapon_bundle,
        WeaponAsset,
        Name::new("weapon test animation")
    ));

    // ground
    // let material_bundle = materials.add(StandardMaterial {
    //     base_color_texture: Some(assets.load("ground_texture.jpg").clone()),
    //     ..default()
    // });

    cmd.spawn((
        Name::new("Ground"),
        PbrBundle{
            mesh: meshes.add(shape::Plane::from_size(500.0).into()),
            material: materials.add(Color::rgb(0., 176./255., 60./255.).into()),
            //material: material_bundle,
            transform: Transform::from_xyz(50.,0.,50.),
            ..default()
        },
        
        RigidBody::Fixed,
        Collider::cuboid(100., 0.2, 100.0),
    ));    

    // cube
    let bullet_tracer_material = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(100., 100., 50.0), // 4. Put something bright in a dark environment to see the effect
        ..default()
    });


    cmd.spawn((
        Name::new("Cube2"),
        PbrBundle{
            mesh: meshes.add(shape::Cube{size:0.25}.into()),
            material: bullet_tracer_material,
            transform: Transform::from_xyz(0., 1.5, -6.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.125, 0.125, 0.125),
    ));

    // generate sky
    // let material_bundle = materials.add(StandardMaterial {
    //     base_color_texture: Some(assets.load("night_sky_texture.avif").clone()),

    //     ..default()
    // });

     cmd.spawn((
        Name::new("Sky"),
        PbrBundle{
            mesh: meshes.add(shape::Box::new(500., 0.5, 500.).into()),

            material: materials.add(Color::rgb(0., 26./255.,51./255.).into()),
            transform: Transform::from_xyz(0., 10., 0.),
            ..default()
        },
    )); 
    
}

fn spawn_maze(
    mut cmd: Commands,
    maze: Res<Maze>,
    assets: Res<AssetServer>,

    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>> 
){
    let tile_size = maze.tile_size;
    let wall_height = maze.wall_height;
    let maze_map = maze.map.clone();

    // wall factory
    let mut create_wall = |x:f32, z:f32| {
        let wall = shape::Box::new(tile_size, wall_height, tile_size);
        let material_bundle = materials.add(StandardMaterial {
            base_color_texture: Some(assets.load("bush_texture.jpg").clone()),
            ..default()
        });


        cmd.spawn((
            Name::new("Wall"),
            PbrBundle{
                mesh: meshes.add(wall.into()),
                //material: materials.add(Color::rgb(68./255.,102./255.,0.).into()),
                material: material_bundle,
                transform: Transform::from_xyz(x, wall_height/2., z),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(tile_size/2., wall_height/2., tile_size/2.),
        ));
    };

    // generate the maze
    let mut coord_x = 0.;
    let mut coord_z = 0.;

    for line in maze_map.iter(){
        for tile in line.iter(){
            if *tile == 1{
                create_wall(coord_x, coord_z);
            }
            coord_x += tile_size;
        }
        coord_x = 0.;
        coord_z += tile_size;
    }

     

}