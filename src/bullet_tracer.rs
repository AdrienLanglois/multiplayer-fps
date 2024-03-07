use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBody, Velocity, GravityScale, Collider};
use crate::shoot::BULLET_VELOCITY;

/// a component that contains the remaining lifetime of the bullet
#[derive(Component)]
pub struct BulletTracer(pub f32);

const BULLET_LIFETIME:f32 = 0.5;


pub fn spawn_bullet_tracer(
    cmd: &mut Commands,
    meshes : &mut ResMut<Assets<Mesh>>,
    materials :&mut ResMut<Assets<StandardMaterial>>,
    player_transform: &Transform,   
    camera_transform: Vec3
){

    let bullet_tracer_material = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(100., 100., 50.0), // 4. Put something bright in a dark environment to see the effect
        ..default()
    });

    let translation = Vec3::new(
        player_transform.translation.x,
        2.,
        player_transform.translation.z       
    );
    let origin = Transform::from_translation(translation - 0.25 * player_transform.right());

    cmd.spawn((
        BulletTracer(BULLET_LIFETIME),
        PbrBundle{
            mesh: meshes.add(shape::Cube{size:0.02}.into()),
            material: bullet_tracer_material,
            transform: origin,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.01, 0.01, 0.01),
        Velocity{
            linvel: BULLET_VELOCITY * camera_transform,
            angvel: Vec3::ZERO
        },
        GravityScale(0.)
    ))
    .with_children(|children| {
        children.spawn((
            BulletTracer(2.*BULLET_LIFETIME),
            PointLightBundle {
            point_light: PointLight {
                intensity: 200.0,
                radius: 0.1,
                color: Color::rgb(1., 204./255., 0.),
                ..default()
            },
            ..default()
        }));
    
    })
    ;
}


/// for performance reasons, we will make the bullet disappear shortly after it has been fired
pub fn handle_lifetime(
    mut bullets_q: Query<(&mut BulletTracer, Entity)>,
    time: Res<Time>,
    mut cmd: Commands
){
    for (mut bullet, entity) in bullets_q.iter_mut(){
        bullet.0 -= time.delta_seconds();
        
        if bullet.0 < 0.{
            cmd.entity(entity).despawn();
        }
    }
}