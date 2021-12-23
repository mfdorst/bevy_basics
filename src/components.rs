use bevy::math::Vec3;
use derive_more::{Deref, DerefMut};

pub struct Enemy;
pub struct EnemyLaser;
pub struct Explosion;
#[derive(Deref, DerefMut)]
pub struct ExplosionToSpawn(pub Vec3);
pub struct Player;
pub struct PlayerLaser;
#[derive(Deref, DerefMut)]
pub struct PlayerReadyFire(pub bool);
#[derive(Deref, DerefMut)]
pub struct Speed(pub f32);
