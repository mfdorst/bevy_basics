use super::components::{Explosion, ExplosionToSpawn};
use super::Materials;
use bevy::prelude::*;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_system(explosion_to_spawn.system())
            .add_system(animate_explosion.system());
    }
}

fn explosion_to_spawn(
    mut commands: Commands,
    query: Query<(Entity, &ExplosionToSpawn)>,
    materials: Res<Materials>,
) {
    for (entity, explosion_to_spawn) in query.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: materials.explosion.clone(),
                transform: Transform {
                    translation: **explosion_to_spawn,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(Timer::from_seconds(0.05, true));

        // ExplosionToSpawn entity handled - get rid of it so we don't keep spawning explosions
        commands.entity(entity).despawn();
    }
}

fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlasses: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        With<Explosion>,
    )>,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle, _) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlasses.get(texture_atlas_handle).unwrap();
            sprite.index += 1;
            if sprite.index == texture_atlas.textures.len() as u32 {
                commands.entity(entity).despawn();
            }
        }
    }
}
