use bevy::prelude::*;

pub struct ActiveEnemies(pub u32);

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub player_laser: Handle<ColorMaterial>,
    pub enemy_laser: Handle<ColorMaterial>,
    pub enemy: Handle<ColorMaterial>,
    pub explosion: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct PlayerState {
    pub alive: bool,
    pub last_shot_time: f64,
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.alive = false;
        self.last_shot_time = time;
    }
    pub fn spawn(&mut self) {
        self.alive = true;
        self.last_shot_time = 0.0;
    }
}
