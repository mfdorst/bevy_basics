use bevy::math::Vec3;

pub struct Enemy;
pub struct EnemyLaser;
pub struct Explosion;
pub struct ExplosionToSpawn(pub Vec3);
pub struct Player;
pub struct PlayerLaser;
pub struct PlayerReadyFire(pub bool);
pub struct Speed(pub f32);
