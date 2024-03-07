use bevy::prelude::*;
use std::time::*;

#[derive(Component)]
pub struct LastShot(pub Instant);

#[derive(Component)]
pub struct Weapon{
    pub damage:f32,
    pub dispersion: f32,
    pub rate_of_fire: Duration,
    pub last_shot: Instant,
    pub ammos: u8,
    pub fire_mode: FireMode,
    pub reload_duration: f32,
    // recoil
    pub recoil_reset: Duration, // the time it takes for the weapon to reset the recoil
    pub consecutive_shots: usize,
    pub spray_pattern: Vec<[f32;2]>
}

#[allow(dead_code)]
pub enum FireMode{
    SemiAuto,
    Auto,
    Burst
}

#[derive(Component)]
pub struct WeaponAsset;

#[allow(dead_code)]
impl Weapon{
    
    pub fn pistol() -> Self{
        Self{
            damage: 150.,
            dispersion: 1.,
            rate_of_fire: Duration::from_millis(500),
            ammos: 12,
            fire_mode: FireMode::SemiAuto,
            reload_duration: 2.,
            recoil_reset: Duration::from_millis(800),
            last_shot: Instant::now(),
            consecutive_shots: 0,
            spray_pattern: simple_spray_pattern()
        }
    }

    pub fn shotgun() -> Self{
        Self{
            damage: 80.,
            dispersion: 10.,
            rate_of_fire: Duration::from_millis(1200),
            ammos: 7,
            fire_mode: FireMode::SemiAuto,
            reload_duration: 2.,
            recoil_reset: Duration::from_millis(800),
            last_shot: Instant::now(),
            consecutive_shots: 0,
            spray_pattern: simple_spray_pattern()
        }
    }

    pub fn rifle() -> Self{
        Self{
            damage: 33.,
            dispersion: 3.,
            rate_of_fire: Duration::from_millis(100),
            ammos: 30,
            fire_mode: FireMode::Auto,
            reload_duration: 2.,
            recoil_reset: Duration::from_millis(800),
            last_shot: Instant::now(),
            consecutive_shots: 0,
            spray_pattern: simple_spray_pattern()
        }
    }
}

// for test
fn simple_spray_pattern() -> Vec<[f32;2]>{
    let mut res = vec![];

    for i in 0..31{
        res.push([0.,i as f32]);
    }

    res
}

