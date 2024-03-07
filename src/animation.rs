use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBody, Velocity, GravityScale};
use crate::{player::{Player, CurrentPlayer}, weapons:: WeaponAsset, shoot::BULLET_VELOCITY};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, load_animation)
        .add_systems(Update, (
            run_player_animations,
            run_gun_animation,
            link_animations
        ));
    }   
}

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);
#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

#[allow(warnings)]
fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

#[allow(warnings)]
fn link_animations(
    anim_player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in anim_player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

#[allow(warnings)]
fn load_animation(
    mut cmd: Commands,
    assets: Res<AssetServer>
){
    cmd.insert_resource(Animations(vec![
        assets.load("character/player.glb#Animation0"), // idle
        assets.load("character/player.glb#Animation1"), // die
        assets.load("ak47.glb#Animation0"), // ak-47 idle
        assets.load("ak47.glb#Animation1"), // ak-47 reload
        assets.load("ak47.glb#Animation2") // ak-47 shoot
    ]));
}


#[allow(warnings)]
fn run_player_animations(
    animations: Res<Animations>,
    mut players_q: Query<(&AnimationEntityLink, &Player)>,
    mut animation_q: Query<&mut AnimationPlayer>
){
    for (animation_entity, player) in players_q.iter_mut(){
        if let Ok(mut animation_player) = animation_q.get_mut(animation_entity.0){ 
            animation_player.stop_repeating();
            if player.hp > 0.{
                animation_player.play(animations.0[0].clone_weak());
            }else{
                animation_player.play(animations.0[1].clone_weak());
            }
        }
    }
}

fn run_gun_animation(
    mut cmd: Commands,
    current_player_q: Query<(Entity, &Player), With<CurrentPlayer>>,
    weapon_q : Query<(Entity, &AnimationEntityLink), With<WeaponAsset>>,

    animations: Res<Animations>,
    mut anim_player_q: Query<&mut AnimationPlayer>,
){

    let Ok((player_ent, player)) = current_player_q.get_single() else{
        return;
    };
    let Ok ((weapon_entity, anim_entity)) = weapon_q.get_single() else{
        eprintln!("ERR: cannot found weapon :(");
        return;
    };

    cmd.entity(player_ent).add_child(weapon_entity);


    let mut weapon_transform = Transform::from_xyz(-0.1, 0.52, 0.3).with_scale(0.1 * Vec3::ONE);
    weapon_transform.rotate_y(3.2);

    cmd.entity(weapon_entity).insert(weapon_transform);


    // shooting, reloading, idle animation
    if let Ok(mut anim_player) = anim_player_q.get_mut(anim_entity.0){
        if player.is_shooting{
            anim_player.play(animations.0[4].clone_weak());
        }else if player.is_reloading{
            anim_player.play(animations.0[3].clone_weak());
        }else{
            anim_player.play(animations.0[2].clone_weak());
        }
    }
}

