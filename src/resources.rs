use bevy::prelude::*;

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub laser: Handle<ColorMaterial>,
    pub enemy: Handle<ColorMaterial>,
    pub explosion: Handle<TextureAtlas>,
}

pub struct ActiveEnemies(pub u32);
