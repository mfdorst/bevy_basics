use bevy::prelude::*;

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub player_laser: Handle<ColorMaterial>,
    pub enemy_laser: Handle<ColorMaterial>,
    pub enemy: Handle<ColorMaterial>,
    pub explosion: Handle<TextureAtlas>,
}

pub struct ActiveEnemies(pub u32);
