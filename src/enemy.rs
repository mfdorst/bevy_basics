use super::components::{Enemy, EnemyLaser, ExplosionToSpawn, PlayerLaser, Speed};
use super::consts::*;
use super::resources::{ActiveEnemies, Materials};
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::thread_rng;
use rand::Rng;
use std::f32::consts::PI;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .insert_resource(ActiveEnemies(0))
            .add_system(enemy_spawn.system())
            .add_system(laser_hit_enemy.system())
            .add_system(enemy_move.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(enemy_fire.system()),
            );
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    materials: Res<Materials>,
) {
    let ActiveEnemies(ref mut active_enemies) = *active_enemies;
    if *active_enemies < 1 {
        let mut rng = thread_rng();
        let w_span = WINDOW_WIDTH / 2.0 - 100.0;
        let h_span = WINDOW_HEIGHT / 2.0 - 100.0;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(0.5, 0.5, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Speed(250.0));

        *active_enemies += 1;
    }
}

fn laser_hit_enemy(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform, &Sprite, With<PlayerLaser>)>,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
    mut active_enemies: ResMut<ActiveEnemies>,
) {
    for (laser_entity, laser_transform, laser_sprite, _) in laser_query.iter_mut() {
        for (enemy_entity, enemy_transform, enemy_sprite, _) in enemy_query.iter_mut() {
            let laser_scale = Vec2::from(laser_transform.scale);
            let enemy_scale = Vec2::from(enemy_transform.scale);
            let collision = collide(
                laser_transform.translation,
                laser_sprite.size * laser_scale,
                enemy_transform.translation,
                enemy_sprite.size * enemy_scale,
            );
            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                **active_enemies -= 1;
                commands.entity(laser_entity).despawn();

                commands
                    .spawn()
                    .insert(ExplosionToSpawn(enemy_transform.translation.clone()));

                if **active_enemies == 0 {
                    return;
                }
            }
        }
    }
}

fn enemy_fire(
    mut commands: Commands,
    materials: Res<Materials>,
    query: Query<&Transform, With<Enemy>>,
) {
    for &transform in query.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy_laser.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(PI),
                    ..transform
                },
                ..Default::default()
            })
            .insert(EnemyLaser)
            .insert(Speed(500.0));
    }
}

fn enemy_move(time: Res<Time>, mut query: Query<(&mut Transform, &Speed, With<Enemy>)>) {
    let now = time.seconds_since_startup() as f32;
    for (mut xform, speed, _) in query.iter_mut() {
        let max_distance = **speed * TIME_STEP;
        let x_origin = xform.translation.x;
        let y_origin = xform.translation.y;
        let (x_offset, y_offset) = (0.0, 100.0);
        let (x_radius, y_radius) = (150.0, 100.0);
        let angle = **speed * TIME_STEP * now % 360.0 / PI;
        let x_dest = x_radius * angle.cos() + x_offset;
        let y_dest = y_radius * angle.sin() + y_offset;
        let dx = x_origin - x_dest;
        let dy = y_origin - y_dest;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance == 0.0 {
            0.0
        } else {
            max_distance / distance
        };
        let x = x_origin - dx * distance_ratio;
        let x = if x > 0.0 {
            x.max(x_dest)
        } else {
            x.min(x_dest)
        };
        let y = y_origin - dy * distance_ratio;
        let y = if y > 0.0 {
            y.max(y_dest)
        } else {
            y.min(y_dest)
        };
        xform.translation.x = x;
        xform.translation.y = y;
    }
}
