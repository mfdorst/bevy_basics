use super::components::{Laser, Speed};
use super::consts::*;
use bevy::prelude::*;
pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.add_system(laser_movement.system());
    }
}

fn laser_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Speed, &mut Transform, With<Laser>)>,
) {
    for (laser_entity, Speed(speed), mut laser_transform, _) in query.iter_mut() {
        let translation = &mut laser_transform.translation;
        translation.y += speed * TIME_STEP;
        if translation.y > WINDOW_HEIGHT {
            commands.entity(laser_entity).despawn();
        }
    }
}
