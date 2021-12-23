use super::components::{EnemyLaser, PlayerLaser, Speed};
use super::consts::*;
use bevy::prelude::*;
pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_system(player_laser_movement.system())
            .add_system(enemy_laser_movement.system());
    }
}

fn player_laser_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Speed, &mut Transform, With<PlayerLaser>)>,
) {
    for (laser_entity, Speed(speed), mut laser_transform, _) in query.iter_mut() {
        let translation = &mut laser_transform.translation;
        translation.y += speed * TIME_STEP;
        if translation.y > WINDOW_HEIGHT / 2.0 {
            commands.entity(laser_entity).despawn();
        }
    }
}

fn enemy_laser_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Speed, &mut Transform, With<EnemyLaser>)>,
) {
    for (laser_entity, Speed(speed), mut laser_transform, _) in query.iter_mut() {
        let translation = &mut laser_transform.translation;
        translation.y -= speed * TIME_STEP;
        if translation.y < -WINDOW_HEIGHT / 2.0 - 50.0 {
            commands.entity(laser_entity).despawn();
        }
    }
}
