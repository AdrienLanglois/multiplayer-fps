use bevy::{prelude::*, utils::Instant};
use bevy_rapier3d::prelude::{RapierContext, Collider, RigidBody, Velocity, GravityScale};
use crate::{player::*, weapons::{Weapon, FireMode}, hitbox::Hitbox};

#[derive(Component)]
struct ShootSound;

#[derive(Component)]
pub struct Bullet{
    damage: f32,
    direction: Vec3
}

impl Bullet{
    pub fn new(damage: f32, direction: Vec3) -> Self{
        Self{damage, direction}
    }
}

pub const BULLET_VELOCITY:f32 = 200.;

fn can_shoot(input: &PlayerInput, weapon: &Weapon, ammos:u8, is_reloading: bool) -> bool{
    let input_ok = match weapon.fire_mode{
        FireMode::Auto => input.left_click,
        FireMode::SemiAuto => input.left_click_just_pressed,
        FireMode::Burst => input.left_click_just_pressed
    };

    input_ok
    && weapon.last_shot.elapsed() >= weapon.rate_of_fire
    && ammos > 0
    && !is_reloading
}

pub fn shoot(
    mut shooter_q: Query<(&PlayerInput, &mut Weapon, &mut Player,&Transform, &Children), Without<Camera>>,
    camera_q: Query<&GlobalTransform, With<Camera>>,
    mut cmd: Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
    
){
    for (input,mut weapon,mut player, player_transform, children) in shooter_q.iter_mut(){
        if !can_shoot(input, &weapon, player.ammos, player.is_reloading){
            player.is_shooting = false;
            continue;
        }

        // recoil
        if weapon.last_shot.elapsed() > weapon.recoil_reset{
            weapon.consecutive_shots = 0;
        }else{
            weapon.consecutive_shots += 1;
        }

        println!("consec shots :{}", weapon.consecutive_shots);
        
        let mut bullet_direction = Vec3::new(
            weapon.spray_pattern[weapon.consecutive_shots][0],
            weapon.spray_pattern[weapon.consecutive_shots][1],
            0.,
        );

        // update weapon state
        weapon.last_shot = Instant::now();
        player.ammos -= 1;
        player.is_shooting = true;

        // shoot a bullet
        let bullet_origin = Vec3::new(
            player_transform.translation.x,
            2.134, // camera height
            player_transform.translation.z
        );
        
        for &child in children.iter(){
            if let Ok(transform) = camera_q.get(child){
                bullet_direction += transform.forward().normalize();
            }
        }

        cmd.spawn((
            Name::new("Bullet"),
            PbrBundle{
                mesh: meshes.add(shape::Cube{size:0.1}.into()),
                material: materials.add(Color::rgb(1., 1., 0.).into()),
                transform: Transform::from_translation(bullet_origin),
                ..default()
            },
            RigidBody::Dynamic,
            Bullet::new(weapon.damage, bullet_direction),
            Collider::segment(Vec3::ZERO, bullet_direction*10.),
            Velocity{
                linvel:BULLET_VELOCITY * bullet_direction,
                angvel: Vec3::ZERO
            },
            GravityScale(0.)
        ));
    }
}   

pub fn bullet_system(
    rapier_context: Res<RapierContext>,
    bullet_q: Query<(&Bullet, Entity)>,
    player_collider_q: Query<(&Hitbox, &Parent), With<Collider>>,
    mut player_q: Query<&mut Player, With<PlayerId>>,
    mut cmd: Commands
) {
    // check if a bullet hits something
    for (bullet, bullet_entity) in bullet_q.iter(){
        for contact in rapier_context.contacts_with(bullet_entity){

            let hit_collider = if contact.collider1() == bullet_entity {
                contact.collider2()
            } else {
                contact.collider1()
            };
            
            let Ok((damage_multiplier, parent)) = player_collider_q.get(hit_collider) else{
                // if the bullet hits something that is not a player (wall, ground ...), despawn the bullet
                cmd.entity(bullet_entity).despawn();
                continue;          
            };
            
            // player got hit
            let Ok(mut player) = player_q.get_mut(parent.get()) else {
                continue;
            };

            player.hp -= bullet.damage * damage_multiplier.0;
            
            cmd.entity(bullet_entity).despawn()                
        }
    }
}

pub fn reload_system(
    mut player_q: Query<(&mut Player, &Weapon, &PlayerInput)>,
    time: Res<Time>
){

    for (mut player, weapon, player_input) in player_q.iter_mut(){
        // handle reload timer
        if player.reload_timer > 0.{
            player.reload_timer -= time.delta_seconds();
            player.just_reloaded = false; // player is currently in the process of reloading, so he did not just reloaded
        }
        // reloading timer finished
        if player.is_reloading && player.reload_timer <= 0.{
            player.is_reloading = false;
            if player.ammos == 0{
                player.ammos = weapon.ammos;
            }else{                
                player.ammos = weapon.ammos + 1;
            }
        }    
        // player reloads
        if !player.is_reloading && player_input.reload{
            player.is_reloading = true;
            player.just_reloaded = true;
            player.reload_timer = weapon.reload_duration;
        }
    }
}
