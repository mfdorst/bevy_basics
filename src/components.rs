use bevy::math::Vec3;

pub struct Enemy;
pub struct Laser;
pub struct Player;
pub struct PlayerReadyFire(pub bool);
pub struct Speed(pub f32);
pub struct Explosion;
pub struct ExplosionToSpawn(pub Vec3);
