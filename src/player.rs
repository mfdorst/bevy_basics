use super::components::{
    EnemyLaser, ExplosionToSpawn, Player, PlayerLaser, PlayerReadyFire, Speed,
};
use super::consts::*;
use super::resources::{Materials, PlayerState};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_startup_stage(
                "game_setup_actors",
                SystemStage::single(player_spawn.system()),
            )
            .insert_resource(PlayerState::default())
            .add_system(player_movement.system())
            .add_system(laser_hit_player.system())
            .add_system(player_respawn.system())
            .add_system(player_fire.system());
    }
}

fn player_spawn(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    materials: Res<Materials>,
) {
    let bottom = -WINDOW_HEIGHT / 2.0;
    let material = materials.player.clone();
    let transform = Transform {
        translation: Vec3::new(0.0, bottom + 25.0, 10.0),
        scale: Vec3::new(0.5, 0.5, 1.0),
        ..Default::default()
    };
    player_state.spawn();
    commands
        .spawn_bundle(SpriteBundle {
            material,
            transform,
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(500.0))
        .insert(PlayerReadyFire(true));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
) {
    if let Ok((Speed(speed), mut transform, _)) = query.single_mut() {
        let dir = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };
        transform.translation.x += dir * speed * TIME_STEP;
    }
}

fn laser_hit_player(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    mut laser_query: Query<(Entity, &Transform, &Sprite, With<EnemyLaser>)>,
    mut player_query: Query<(Entity, &Transform, &Sprite, With<Player>)>,
) {
    for (laser_entity, laser_xform, laser_sprite, _) in laser_query.iter_mut() {
        for (player_entity, player_xform, player_sprite, _) in player_query.iter_mut() {
            let collision = collide(
                laser_xform.translation,
                laser_sprite.size * Vec2::from(laser_xform.scale),
                player_xform.translation,
                player_sprite.size * Vec2::from(player_xform.scale),
            );

            if let Some(_) = collision {
                player_state.shot(time.seconds_since_startup());
                commands.entity(laser_entity).despawn();
                commands.entity(player_entity).despawn();
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(player_xform.translation.clone()));
            }
        }
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Transform, &mut PlayerReadyFire, With<Player>)>,
) {
    if let Ok((&player_transform, mut ready_fire, _)) = query.single_mut() {
        let PlayerReadyFire(ref mut ready_fire) = *ready_fire;
        if *ready_fire && keyboard.pressed(KeyCode::Space) {
            let x = player_transform.translation.x;
            let y = player_transform.translation.y;
            let mut spawn_laser = |x_offset| {
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 20.0, 0.0),
                            scale: Vec3::new(0.5, 0.5, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(PlayerLaser)
                    .insert(Speed(500.0));
            };
            let x_offset = 144.0 / 4.0 - 5.0;
            spawn_laser(x_offset);
            spawn_laser(-x_offset);
            *ready_fire = false;
        }
        if keyboard.just_released(KeyCode::Space) {
            *ready_fire = true;
        }
    }
}

fn player_respawn(
    commands: Commands,
    mut player_state: ResMut<PlayerState>,
    materials: Res<Materials>,
    time: Res<Time>,
) {
    if !player_state.alive {
        if time.seconds_since_startup() >= player_state.last_shot_time + PLAYER_RESPAWN_DELAY {
            player_state.spawn();
            player_spawn(commands, player_state, materials);
        }
    }
}
