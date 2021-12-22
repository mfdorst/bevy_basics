use super::components::{Enemy, Laser};
use super::consts::*;
use super::resources::{ActiveEnemies, Materials};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::thread_rng;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_system(enemy_spawn.system())
            .add_system(laser_hit_enemy.system());
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
            .insert(Enemy);

        *active_enemies += 1;
    }
}

fn laser_hit_enemy(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform, &Sprite, With<Laser>)>,
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
                active_enemies.0 -= 1;
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
