use bevy::prelude::*;

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub laser: Handle<ColorMaterial>,
    pub enemy: Handle<ColorMaterial>,
}

pub struct ActiveEnemies(pub u32);
