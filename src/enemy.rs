use super::components::Enemy;
use super::consts::*;
use super::resources::{ActiveEnemies, Materials};
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::thread_rng;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawn.system()),
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
            .insert(Enemy);

        *active_enemies += 1;
    }
}
