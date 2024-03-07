use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

#[derive(Component)]
/// contains the damage multiplier
pub struct Hitbox(pub f32);
impl Hitbox{
    pub fn head() -> impl Bundle{
        (
            Name::new("Head Collider"),
            Collider::cuboid(0.05, 0.05, 0.05),
            TransformBundle::from_transform(Transform::from_xyz(0., 0.6, 0.)),
            Hitbox(3.5) // headshot deals x3.5 damage
        )
    }

    pub fn body() -> impl Bundle{
        (
            Name::new("Body Collider"),
            Collider::cylinder(0.115, 0.08),
            TransformBundle::from_transform(Transform::from_xyz(0., 0.425, 0.)),
            Hitbox(1.)
        )
    }

    pub fn legs() -> impl Bundle{
        (
            Name::new("Body Collider"),
            Collider::cylinder(0.155, 0.08),
            TransformBundle::from_transform(Transform::from_xyz(0., 0.155, 0.)),
            Hitbox(0.5)
        )
    }
}
